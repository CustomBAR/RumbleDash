const express = require("express");
const axios = require("axios");
const fs = require("fs");
const path = require("path");
const os = require("os");
const app = express();
const PORT = process.env.PORT || 9000;

// Function to get the server's internal IP address
function getServerIP() {
    const networkInterfaces = os.networkInterfaces();
    for (let interfaceKey in networkInterfaces) {
        const networkInterface = networkInterfaces[interfaceKey];
        for (let i = 0; i < networkInterface.length; i++) {
            const address = networkInterface[i];
            if (address.family === "IPv4" && !address.internal) {
                return address.address; // Return the internal IP address
            }
        }
    }
    return "localhost"; // Fallback to localhost if no internal IP is found
}

// Endpoint to return the server's internal IP and port
app.get("/api/server-ip", (req, res) => {
    const internalIP = getServerIP();
    res.json({
        ip: internalIP,
        port: PORT,
    });
});

// Serve static files from the public directory
app.use(express.static("public"));
app.use(express.static("widgets"));
app.use(express.static("pages"));
app.use(express.json()); // Middleware to parse JSON
app.use(express.urlencoded({ extended: true })); // Middleware to parse URL-encoded data

// Load configuration from JSON file
let config = {};
try {
    config = JSON.parse(fs.readFileSync("config.json", "utf8"));
    validateConfig(config); // Ensure the config is valid
} catch (error) {
    console.error("Error loading configuration:", error.message);
}

function validateConfig(config) {
    if (!config.apiUrl || typeof config.apiUrl !== "string") {
        throw new Error("Invalid API URL in configuration");
    }
    // Add more validation checks as needed
}

const CACHE_DURATION = 5000; // Cache duration in milliseconds (5 seconds)
let cachedDashboardData = null;
let lastFetchTime = 0;

app.get("/api/dashboard", async (req, res) => {
    if (!config.apiUrl) {
        return res
            .status(400)
            .send(
                "API URL not set. Please set your Rumble API URL in the dashboard.",
            );
    }

    const currentTime = Date.now();
    // Check if cached data is still valid
    if (cachedDashboardData && currentTime - lastFetchTime < CACHE_DURATION) {
        return res.json(cachedDashboardData);
    }

    try {
        const response = await axios.get(config.apiUrl);
        const data = response.data;

        const followerCount = data.followers?.num_followers || 0;
        const subscriberCount = data.subscribers?.num_subscribers || 0;

        // Cache the new data
        cachedDashboardData = {
            followers: followerCount,
            subscribers: subscriberCount,
            goals: config.goals || {},
        };
        lastFetchTime = currentTime;

        res.json(cachedDashboardData);
    } catch (error) {
        console.error("Error fetching data from Rumble API:", error.message);
        console.error(error.response ? error.response.data : error);
        res.status(500).send("Error fetching data");
    }
});

// Endpoint to get the current configuration
app.get("/api/config", (req, res) => {
    res.json(config);
});

// Endpoint to update the configuration
app.post("/api/update-config", (req, res) => {
    const newConfig = req.body;
    try {
        fs.writeFileSync("config.json", JSON.stringify(newConfig, null, 2));
        config = newConfig; // Update the in-memory config
        res.json({
            success: true,
            message: "Configuration updated successfully!",
        });
    } catch (error) {
        console.error("Error updating configuration:", error.message);
        res.status(500).json({
            success: false,
            message: "Error updating configuration",
        });
    }
});

// Serve the followers goal UI
app.get("/followers", (req, res) => {
    res.sendFile(path.join(__dirname, "widgets", "followers.html"));
});

// Serve the subscribers goal UI
app.get("/subscribers", (req, res) => {
    res.sendFile(path.join(__dirname, "widgets", "subscribers.html"));
});

// Serve the forms page
app.get("/form", (req, res) => {
    res.sendFile(path.join(__dirname, "pages", "form.html"));
});

// Serve the changelog page
app.get("/changelog", (req, res) => {
    res.sendFile(path.join(__dirname, "pages", "changelog.html"));
});

// Serve the changelog page
app.get("/contributors", (req, res) => {
    res.sendFile(path.join(__dirname, "pages", "contributors.html"));
});

// Serve the minimal-followers goal UI

// Start the server
app.listen(PORT, () => {
    console.log(`RumbleDash is running on http://${getServerIP()}:${PORT}`);
});
