# Guide to Building FFmpeg on Windows with MSVC and NVENC

This document provides a step-by-step guide to building FFmpeg as a static library on Windows, using the MSVC compiler and NVIDIA's GPU NVENC support.

## 1. Prepare the Environment

### Software Requirements

- **Visual Studio Build Tools** (version 2022 or equivalent)

  - Required components: MSVC and Windows SDK.

- **MSYS2** ([https://www.msys2.org](https://www.msys2.org))

- **NASM**

Install NASM via MSYS2:

```bash
pacman -S nasm
```

- **LLVM and Clang**
  1. Download LLVM from [https://releases.llvm.org/download.html](https://releases.llvm.org/download.html)
     - Choose the pre-built Windows installer (64-bit)
  2. During installation:
     - Select "Add LLVM to the system PATH"
     - Choose a destination folder (e.g., `C:\Program Files\LLVM`)
  3. After installation, set up environment variable:
     - Open System Properties > Advanced > Environment Variables
     - Under "System variables", click "New"
     - Variable name: `LIBCLANG_PATH`
     - Variable value: `C:\Program Files\LLVM\bin` (or your installation path)
  4. Verify installation by opening Command Prompt and running:
     ```cmd
     clang --version
     ```

## 2. Configure the Build Environment

### Step 1: Launch Developer Command Prompt

Open **x64 Native Tools Command Prompt for Visual Studio** (ensure you are building the 64-bit version).

### Step 2: Add NASM, YASM to PATH

If NASM and YASM are installed via MSYS2, no additional steps are needed. If installed manually, add their paths to PATH:

```cmd
set PATH=%PATH%;C:\path\to\nasm;C:\path\to\yasm
```

### Step 3: Open MSYS2 Shell from Command Prompt

```cmd
C:\msys64\msys2_shell.cmd -use-full-path
```

## 3. Install NV-codec-headers

In the MSYS2 window you just opened:

```bash
cd third_party/nv-codec-headers
make install PREFIX=/usr
```

## 4. Build FFmpeg

### Step 1: Navigate to the FFmpeg Directory

```bash
cd ../../third_party/ffmpeg
```

### Step 2: Run Configure

```bash
./configure --enable-nonfree --disable-shared --toolchain=msvc --enable-cuda-nvcc --enable-libnpp --prefix=../ffmpeg-built
```

If you encounter issues with the CUDA toolkit or have not fully installed CUDA, use a simpler command:

```bash
./configure --enable-nonfree --disable-shared --toolchain=msvc --prefix=../ffmpeg-built
```

### Step 3: Run Make to Compile

```bash
make install -j8
```

Once completed, you will find the `.lib` files in FFmpeg's build directory.

## 5. Finalize and Verify

Run `cargo build` again in the Tauri project directory to verify the results.

