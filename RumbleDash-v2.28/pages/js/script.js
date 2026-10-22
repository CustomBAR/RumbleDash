const ads = [
    {
        href: "https://rumble.bot/",
        imgSrc: "Assets/promotions/Affiliate Banners RumbleBot.png",
    },
    {
        href: "https://nerdordie.com/shop/ref/tinyplayerss/",
        imgSrc: "Assets/promotions/Affiliate Banners NoD.png",
    },
    {
        href: "https://discord.gg/nJywsXTaFY",
        imgSrc: "Assets/promotions/Discord Ad Banner RumbleForGaming.png",
    },
    {
        href: "https://paypal.me/2players1Gamer/",
        imgSrc: "Assets/promotions/Paypal Donation Encouragement Banner.png",
    },
    {
        href: "https://github.com/CustomBAR/RumbleDash",
        imgSrc: "Assets/promotions/Github Repository RumbleDash.png",
    },
    // Add more ads as needed
];

let currentAdIndex = 0;
const adInterval = 30000;
const adLink = document.getElementById("ad-link");
const adImage = document.getElementById("ad-image");

function preloadImage(src, callback) {
    const img = new Image();
    img.src = src;
    img.onload = callback; // Call callback once image is fully loaded
}

function cycleAds() {
    // Fade out the current ad
    adImage.classList.add("hidden");

    setTimeout(() => {
        // Move to the next ad in the array
        currentAdIndex = (currentAdIndex + 1) % ads.length;
        const ad = ads[currentAdIndex];
        adLink.href = ad.href;

        // Preload the next ad image to avoid memory buildup
        preloadImage(ad.imgSrc, () => {
            // Once the next image is loaded, update the src
            adImage.src = ad.imgSrc;

            // Fade in the new ad
            adImage.classList.remove("hidden");
        });
    }, 500); // Delay matches fade-out duration
}

// Start cycling ads
setInterval(cycleAds, adInterval);
window.onload = cycleAds; // Show the first ad on page load
