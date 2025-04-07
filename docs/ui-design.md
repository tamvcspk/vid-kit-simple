## **DETAILED UI DESIGN FOR EACH SCREEN**

### **1. Main Window**
**Objective**: Introduce main functionalities, easily switch between tabs.

- **Layout**:
  - **Navigation Bar (top)**: 4 large buttons, tab-style:
    - `Convert` | `Split` | `Edit` | `Sanitize`
    - Each button has a small icon (e.g., Convert is a circular arrow, Split is scissors) and clear text.
    - The selected tab is highlighted with a bright color (e.g., light blue on a dark background).
  - **Content Area**: Occupies 80% of the screen, changes based on the selected tab.
  - **Status Bar (bottom)**: Displays GPU/CPU information (e.g., "GPU: NVIDIA GTX 1650 - Enabled" or "CPU Only"), along with a toggle button to enable/disable GPU.

- **Colors**:
  - Background: `#1E1E1E` (dark gray)
  - Unselected Tab: `#333333`
  - Selected Tab: `#00A1D6` (light blue)
  - Text: `#FFFFFF` (white)

- **Drag-and-Drop Feature**:
  - Empty area in the middle displays faded text: "Drag & Drop Video Here" when no file is present.

---

### **2. ConvertView**
**Objective**: Convert video formats with intuitive options.

- **Layout**:
  - **Left Column (30%)**:
    - Drag-and-drop area: Large box with dotted border (`#666666`), displays file name when dragged in.
    - File list (for batch processing): Displays as a list, with an "X" button to delete individual files.
  - **Right Column (70%)**:
    - **Output Format**: Dropdown to select format (MP4, MKV, AVI, etc.).
    - **Resolution**: Dropdown (Original, 720p, 1080p, Custom).
    - **Bitrate**: Slider (Low-Medium-High) + manual input box.
    - **FPS**: Dropdown (Original, 30, 60).
    - **Codec**: Toggle between CPU/GPU (e.g., `h264` vs `h264_nvenc`).
    - **Preset**: "Load Preset" button (opens list) and "Save Preset" button (saves current configuration).
    - **Convert Button**: Large blue button (`#00A1D6`), turns into a progress bar when running.

- **UX**:
  - When a file is dragged in, automatically displays original video information (format, size, duration).
  - Hovering over the Bitrate slider shows a tooltip (e.g., "Higher bitrate = better quality, larger file").
  - "Advanced" button (hidden by default) opens additional options like audio codec.

- **States**:
  - Progress bar: `% completed + estimated remaining time`.
  - "Open Folder" button appears upon completion.

---

### **3. SplitView**
**Objective**: Split videos with preview capabilities.

- **Layout**:
  - **Top**: Drag-and-drop area (same as ConvertView).
  - **Middle**: 
    - **Timeline**: Horizontal bar showing video length, with draggable markers for cuts.
    - **Preview**: Small video player (around 300x200px), with Play/Pause button to preview segments.
  - **Bottom**: 
    - **Split Options**: 
      - Radio buttons: "By Time" (input minutes/seconds), "By Size" (input MB), "Manual" (use markers on the timeline).
    - **Output Naming**: Text box to customize output file names (default: "video_part1.mp4", "video_part2.mp4", etc.).
    - **Split Button**: Large blue button.

- **UX**:
  - Clicking a marker on the timeline previews the corresponding segment in the Preview.
  - Hovering over a marker shows a tooltip (e.g., "00:05:00 - 00:10:00").
  - After splitting, displays a list of output files with an "Open Folder" button.

- **States**: 
  - Progress bar for each segment during batch processing.

---

### **4. EditView**
**Objective**: Basic editing with an easy-to-use timeline.

- **Layout**:
  - **Top**: Drag-and-drop area or "Add More Videos" button (for Merge).
  - **Middle**: 
    - **Timeline**: Displays video as a horizontal bar, supports dragging to trim (Trim) or merge (Merge).
    - **Preview**: Larger video player (400x300px), with Play/Pause button.
  - **Bottom**: 
    - **Tools**: 
      - "Trim" button (activates trimming on the timeline).
      - "Rotate" button (dropdown: 90/180/270 degrees).
      - "Merge" button (only active when >1 video is present).
    - **Apply Button**: Processes and saves the result.

- **UX**:
  - Dragging the ends of the timeline trims the start/end.
  - When adding videos for Merge, segments are stacked on the timeline with distinguishable borders.
  - Rotate applies instantly in the Preview for a live preview.

- **States**: 
  - Progress bar during processing, with a short log (e.g., "Rotating... Done").

---

### **5. SanitizeView**
**Objective**: Standardize videos before sharing.

- **Layout**:
  - **Top**: Drag-and-drop area.
  - **Middle**: 
    - Checkbox list:
      - "Remove Metadata" (removes EXIF, device info).
      - "Remove Watermark" (select watermark area with mouse if needed).
      - "Normalize Audio" (advanced option).
    - **Preview Before/After**: Two small side-by-side frames for comparison.
  - **Bottom**: Large blue "Sanitize" button.

- **UX**:
  - Hovering over a checkbox shows a short explanation (e.g., "Remove Metadata: Removes device and recording time info").
  - If "Remove Watermark" is selected, opens an overlay for users to mark the watermark area.
  - After processing, displays a report (e.g., "Removed: Metadata, Watermark").

- **States**: 
  - Progress bar + "Open Folder" button.

---

## **PRESET MANAGER**
**Objective**: Quickly manage and apply presets.

- **Layout** (Popup Window):
  - Preset list: Each preset is a row (name + short description).
  - "Apply" button next to each preset.
  - "New Preset" button (opens form to input name and save current configuration).
  - "Delete" button (removes preset).

- **UX**:
  - Double-clicking a preset applies it immediately.
  - Default presets: "YouTube 1080p", "Mobile MP4", "Fast Convert".

---

## **ADDITIONAL UX/UI DETAILS**
- **Font**: Use modern sans-serif fonts like `Roboto` or `Inter`, size 12-14px for text, 16px for titles.
- **Animation**: 
  - Tab transitions have a light fade effect (0.2s).
  - Progress bars have a moving gradient.
- **Error Handling**: 
  - Small red popup (`#FF5555`) for errors, with a "View Log" button for details.
- **Responsive**: Adjust layout when resizing the window, prioritizing shrinking the right column first.