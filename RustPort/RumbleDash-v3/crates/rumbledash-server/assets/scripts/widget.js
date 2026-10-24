/*
 * Shared behaviour for every RumbleDash OBS widget.
 *
 * The widget type is taken from the `data-widget` attribute on <body>, which
 * keeps a single script for the followers and subscriber overlays.
 */
(function () {
    "use strict";

    var REFRESH_MS = 5000;
    /* How long the one-shot "goal reached" flash runs for, in ms. */
    var FLASH_MS = 900;

    var body = document.body;
    var kind = body.getAttribute("data-widget") || "followers";

    var counterEl = document.getElementById("counter");
    var goalEl = document.getElementById("goal");
    var progressEl = document.getElementById("progress");

    var compact =
        (window.RumbleDashNumbers && window.RumbleDashNumbers.compact) ||
        function (value) {
            return String(value);
        };

    var wasFull = false;
    var flashTimer = null;

    function text(node, value) {
        if (node) {
            node.innerText = String(value);
        }
    }

    /*
     * Marks the bar as full so the CSS sheen can start, and restarts the
     * one-shot flash whenever the goal is reached from below. Keeping the
     * transition here means a bar that stays full does not flash every poll.
     */
    function setFull(full) {
        if (!progressEl || full === wasFull) {
            return;
        }

        wasFull = full;
        progressEl.classList.toggle("is-full", full);

        if (!full) {
            progressEl.classList.remove("just-filled");
            return;
        }

        // Removing and re-adding the class restarts the keyframes.
        progressEl.classList.remove("just-filled");
        void progressEl.offsetWidth;
        progressEl.classList.add("just-filled");

        if (flashTimer) {
            window.clearTimeout(flashTimer);
        }
        flashTimer = window.setTimeout(function () {
            progressEl.classList.remove("just-filled");
            flashTimer = null;
        }, FLASH_MS);
    }

    function apply(data) {
        var current = 0;
        var goal = 0;

        if (kind === "subscribers") {
            current = data.subscribers || 0;
            goal = (data.goals && data.goals.subscribers) || 0;
        } else {
            current = data.followers || 0;
            goal = (data.goals && data.goals.followers) || 0;
        }

        text(counterEl, current);
        // The goal is the only number that is shortened: the live counter has
        // to stay exact for the streamer reading it.
        text(goalEl, compact(goal));

        if (progressEl) {
            var percentage = goal > 0 ? (current / goal) * 100 : 0;
            progressEl.style.width = Math.min(percentage, 100) + "%";
            setFull(goal > 0 && current >= goal);
        }
    }

    function refresh() {
        fetch("/api/dashboard", { cache: "no-store" })
            .then(function (response) {
                if (!response.ok) {
                    throw new Error("Error fetching dashboard data: " + response.statusText);
                }
                return response.json();
            })
            .then(apply)
            .catch(function (error) {
                console.error("RumbleDash widget:", error);
            });
    }

    refresh();
    setInterval(refresh, REFRESH_MS);
})();
