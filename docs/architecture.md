## **Overview Architecture**

The application is built using a client-server model, separating the frontend and backend to leverage the strengths of each technology. The frontend is developed with React (TypeScript, SCSS) using the PrimeReact library for UI, while the backend is built with Rust, integrating with FFmpeg through the ffmpeg-next crate for video processing. Tauri acts as the bridge between these two parts, providing a secure and efficient API.

---

## **1. Frontend**

- **Technology:** React with TypeScript and SCSS.
- **UI Library:** PrimeReact – helps build modern, minimalist, and user-friendly UI components.
- **Role:**
  - **View:** Displays the main interface with functional tabs (Convert, Split, Edit, Sanitize).
  - **Services:** Handles user events (e.g., drag-and-drop files, select presets, configure tasks) and calls Tauri APIs to execute tasks.
- **State Management:** Uses Zustand for frontend state management, with separate stores for different domains (app state, UI state, conversion state, preferences).

---

## **2. Backend**

- **Technology:** Rust with Tauri v2.
- **Core Services:**
  - **VideoProcessor:** Performs tasks like converting, splitting, editing, and sanitizing videos by integrating with FFmpeg through the ffmpeg-next crate.
  - **StateManager:** Manages application state including tasks, queue, and GPU information.
  - **GPUDetector:** Checks GPU acceleration capabilities and configures corresponding codecs (e.g., h264_nvenc, hevc_nvenc, scale_cuda for NVIDIA, h264_qsv for Intel, h264_amf for AMD).
  - **TaskQueue:** Handles task scheduling, execution, pause/resume, and concurrency control for video processing operations.
- **Command Layer:** Exposes Rust functions to the frontend through Tauri commands, providing a clean API for frontend-backend communication.
- **State Management:** Uses Tauri's state management capabilities with thread-safe wrappers (Arc<Mutex<...>>) to maintain application state across command invocations. Uses tauri-plugin-store for persisting state to JSON files.
- **Event System:** Utilizes Tauri events for asynchronous communication, especially for progress updates, task status changes, and long-running tasks.

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
│   │   │   └── video_processor/  # Video processing functionality
│   │   │       ├── mod.rs    # Exports video processor components
│   │   │       ├── error.rs  # Error handling for video processing
│   │   │       └── processor.rs # Video processing implementation
│   │   ├── state/           # Application state management
│   │   │   ├── mod.rs       # Exports state components
│   │   │   ├── app_state.rs # Application state definitions
│   │   │   ├── conversion_state.rs # File list state
│   │   │   ├── preferences_state.rs # User preferences
│   │   │   └── task_manager/ # Task queue management
│   │   │       ├── mod.rs   # Task manager implementation
│   │   │       ├── processor.rs # Task processor
│   │   │       └── errors.rs # Task error handling
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
│   │   ├── app-state.ts      # App state store
│   │   ├── conversion-state.ts # File list state store
│   │   ├── preferences-state.ts # User preferences store
│   │   ├── presets.store.ts   # Presets management store
│   │   └── tasks.store.ts     # Task management store
│   ├── styles/               # Shared styles
│   ├── types/                # Type definitions
│   │   ├── video.types.ts    # Video-related types
│   │   ├── preset.types.ts   # Preset-related types
│   │   ├── state.types.ts    # State-related types
│   │   ├── store.types.ts    # Store-related types
│   │   ├── task.types.ts     # Task-related types
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

## **4. State Management Architecture**

### 4.1 State Domains

| Domain | Description | Location | Persistence |
|--------|-------------|----------|------------|
| **Global UI State** | • Active tab (`Convert`/`Split`/`Edit`/`Sanitize`)<br>• GPU info (`available`, `enabled`, `name`)<br>• Modal visibility (error alerts)<br>• Theme (light/dark)<br>• Notification state (unread count, notification list) | Frontend (Zustand) | Not persisted |
| **Tab‑Specific UI State** | • **ConvertView**: selected files, output format, resolution, bitrate, FPS, codec<br>• **SplitView**: selected file, mode (time/size/manual), time/size values, markers, naming template<br>• **EditView**: files list, trim ranges, rotate angle, merge order<br>• **SanitizeView**: file, checkbox options (metadata, watermark, audio), watermark region | Frontend (Zustand) | Not persisted |
| **Presets State** | User-defined conversion presets:<br>• `presetId` (UUID)<br>• name, description<br>• output format, resolution, bitrate, FPS, codec<br>• GPU acceleration settings | Frontend (Zustand) | Persisted via tauri-plugin-store |
| **Processing State** | Queue of tasks with full metadata:<br>• `taskId` (UUID)<br>• input paths<br>• type (`Convert`/`Split`/`Edit`/`Sanitize`)<br>• config snapshot<br>• status (`Pending`/`Running`/`Paused`/`Completed`/`Failed`/`Canceled`)<br>• progress `%`<br>• retry attempts & error info<br>• creation/completion time<br>• output path | Backend (Rust) | Persisted via tauri-plugin-store |
| **Configuration State** | • Global settings (output folder, GPU default, `retryLimit`, `maxParallelJobs`) | Frontend (Zustand) | Persisted via tauri-plugin-store |

### 4.2 Task Queue Implementation

1. **Queue Structure**
   - Uses `VecDeque<Task>` for FIFO (First-In-First-Out) ordering of pending tasks
   - Maintains separate collections for running, completed, and failed tasks
   - Implements concurrency control using `tokio::sync::Semaphore`
   - Supports pause/resume using `Arc<(Mutex<bool>, Condvar)>` pattern

2. **Task Lifecycle**
   - Creation → Pending → Running → Completed/Failed/Canceled
   - Support for paused state during Running phase
   - Manual retry option for failed tasks initiated by user action
   - Task progress tracking and event emission

3. **Task Operations**
   - `enqueue_task(task)`: Add task to queue
   - `start_queue()`: Begin processing all tasks
   - `start_task(id)`: Start specific task
   - `pause_task(id)`: Pause running task
   - `resume_task(id)`: Resume paused task
   - `cancel_task(id)`: Cancel pending or running task
   - `retry_task(id)`: Retry a failed task (user-initiated)
   - `reorder_tasks(new_order)`: Reorder pending tasks

### 4.3 Data Flow and Interaction

1. **Startup:**
   - Tauri initializes the application, runs the Rust backend, and starts the React frontend.
   - The frontend displays the main interface with functional tabs.
   - Backend initializes state managers and task queue.
   - Persisted state (presets, tasks, configuration) is loaded from JSON files via tauri-plugin-store.
   - Frontend initializes its Zustand stores with data loaded from persistent storage.

2. **User Interaction:**
   - Users interact with the interface (e.g., drag-and-drop files, select presets, configure tasks) through React components.
   - Frontend services handle events and call Tauri commands, passing requests to the backend.
   - UI state is managed in Zustand stores.

3. **Task Processing:**
   - The Rust backend receives task requests through Tauri commands.
   - Tasks are added to the queue and processed according to FIFO order and concurrency limits.
   - VideoProcessor executes video tasks via FFmpeg integration through the ffmpeg-next crate.
   - TaskQueue manages execution, pause/resume, and retry logic.
   - Progress and status updates are emitted as events to the frontend.

4. **Result Feedback:**
   - Task status changes trigger events from backend to frontend.
   - Frontend updates UI based on task events (progress, completion, failure).
   - Completed tasks show output location and success status.
   - Failed tasks show error information and retry options.

---

## **5. Key Components**

- **Frontend:**
  - **Features (convert, edit, split, sanitize):** Contain the main functionality of the application.
  - **Components:** Reusable UI components organized by functionality.
  - **Services:** Handle business logic and communicate with the backend via Tauri commands.
  - **Stores:** Zustand stores for state management, organized by domain:
    - **app-state.ts:** Global UI state (active tab, GPU info, theme, notifications).
    - **conversion-state.ts:** File list and selected file management.
    - **preferences-state.ts:** User preferences and settings.
    - **presets.store.ts:** User-defined conversion presets.
    - **tasks.store.ts:** Task management interface for the frontend.
  - **Hooks:** Custom hooks for accessing state and backend functionality.

- **Backend:**
  - **Commands:** Handle requests from the frontend and delegate to appropriate services.
  - **Services:** Core business logic modules that implement the application's functionality:
    - **VideoProcessor:** Handles video operations (convert, split, edit, sanitize) via FFmpeg.
  - **State:** Manages backend processing state:
    - **TaskManager:** Task queue and execution management with full processing state (the only state managed in backend).
  - **Utils:** Utility functions including GPU detection and error handling.

- **Tauri Bridge:** Acts as the bridge between the frontend and backend, ensuring secure and efficient communication.

---

## **6. GPU Integration**

- The Rust backend uses GPU codecs (such as h264_nvenc, hevc_nvenc, scale_cuda for NVIDIA; h264_qsv for Intel; h264_amf for AMD) if GPUDetector identifies acceleration capabilities.
- Users can enable/disable GPU acceleration via the frontend interface.

---

## **7. State Persistence**

- **tauri-plugin-store** is used to persist important state to JSON files:
  - **Presets**: User-defined conversion presets are managed entirely on the frontend through presets.store.ts and persisted to JSON.
  - **Tasks**: Task queue and task status are managed by TaskManager in the backend and persisted to allow resuming work after application restart.
  - **Configuration**: Global settings like output folder and concurrency limits are managed on the frontend and persisted.
- State is loaded during application startup and saved whenever it changes (with debouncing to prevent excessive writes).

---

## **8. Task Queue System**

### 8.1 Architecture

- **FIFO Queue**: Uses `VecDeque<Task>` to maintain strict first-come-first-served ordering of tasks.
- **Concurrency Control**: Limits parallel execution using `tokio::sync::Semaphore` with configurable maximum jobs.
- **Asynchronous Processing**: Leverages `tokio::spawn` for non-blocking task execution.
- **State Tracking**: Maintains separate collections for pending, running, and completed tasks.

### 8.2 Task Lifecycle

1. **Creation**: Task is created with a unique UUID and initial configuration.
2. **Enqueuing**: Task is added to the pending queue.
3. **Scheduling**: When resources are available, task is moved to running state.
4. **Execution**: FFmpeg processing is performed with progress tracking.
5. **Completion**: Task is marked as completed, failed, or canceled.
6. **Cleanup**: Completed tasks are retained for a configurable period before removal.

### 8.3 Pause/Resume Mechanism

- **Worker Self-Regulation**: Instead of using system signals (SIGSTOP/SIGCONT), tasks check a shared flag.
- **Implementation**: Uses `Arc<(Mutex<bool>, Condvar)>` pattern:
  - Worker thread periodically checks the pause flag.
  - If paused, worker waits on the condition variable.
  - On resume, condition variable is notified and processing continues.
- **Advantages**: More reliable than signals, works cross-platform, preserves internal state.

### 8.4 Error Handling and Retry

- **User-Initiated Retry**: Failed tasks can be manually retried by user action rather than automatic retry.
- **Retry Counter**: Tracks the number of retry attempts for each task.
- **Error Categorization**: Provides detailed error information to help users decide whether to retry.
- **Detailed Logging**: Records error details, attempt count, and timing information for debugging.
- **Error Persistence**: Preserves error information across application restarts to allow later retry.

### 8.5 Queue Management

- **Dynamic Reordering**: Supports reordering of pending tasks without disrupting running tasks.
- **Batch Operations**: Provides operations for pausing/resuming/canceling multiple tasks.
- **Task Management**: TaskManager handles all task operations and maintains task state.
- **Persistence**: Queue state is saved to JSON files via tauri-plugin-store and restored across application restarts.

---

## **Conclusion**

This architecture leverages Tauri's power to combine a modern frontend (React, TypeScript, SCSS, PrimeReact) with a robust Rust backend. The clear separation between interface and business logic ensures the application is maintainable, scalable, and performant when handling complex video tasks.

Key strengths of this architecture include:

1. **Robust State Management**: The combination of Zustand for frontend state, Tauri's thread-safe state management for backend, and tauri-plugin-store for persistence provides a clean, predictable state flow throughout the application with reliable state recovery after restarts.

2. **Advanced Task Queue System**: The implementation of a FIFO queue with concurrency control, pause/resume capabilities, and user-controlled retry logic ensures reliable video processing with appropriate user oversight.

3. **Clear Domain Separation**: The separation of state into distinct domains (Global UI, Tab-Specific UI, Processing, Configuration) makes the application easier to maintain and extend.

4. **Efficient Resource Utilization**: The task queue's concurrency control and asynchronous processing ensure optimal use of system resources without overwhelming the CPU or memory.

5. **User-Centric Error Handling**: The approach of providing detailed error information and allowing user-initiated retries gives users more control over the processing workflow while maintaining comprehensive logging for troubleshooting.

The modular directory structure enhances maintainability and scalability in both frontend and backend:

- **Frontend:** The feature-based organization with domain-specific state stores keeps related code together, making it easier to extend the application with new features while minimizing conflicts during team collaboration.

- **Backend:** The layered architecture with clear separation between commands, services, state management, and task processing promotes code reuse and maintainability. This organization follows best practices for Rust projects and Tauri applications, making the codebase easier to navigate and extend as the application grows.
