## **Overview Architecture**

The application is built using a client-server model, separating the frontend and backend to leverage the strengths of each technology. The frontend is developed with React (TypeScript, SCSS) using the PrimeReact library for UI, while the backend is built with Rust, directly integrating with ffmpeg (using static libraries) for video processing. Tauri acts as the bridge between these two parts, providing a secure and efficient API.

---

## **1. Frontend**

- **Technology:** React with TypeScript and SCSS.
- **UI Library:** PrimeReact – helps build modern, minimalist, and user-friendly UI components.
- **Role:**  
  - **View:** Displays the main interface with functional tabs (Convert, Split, Edit, Sanitize).  
  - **Controller:** Handles user events (e.g., drag-and-drop files, select presets, configure tasks) and calls Tauri APIs to execute tasks.
- **State Management:** Solutions like Context API or Redux can be used if state management logic needs to be expanded.

---

## **2. Backend**

- **Technology:** Rust.
- **Video Processing:**  
  - **VideoProcessor:** Performs tasks like converting, splitting, editing, and sanitizing videos by calling ffmpeg through static libraries.  
  - **GPUDetector:** Checks GPU acceleration capabilities and configures corresponding codecs (e.g., h264_nvenc, hevc_nvenc, scale_cuda for NVIDIA, h264_qsv for Intel, h264_amf for AMD).
- **Preset Management:**  
  - **PresetManager:** Stores and manages user presets in JSON files. This is a simple and suitable choice for small data volumes, allowing users to easily edit or back up configurations.
- **Frontend Communication:** Uses Tauri commands to receive requests and return results, ensuring secure and efficient communication between the frontend and backend.

---

## **3. Project Directory Structure**

```
VidKitSimple/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── video_processor.rs
│   │   ├── preset_manager.rs
│   │   ├── gpu_detector.rs
│   │   └── lib.rs
│   └── Cargo.toml
├── src/
│   ├── App.tsx
│   ├── index.tsx
│   ├── components/
│   │   ├── ConvertView.tsx
│   │   ├── SplitView.tsx
│   │   ├── EditView.tsx
│   │   └── SanitizeView.tsx
│   ├── controllers/
│   │   ├── convertController.ts
│   │   ├── splitController.ts
│   │   ├── editController.ts
│   │   └── sanitizeController.ts
│   ├── assets/
│   │   ├── icons/
│   │   └── styles/
│   └── package.json
├── third_party/
│   └── ffmpeg/
│       ├── include/     // Header files
│       ├── lib/         // Static libraries (.a, .lib)
│       └── bin/         // Optional: Executable tools
├── tauri.conf.json
└── CMakeLists.txt

```

---

## **4. Data Flow and Interaction**

1. **Startup:**
   - Tauri initializes the application, runs the Rust backend, and starts the React frontend.
   - The frontend displays the main interface with functional tabs.

2. **User Interaction:**
   - Users interact with the interface (e.g., drag-and-drop files, select presets, configure tasks) through React components.
   - Frontend controllers handle events and call Tauri commands, passing requests to the backend.

3. **Task Processing:**
   - The Rust backend receives requests and executes video tasks via ffmpeg.
   - GPUDetector checks and configures GPU acceleration if available.
   - PresetManager retrieves configurations from JSON files to apply to tasks.

4. **Result Feedback:**
   - After processing, the backend returns results via Tauri events.
   - The frontend updates the interface, displaying progress and results to the user.

---

## **5. Key Components**

- **Frontend:**
  - **Views (ConvertView, SplitView, EditView, SanitizeView):** Display the interface and handle user events.
  - **Controllers:** Handle business logic and communicate with the backend via Tauri commands.
  
- **Backend:**
  - **VideoProcessor:** Handles video operations (convert, split, edit, sanitize) via ffmpeg.
  - **PresetManager:** Manages user presets in JSON format.
  - **GPUDetector:** Checks GPU acceleration capabilities and configures appropriate codecs.

- **Tauri Bridge:** Acts as the bridge between the frontend and backend, ensuring secure and efficient communication.

---

## **6. GPU Integration**

- The Rust backend uses GPU codecs (such as h264_nvenc, hevc_nvenc, scale_cuda for NVIDIA; h264_qsv for Intel; h264_amf for AMD) if GPUDetector identifies acceleration capabilities.
- Users can enable/disable GPU acceleration via the frontend interface.

---

## **7. Preset Management**

- **PresetManager** is responsible for storing and managing presets in JSON files, allowing quick configuration and storage without the complexity of databases like SQLite.
- Frontend controllers can retrieve or update presets via Tauri APIs when needed.

---

## **Conclusion**

This architecture leverages Tauri's power to combine a modern frontend (React, TypeScript, SCSS, PrimeReact) with a robust Rust backend. The clear separation between interface and business logic ensures the application is maintainable, scalable, and performant when handling complex video tasks. Using JSON for PresetManager is suitable for small data volumes and the application's simple requirements, while Rust provides safety and optimal performance for ffmpeg-related processing.