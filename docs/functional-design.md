## 🧠 SOFTWARE IDEA

### Suggested Names:
- VidKit Simple, QuickV Edit, GPU Video Tools, VidChop & Convert, Easy Vid Processor

### Goal:
Create a lightweight, easy-to-use desktop application that provides basic functionalities such as video format conversion, video splitting, simple editing, and video sanitization before sharing or reuse. The application leverages GPU (if available) for accelerated processing while still running smoothly on CPU-only machines.

---

## 🎯 TARGET USERS

- Regular users needing quick video processing (e.g., phone, dashcam videos).
- Content creators wanting to preprocess videos before using a main editor.
- Users with low CPU performance but strong dedicated or integrated GPUs.

---

## 🧩 MAIN FEATURES

### 1. Convert Video
- Convert between formats like MP4, MKV, AVI, MOV, WebM, etc.
- Customization options:
  - Resolution (keep original, 720p, 1080p, etc.)
  - Bitrate (low/medium/high)
  - FPS (keep original, 30, 60)
  - Codec (CPU or GPU acceleration: h264_nvenc, hevc_nvenc, etc.)
- **Preset:** Provide default presets and support saving user-defined presets.

### 2. Split Video
- Split videos into multiple parts:
  - By time (e.g., every 5 minutes)
  - By file size
  - By specific timeline markers
- Interface allows previewing split segments.

### 3. Edit Video
- Trim/cut the beginning, end, or middle of videos.
- Merge multiple videos (no re-encoding required if formats match).
- Rotate videos by 90/180/270 degrees.
- Basic timeline interface.

### 4. Sanitize Video
- Remove metadata, EXIF, device information, basic watermarks.
- Standardize videos before sharing or uploading.

### 5. Batch Processing
- Support drag-and-drop for multiple files.
- Apply the same preset and operation (Convert/Split/Sanitize).
- Interface displays batch processing progress (queue).

---

## 🎨 UI / UX DESIGN

### Principles
- **Minimalist, intuitive:** Interface suitable for non-tech-savvy users.
- **Dark UI:** Modern and easy on the eyes.
- **Smooth interactions:** Support drag-and-drop, clear progress bars, GPU status display.

### Main Screen
- Navigation bar with 4 buttons: **Convert**, **Split**, **Edit**, **Sanitize**.
- Each tab has its own layout:
  - **ConvertView:** File input, output format, advanced options, GPU toggle, Preset manager.
  - **SplitView:** Video splitting settings with timeline preview.
  - **EditView:** Trim, Rotate, Merge operations with timeline.
  - **SanitizeView:** Checkboxes (remove metadata, watermark, etc.) and result report.

### Preset Manager
- Allows saving personal presets.
- Displays a list of presets for quick selection and application.
- Saves presets as JSON in the user configuration folder.

---

## 🔁 USER FLOW

1. Select a feature tab (e.g., Convert).
2. Drag and drop the video file to process.
3. Customize settings or select an existing preset.
4. (Optional) Enable/disable GPU acceleration.
5. Click “Convert/Split/Apply” to start processing.
6. Display processing progress and detailed logs.
7. Preview or open the folder containing the output.

---

## 🛠️ RECOMMENDED TECHNOLOGIES (Revised)

### Frontend (UI)
- **Tauri + React (TypeScript, SCSS)**
  - **Tauri:** Modern, lightweight, and secure desktop application framework with excellent system integration.
  - **React with TypeScript:** Component-based UI development, ensuring scalability and maintainability.
  - **SCSS:** Powerful styling with CSS modularization capabilities.
  - **PrimeReact:** Use the PrimeReact UI library to build beautiful, modern, and efficient UI components.

### Backend (Video Processing)
- **Rust**
  - Use Rust to build the backend, ensuring performance and memory safety.
  - **Integration with FFmpeg:** Rust will directly call ffmpeg functions via precompiled static libraries, ensuring video encode/decode processing and GPU acceleration support.
  - Manage video processing workflows, control data streams, and handle multithreading efficiently.

### GPU Integration
- **NVIDIA:** h264_nvenc, hevc_nvenc, scale_cuda
- **Intel QSV:** h264_qsv
- **AMD AMF:** h264_amf
- Detect GPU drivers to automatically configure the appropriate `-hwaccel` parameter when calling FFmpeg via Rust.

### Configuration Storage
- **Custom Presets:** Save personal presets as JSON (e.g., `~/.vidkit/presets/*.json`).
- **General Settings:** Save general settings via JSON files or use Tauri’s configuration APIs.

---

## ✅ OPTIMIZATION & DEPLOYMENT

- **Memory Optimization:** Use efficient buffers and minimize large I/O operations during video processing.
- **Multithreading:** The Rust application will leverage async and multi-threading to process videos in parallel, enhancing performance.
- **Smart Error Handling:** Display user-friendly error messages while saving detailed logs for debugging.
- **Application Packaging:** Use Tauri to create lightweight desktop application packages, releasing as portable EXE for Windows or AppImage for Linux.