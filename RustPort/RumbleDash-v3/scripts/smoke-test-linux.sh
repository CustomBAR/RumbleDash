#!/usr/bin/env bash
# Verifies that a RumbleDash build actually runs on Linux.
#
# Usage: bash scripts/smoke-test-linux.sh <rumbledash-binary> [display-number]
#
# The binary is started on a virtual X display, then we wait for the widget
# server to report healthy and for a window titled "RumbleDash v3" to appear.
# Anything short of that counts as a failure, so a build that compiles but
# cannot open a window is caught before it is published.
set -euo pipefail

BINARY="${1:?usage: smoke-test-linux.sh <binary> [display-number]}"
DISPLAY_NUM="${2:-99}"
PORT="${RUMBLE_DASH_PORT:-9123}"
WORKDIR="$(mktemp -d)"
LOG="$WORKDIR/app.log"

cleanup() {
    if [[ -n "${APP_PID:-}" ]] && kill -0 "$APP_PID" 2>/dev/null; then
        kill "$APP_PID" 2>/dev/null || true
        wait "$APP_PID" 2>/dev/null || true
    fi
    if [[ -n "${XVFB_PID:-}" ]] && kill -0 "$XVFB_PID" 2>/dev/null; then
        kill "$XVFB_PID" 2>/dev/null || true
        wait "$XVFB_PID" 2>/dev/null || true
    fi
    rm -rf "$WORKDIR"
}
trap cleanup EXIT

rm -f "/tmp/.X${DISPLAY_NUM}-lock"
Xvfb ":$DISPLAY_NUM" -screen 0 1400x900x24 -nolisten tcp > "$WORKDIR/xvfb.log" 2>&1 &
XVFB_PID=$!
export DISPLAY=":$DISPLAY_NUM"
# llvmpipe gives us a working OpenGL implementation without a GPU.
export LIBGL_ALWAYS_SOFTWARE=1

for _ in $(seq 1 30); do
    if xdpyinfo -display ":$DISPLAY_NUM" >/dev/null 2>&1; then
        break
    fi
    sleep 0.5
done

echo "Starting $BINARY on $DISPLAY"
RUMBLE_DASH_PORT="$PORT" "$BINARY" > "$LOG" 2>&1 &
APP_PID=$!

healthy=0
for _ in $(seq 1 60); do
    if curl -fsS "http://127.0.0.1:${PORT}/api/health" -o "$WORKDIR/health.json"; then
        healthy=1
        break
    fi
    if ! kill -0 "$APP_PID" 2>/dev/null; then
        echo "error: the app exited before its server answered"
        cat "$LOG"
        exit 1
    fi
    sleep 1
done

if [[ "$healthy" -ne 1 ]]; then
    echo "error: /api/health never answered on port $PORT"
    cat "$LOG"
    exit 1
fi

echo "health: $(cat "$WORKDIR/health.json")"

# Give the window a moment to map, then confirm it really exists.
windows=0
for _ in $(seq 1 20); do
    windows=$(xdotool search --name "RumbleDash v3" 2>/dev/null | wc -l | tr -d ' ')
    if [[ "$windows" -gt 0 ]]; then
        break
    fi
    sleep 0.5
done

if [[ "$windows" -eq 0 ]]; then
    echo "error: no window titled 'RumbleDash v3' appeared"
    cat "$LOG"
    exit 1
fi

if ! kill -0 "$APP_PID" 2>/dev/null; then
    echo "error: the app crashed shortly after opening its window"
    cat "$LOG"
    exit 1
fi

echo "window: ok ($windows match(es))"
echo "--- app log ---"
cat "$LOG"
echo "smoke test passed"
