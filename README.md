# Mbox to CSV App

A desktop application built with Tauri (Rust + Webview) to convert `.mbox` email archive files into `.csv` format via a simple drag-and-drop interface.

## Features

*   **Drag and Drop:** Easily convert files by dragging `.mbox` files onto the application window.
*   **Cross-Platform:** Designed to run on both Windows and macOS.

## Tech Stack

*   **Frontend:** HTML, CSS, JavaScript (using Tauri's webview)
*   **Backend:** Rust
*   **Framework:** Tauri

## Building from Source

### Prerequisites

*   [Node.js](https://nodejs.org/) (v18 or later recommended)
*   [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
*   System dependencies for Tauri (see [Tauri prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites))

### Steps

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd mbox-to-csv-app
    ```
2.  **Install Node dependencies:**
    ```bash
    npm install
    ```
3.  **Build the application:**
    ```bash
    npm run tauri build
    ```
    The executable will be located in `src-tauri/target/release/bundle/`.

## Automation

*   This project uses GitHub Actions (`.github/workflows/build.yml`) to automatically build binaries for Windows and macOS upon manual trigger (`workflow_dispatch`). Build artifacts are uploaded for download.

## How it Works

The application uses a simple webview frontend (`src/`) for the user interface. The core MBOX parsing and CSV conversion logic is handled by the Rust backend (`src-tauri/`). Tauri manages the communication between the frontend and the backend.