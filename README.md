# Dashboard Tool

![image](https://github.com/user-attachments/assets/6086fb66-6797-4ab3-8a92-b08d5f2aa537)


This project is a dashboard tool designed to help users track their followers and subscribers effectively. It provides real-time updates and visualizations of follower and subscriber activity.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Setting Up Node.js and npm](#setting-up-nodejs-and-npm)
  - [Windows](#windows)
  - [Linux](#linux)
  - [macOS](#macos)
- [Usage](#usage)
- [Accessing the Dashboard](#accessing-the-dashboard)
- [Contributing](#contributing)
- [License](#license)

## Features

- Real-time tracking of followers and subscribers.
- Visual representation through charts.
- Progress bars for goals.
- PayPal donation support to help fund the project.

## Requirements

To run this project, you will need:

- Node.js (v14 or higher)
- npm (Node Package Manager)

## Installation

1. **Download the Project**:
   - Visit the project repository page.
   - Click on the "Code" button and select "Download ZIP".
   - Save the ZIP file to your computer.

2. **Extract the ZIP File**:
   - Locate the downloaded ZIP file.
   - Right-click on the file and select "Extract All..." (Windows) or use an extraction tool (Linux/macOS).
   - Choose a destination folder to extract the files.

## Setting Up Node.js and npm

### Windows

1. **Download Node.js Installer**:
   - Go to the official Node.js website: [Node.js Downloads](https://nodejs.org/en/download/).
   - Click on the **Windows Installer** button to download the latest LTS version.

2. **Install Node.js**:
   - Double-click the downloaded `.msi` file to start the installer.
   - Follow the prompts in the setup wizard, accepting the license agreement and choosing default installation options.
   - Click "Install" to complete the installation.

3. **Verify Installation**:
   - Open Command Prompt (search for `cmd`).
   - Run the following commands to verify that Node.js and npm are installed:
     ```bash
     node -v
     npm -v
     ```

### Linux

1. **Install Node.js using a package manager**:
   - Open your terminal.
   - For Debian-based distributions (like Ubuntu), run:
     ```bash
     sudo apt update
     sudo apt install nodejs npm
     ```
   - For Red Hat-based distributions (like CentOS), run:
     ```bash
     sudo yum install nodejs npm
     ```

2. **Verify Installation**:
   - Run the following commands in your terminal:
     ```bash
     node -v
     npm -v
     ```

### macOS

1. **Using Homebrew** (recommended):
   - Open your terminal.
   - If you don’t have Homebrew installed, you can install it with:
     ```bash
     /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
     ```
   - Once Homebrew is installed, run:
     ```bash
     brew install node
     ```

2. **Verify Installation**:
   - Run the following commands in your terminal:
     ```bash
     node -v
     npm -v
     ```

## Usage

1. **Navigate to the Project Directory**:
   ```bash
   cd path/to/extracted-folder

    Install Dependencies:

    bash
    npm install

Start the Server:

bash
node server.js

Open Your Web Browser and navigate to:

text
http://localhost:3000

Accessing the Dashboard
The dashboard can be accessed through your web browser at http://localhost:3000. You will see options to configure your API settings, view follower/subscriber counts, and visualize activity over time.
Widget URL
The Widget URL is displayed on the dashboard, allowing you to integrate it with other tools or services.
Contributing
Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

    Fork the repository.
    Create a new branch (git checkout -b feature/YourFeature).
    Make your changes and commit them (git commit -m 'Add some feature').
    Push to the branch (git push origin feature/YourFeature).
    Open a pull request.
