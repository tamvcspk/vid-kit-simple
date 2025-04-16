## **Overview Architecture**

The application is built using a client-server model, separating the frontend and backend to leverage the strengths of each technology. The frontend is developed with React (TypeScript, SCSS) using the PrimeReact library for UI, while the backend is built with Rust, integrating with FFmpeg through the ffmpeg-next crate for video processing. Tauri acts as the bridge between these two parts, providing a secure and efficient API.

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

- **Technology:** Rust with Tauri v2.
- **Core Services:**
  - **VideoProcessor:** Performs tasks like converting, splitting, editing, and sanitizing videos by integrating with FFmpeg through the ffmpeg-next crate.
  - **PresetManager:** Stores and manages user presets in JSON files, allowing users to easily edit or back up configurations.
  - **GPUDetector:** Checks GPU acceleration capabilities and configures corresponding codecs (e.g., h264_nvenc, hevc_nvenc, scale_cuda for NVIDIA, h264_qsv for Intel, h264_amf for AMD).
  - **StateManager:** Manages application state including preferences, conversion tasks, and GPU information.
- **Command Layer:** Exposes Rust functions to the frontend through Tauri commands, providing a clean API for frontend-backend communication.
- **State Management:** Uses Tauri's state management capabilities to maintain application state across command invocations.
- **Event System:** Utilizes Tauri events for asynchronous communication, especially for progress updates and long-running tasks.

---

## **3. Project Directory Structure**

```
VidKitSimple/
├── src-tauri/                # Rust backend code
│   ├── src/
│   │   ├── main.rs           # Entry point for Rust backend
│   │   ├── lib.rs            # Library exports and Tauri setup
│   │   ├── commands/         # Command handlers exposed to frontend
│   │   │   ├── mod.rs        # Exports all commands
│   │   │   └── video.rs      # Video-related commands
│   │   ├── services/         # Core business logic modules
│   │   │   ├── mod.rs        # Exports all services
│   │   │   ├── video_processor/  # Video processing functionality
│   │   │   │   ├── mod.rs    # Exports video processor components
│   │   │   │   ├── encoder.rs # Video encoding logic
│   │   │   │   └── filter.rs  # Video filtering logic
│   │   │   ├── preset_manager/ # Preset management
│   │   │   │   ├── mod.rs    # Exports preset manager components
│   │   │   │   └── config.rs # Preset configuration handling
│   │   │   └── database/     # Database operations (if needed)
│   │   │       ├── mod.rs    # Exports database components
│   │   │       └── models.rs # Data models
│   │   ├── state/           # Application state management
│   │   │   ├── mod.rs       # Exports state components
│   │   │   └── app_state.rs # Application state definitions
│   │   └── utils/           # Utility functions and helpers
│   │       ├── mod.rs       # Exports utility functions
│   │       ├── gpu_detector.rs # GPU detection and configuration
│   │       └── error.rs     # Error handling utilities
│   ├── build.rs            # Build script for FFmpeg integration
│   ├── config.toml         # Configuration for build process
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── icons/              # Application icons
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
├── .cargo/                  # Cargo configuration
│   └── config.toml           # Cargo configuration for FFmpeg paths
├── third_party/              # Third-party dependencies (if needed)
│   └── ffmpeg/               # FFmpeg DLLs for Windows (copied by build script)
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
   - The Rust backend receives requests through Tauri commands and delegates to appropriate service modules.
   - VideoProcessor executes video tasks via FFmpeg integration through the ffmpeg-next crate.
   - GPUDetector checks and configures GPU acceleration if available.
   - PresetManager retrieves configurations from JSON files to apply to tasks.
   - StateManager maintains application state and provides progress updates via events.

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
  - **Commands:** Handle requests from the frontend and delegate to appropriate services.
  - **Services:** Core business logic modules that implement the application's functionality:
    - **VideoProcessor:** Handles video operations (convert, split, edit, sanitize) via FFmpeg.
    - **PresetManager:** Manages user presets in JSON format.
  - **State:** Manages application state and provides access to shared data.
  - **Utils:** Utility functions including GPU detection and error handling.

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

This architecture leverages Tauri's power to combine a modern frontend (React, TypeScript, SCSS, PrimeReact) with a robust Rust backend. The clear separation between interface and business logic ensures the application is maintainable, scalable, and performant when handling complex video tasks. Using JSON for PresetManager is suitable for small data volumes and the application's simple requirements, while Rust provides safety and optimal performance for FFmpeg-related processing.

The modular directory structure enhances maintainability and scalability in both frontend and backend:

- **Frontend:** The feature-based organization keeps related code together, making it easier to extend the application with new features while minimizing conflicts during team collaboration.

- **Backend:** The layered architecture with clear separation between commands, services, and utilities promotes code reuse and maintainability. This organization follows best practices for Rust projects and Tauri applications, making the codebase easier to navigate and extend as the application grows.
