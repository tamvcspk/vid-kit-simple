## **Overview Architecture**

The application is built using a client-server model, separating the frontend and backend to leverage the strengths of each technology. The frontend is developed with React (TypeScript, SCSS) using the PrimeReact library for UI, while the backend is built with Rust, directly integrating with ffmpeg (using static libraries) for video processing. Tauri acts as the bridge between these two parts, providing a secure and efficient API.

---

## **1. Frontend**

- **Technology:** React with TypeScript and SCSS.
- **UI Library:** PrimeReact – helps build modern, minimalist, and user-friendly UI components.
- **Role:**
  - **View:** Displays the main interface with functional tabs (Convert, Split, Edit, Sanitize).
  - **Services:** Handles user events (e.g., drag-and-drop files, select presets, configure tasks) and calls Tauri APIs to execute tasks.
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
├── src-tauri/                # Rust backend code
│   ├── src/
│   │   ├── main.rs           # Entry point for Rust backend
│   │   ├── video_processor.rs  # Video processing functionality
│   │   ├── preset_manager.rs   # Preset management
│   │   ├── gpu_detector.rs     # GPU detection and configuration
│   │   └── lib.rs             # Library exports
│   └── Cargo.toml           # Rust dependencies
├── src/                     # Frontend React code
│   ├── assets/               # Static assets (images, icons)
│   ├── components/           # Reusable UI components
│   │   ├── common/           # Common components (Button, Input, etc.)
│   │   ├── layout/           # Layout components
│   │   │   ├── Header.tsx    # Header component
│   │   │   ├── Footer/       # Footer component
│   │   │   └── index.ts      # Component exports
│   │   ├── video/            # Video-related components
│   │   └── index.ts          # Component exports
│   ├── features/             # Main application features
│   │   ├── convert/          # Video conversion feature
│   │   │   ├── components/   # Feature-specific components
│   │   │   ├── hooks/        # Feature-specific hooks
│   │   │   ├── utils/        # Feature-specific utilities
│   │   │   ├── ConvertView.tsx # Main feature component
│   │   │   └── index.ts      # Feature exports
│   │   ├── edit/             # Video editing feature
│   │   ├── split/            # Video splitting feature
│   │   └── sanitize/         # Video sanitizing feature
│   ├── hooks/                # Shared custom hooks
│   │   ├── useTheme.ts       # Theme management hook
│   │   └── useError.ts       # Error handling hook
│   ├── services/             # Backend communication services
│   │   ├── baseService.ts    # Base service with error handling
│   │   ├── videoService.ts   # Video processing service
│   │   ├── presetService.ts  # Preset management service
│   │   └── index.ts          # Service exports
│   ├── store/                # State management
│   │   └── slices/           # Redux slices or reducers
│   ├── styles/               # Shared styles
│   ├── types/                # Type definitions
│   │   ├── video.types.ts    # Video-related types
│   │   ├── preset.types.ts   # Preset-related types
│   │   └── index.ts          # Type exports
│   ├── utils/                # Shared utility functions
│   │   └── errorUtils.ts     # Error handling utilities
│   ├── App.tsx               # Root component
│   ├── index.tsx             # Application entry point
│   └── routes.tsx            # Routing configuration
├── third_party/              # Third-party dependencies
│   └── ffmpeg/               # FFmpeg libraries
│       ├── include/          # Header files
│       ├── lib/              # Static libraries (.a, .lib)
│       └── bin/              # Optional: Executable tools
├── docs/                     # Documentation
│   └── architecture.md       # Architecture documentation
├── tauri.conf.json           # Tauri configuration
└── package.json              # Node.js dependencies
```

---

## **4. Data Flow and Interaction**

1. **Startup:**
   - Tauri initializes the application, runs the Rust backend, and starts the React frontend.
   - The frontend displays the main interface with functional tabs.

2. **User Interaction:**
   - Users interact with the interface (e.g., drag-and-drop files, select presets, configure tasks) through React components.
   - Frontend services handle events and call Tauri commands, passing requests to the backend.

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
  - **Features (convert, edit, split, sanitize):** Contain the main functionality of the application.
  - **Components:** Reusable UI components organized by functionality.
  - **Services:** Handle business logic and communicate with the backend via Tauri commands.

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
- Frontend services can retrieve or update presets via Tauri APIs when needed.

---

## **Conclusion**

This architecture leverages Tauri's power to combine a modern frontend (React, TypeScript, SCSS, PrimeReact) with a robust Rust backend. The clear separation between interface and business logic ensures the application is maintainable, scalable, and performant when handling complex video tasks. Using JSON for PresetManager is suitable for small data volumes and the application's simple requirements, while Rust provides safety and optimal performance for ffmpeg-related processing.

The feature-based directory structure enhances modularity and maintainability, making it easier to extend the application with new features while keeping related code together. This organization also improves team collaboration by allowing developers to work on separate features with minimal conflicts.
