# Experemental OpenCV Desktop App

This app in not in production mode, it still in dev mode so you have to run the code to get the executable app, and for that you need to some requirements installed in your system. (The app is build for Windows 10/11 OS).

## Requirement:

### 1. Rust and Cargo

- you can follow the instractions https://www.rust-lang.org/tools/install
- you might need to install node.js as will https://nodejs.org/en/download

### 2. Tauri

- you can follow the instractions https://tauri.app or by simply run this command after installing cargo:

```bash
$ cargo install create-tauri-app
```

### 3. Visual Studio 2019 with C++ build tools

- you can install from https://visualstudio.microsoft.com/vs/older-downloads/

### 4. OpenCV

- you need to install OpenCV vesion 4.5.0 or higher from https://opencv.org/releases/

### 5. Environment Variables

- OPENCV_INCLUDE_PATHS which can be set to where "include" folder exists, ".../opencv\build\include".
- OPENCV_LINK_LIBS which can be set to the value "opencv_world460" (the number 460 indecates to the OpenCV version).
- OPENCV_LINK_PATHS which contains the path to the "lib" folder, ".../opencv\build\x64\vc15\lib".

## Run:

you can start the program by running this command from the root directory:

```bash
$ cargo tauri dev
```

for more details for running and debuging, visit https://tauri.app/v1/guides/
