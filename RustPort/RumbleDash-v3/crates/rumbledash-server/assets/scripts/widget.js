/*
 * Shared behaviour for every RumbleDash OBS widget.
 *
 * The widget type is taken from the `data-widget` attribute on <body>, which
 * keeps a single script for the followers and subscriber overlays.
 */
(function () {
    "use strict";

    var REFRESH_MS = 5000;

    var body = document.body;
    var kind = body.getAttribute("data-widget") || "followers";

    var counterEl = document.getElementById("counter");
    var goalEl = document.getElementById("goal");
    var progressEl = document.getElementById("progress");

    function text(node, value) {
        if (node) {
            node.innerText = String(value);
        }
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
        text(goalEl, goal);

        if (progressEl) {
            var percentage = goal > 0 ? (current / goal) * 100 : 0;
            progressEl.style.width = Math.min(percentage, 100) + "%";
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
