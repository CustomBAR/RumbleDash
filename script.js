/*
 * RumbleDash – landing page behaviour.
 *
 * Everything here is progressive enhancement: the page is fully readable and
 * usable with JavaScript disabled. The script only adds the motion.
 */
(function () {
    "use strict";

    var reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    var $ = function (sel, ctx) {
        return (ctx || document).querySelector(sel);
    };
    var $$ = function (sel, ctx) {
        return Array.prototype.slice.call(
            (ctx || document).querySelectorAll(sel),
        );
    };

    /* ------------------------------------------------------- scroll state */

    var progress = $(".scroll-progress span");
    var header = $(".site-header");

    function onScroll() {
        var top = window.pageYOffset || document.documentElement.scrollTop;
        var height =
            document.documentElement.scrollHeight - window.innerHeight;

        if (progress) {
            progress.style.width =
                (height > 0 ? Math.min(1, top / height) : 0) * 100 + "%";
        }
        if (header) {
            header.classList.toggle("is-stuck", top > 12);
        }
    }

    var ticking = false;
    window.addEventListener(
        "scroll",
        function () {
            if (ticking) return;
            ticking = true;
            window.requestAnimationFrame(function () {
                onScroll();
                ticking = false;
            });
        },
        { passive: true },
    );
    onScroll();

    /* ------------------------------------------------------- mobile menu */

    var navToggle = $("#nav-toggle");
    var nav = $("#nav");

    if (navToggle && nav) {
        navToggle.addEventListener("click", function () {
            var open = nav.classList.toggle("is-open");
            navToggle.classList.toggle("is-open", open);
            navToggle.setAttribute("aria-expanded", open ? "true" : "false");
            navToggle.setAttribute(
                "aria-label",
                open ? "Close menu" : "Open menu",
            );
        });

        nav.addEventListener("click", function (event) {
            if (event.target.closest("a")) {
                nav.classList.remove("is-open");
                navToggle.classList.remove("is-open");
                navToggle.setAttribute("aria-expanded", "false");
            }
        });
    }

    /* ------------------------------------------------------ kicker rotator */

    var rotator = $("#rotator");
    var phrases = [
        "Live follower counts",
        "Live subscriber counts",
        "Live goal progress",
        "OBS ready widgets",
    ];

    if (rotator && !reduced) {
        var phraseIndex = 0;
        rotator.textContent = phrases[0];

        window.setInterval(function () {
            rotator.classList.add("is-swapping");
            window.setTimeout(function () {
                phraseIndex = (phraseIndex + 1) % phrases.length;
                rotator.textContent = phrases[phraseIndex];
                rotator.classList.remove("is-swapping");
            }, 280);
        }, 3200);
    }

    /* ------------------------------------------------- scroll reveal + count */

    var revealables = $$(".reveal");

    if (reduced || !("IntersectionObserver" in window)) {
        revealables.forEach(function (el) {
            el.classList.add("is-visible");
        });
        $$("[data-count]").forEach(function (el) {
            el.textContent = el.getAttribute("data-count");
        });
    } else {        var observer = new IntersectionObserver(
            function (entries) {
                entries.forEach(function (entry) {
                    if (!entry.isIntersecting) return;

                    var el = entry.target;
                    var siblings = el.parentElement
                        ? $$(".reveal", el.parentElement)
                        : [];
                    var index = siblings.indexOf(el);
                    var delay = index > 0 ? Math.min(index, 6) * 90 : 0;

                    window.setTimeout(function () {
                        el.classList.add("is-visible");
                        countUp(el);
                    }, delay);

                    observer.unobserve(el);
                });
            },
            { threshold: 0.18, rootMargin: "0px 0px -8% 0px" },
        );

        revealables.forEach(function (el) {
            observer.observe(el);
        });
    }

    function countUp(el) {
        var target = parseFloat(el.getAttribute("data-count"));
        if (isNaN(target)) return;

        var duration = 900;
        var start = null;

        el.textContent = "0";

        function step(time) {
            if (start === null) start = time;
            var progress = Math.min((time - start) / duration, 1);
            var eased = 1 - Math.pow(1 - progress, 3);

            el.textContent = String(Math.round(target * eased));

            if (progress < 1) {
                window.requestAnimationFrame(step);
            } else {
                el.textContent = String(target);
            }
        }

        window.requestAnimationFrame(step);
    }

    /* ----------------------------------------------------------- 3D tilt */

    var tilt = $("[data-tilt]");

    if (tilt && !reduced && window.matchMedia("(pointer: fine)").matches) {
        var frame = $(".shot-frame", tilt);

        window.addEventListener(
            "mousemove",
            function (event) {
                var rect = tilt.getBoundingClientRect();
                var x = (event.clientX - rect.left) / rect.width - 0.5;
                var y = (event.clientY - rect.top) / rect.height - 0.5;

                frame.style.transform =
                    "rotateX(" +
                    (-y * 6).toFixed(2) +
                    "deg) rotateY(" +
                    (x * 8).toFixed(2) +
                    "deg) translateZ(0)";
            },
            { passive: true },
        );

        window.addEventListener("mouseout", function () {
            frame.style.transform = "";
        });
    }

    /* ------------------------------------------------------ magnetic buttons */

    if (!reduced && window.matchMedia("(pointer: fine)").matches) {
        $$("[data-magnetic]").forEach(function (button) {
            button.addEventListener("mousemove", function (event) {
                var rect = button.getBoundingClientRect();
                var x = event.clientX - rect.left - rect.width / 2;
                var y = event.clientY - rect.top - rect.height / 2;

                button.style.transform =
                    "translate(" +
                    (x * 0.16).toFixed(1) +
                    "px, " +
                    (y * 0.22).toFixed(1) +
                    "px)";
            });

            button.addEventListener("mouseleave", function () {
                button.style.transform = "";
            });
        });
    }

    /* ----------------------------------------------------- copy to clipboard */

    var toast = $("#toast");
    var toastTimer = null;

    function showToast(message) {
        if (!toast) return;

        toast.textContent = message;
        toast.classList.add("is-visible");

        window.clearTimeout(toastTimer);
        toastTimer = window.setTimeout(function () {
            toast.classList.remove("is-visible");
        }, 2200);
    }

    function copyText(text) {
        if (navigator.clipboard && window.isSecureContext) {
            return navigator.clipboard.writeText(text);
        }

        return new Promise(function (resolve, reject) {
            var area = document.createElement("textarea");
            area.value = text;
            area.setAttribute("readonly", "");
            area.style.position = "fixed";
            area.style.opacity = "0";
            document.body.appendChild(area);
            area.select();

            try {
                document.execCommand("copy") ? resolve() : reject();
            } catch (error) {
                reject(error);
            }

            document.body.removeChild(area);
        });
    }

    $$(".copy-chip").forEach(function (chip) {
        chip.addEventListener("click", function () {
            var text = chip.getAttribute("data-copy");

            copyText(text)
                .then(function () {
                    chip.classList.add("is-copied");
                    $(".copy-label", chip).textContent = "Copied";
                    showToast("Copied " + text);

                    window.setTimeout(function () {
                        chip.classList.remove("is-copied");
                        $(".copy-label", chip).textContent = "Copy";
                    }, 1800);
                })
                .catch(function () {
                    showToast("Press Ctrl+C to copy " + text);
                });
        });
    });

    /* ------------------------------------------------------- active nav link */

    var sections = $$("main section[id]");
    var navLinks = $$('.nav a[href^="#"]');

    if (sections.length && navLinks.length && "IntersectionObserver" in window) {
        var sectionObserver = new IntersectionObserver(
            function (entries) {
                entries.forEach(function (entry) {
                    if (!entry.isIntersecting) return;

                    navLinks.forEach(function (link) {
                        link.classList.toggle(
                            "is-active",
                            link.getAttribute("href") === "#" + entry.target.id,
                        );
                    });
                });
            },
            { rootMargin: "-45% 0px -50% 0px" },
        );

        sections.forEach(function (section) {
            sectionObserver.observe(section);
        });
    }
})();
