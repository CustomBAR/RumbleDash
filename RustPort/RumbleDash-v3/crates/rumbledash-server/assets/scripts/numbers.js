/*
 * Number formatting shared by every RumbleDash overlay and the status page.
 *
 * Kept in a single file so the browser and the desktop app always agree on how
 * a goal is written. The Rust side lives in `crates/rumbledash-app/src/format.rs`.
 */
(function (global) {
    "use strict";

    /**
     * Shortens a goal the way the overlay labels read:
     * 999 -> "999", 1000 -> "1k", 1500 -> "1.5k", 1000000 -> "1M".
     *
     * The value is truncated at one decimal place rather than rounded, so a
     * goal of 1999 reads "1.9k" instead of claiming it has been reached. The
     * decimal is dropped when it is zero, and the unit only changes at exactly
     * 1000, 1000000, and so on, so 999999 stays "999.9k".
     */
    function compact(value) {
        var number = Number(value) || 0;
        if (number < 1000) {
            return String(number);
        }

        var units = [
            [1e12, "T"],
            [1e9, "B"],
            [1e6, "M"],
            [1e3, "k"]
        ];

        for (var i = 0; i < units.length; i++) {
            var divisor = units[i][0];
            if (number >= divisor) {
                var tenths = Math.floor(number / (divisor / 10));
                var whole = Math.floor(tenths / 10);
                var fraction = tenths % 10;
                return fraction === 0
                    ? whole + units[i][1]
                    : whole + "." + fraction + units[i][1];
            }
        }

        return String(number);
    }

    global.RumbleDashNumbers = { compact: compact };
})(window);
