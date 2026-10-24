/* Status page refresh logic for the RumbleDash v3 widget server. */
(function () {
    "use strict";

    var REFRESH_MS = 5000;

    var compact =
        (window.RumbleDashNumbers && window.RumbleDashNumbers.compact) ||
        function (value) {
            return String(value);
        };

    /* Whether each bar is currently marked as full, keyed by element id. */
    var filled = {};

    function set(id, value) {
        var node = document.getElementById(id);
        if (node) {
            node.innerText = String(value);
        }
    }

    function bar(id, current, goal) {
        var node = document.getElementById(id);
        if (!node) {
            return;
        }

        var percentage = goal > 0 ? Math.min((current / goal) * 100, 100) : 0;
        node.style.width = percentage + "%";

        var full = goal > 0 && current >= goal;
        if (filled[id] !== full) {
            filled[id] = full;
            node.classList.toggle("is-full", full);
        }
    }

    function refresh() {
        fetch("/api/dashboard", { cache: "no-store" })
            .then(function (response) {
                if (!response.ok) {
                    throw new Error(response.statusText);
                }
                return response.json();
            })
            .then(function (data) {
                var goals = data.goals || {};
                set("followerCount", data.followers || 0);
                set("subscriberCount", data.subscribers || 0);
                // Goals are shortened the same way as on the overlays.
                set("followerGoal", compact(goals.followers || 0));
                set("subscriberGoal", compact(goals.subscribers || 0));
                bar("followerProgress", data.followers || 0, goals.followers || 0);
                bar("subscriberProgress", data.subscribers || 0, goals.subscribers || 0);
            })
            .catch(function (error) {
                console.error("RumbleDash:", error);
            });
    }

    refresh();
    setInterval(refresh, REFRESH_MS);
})();
