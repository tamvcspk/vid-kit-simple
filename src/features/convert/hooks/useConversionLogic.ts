import { useState } from 'react';
import { videoService, presetService } from '../../../services';
import { useConversionState } from '../../../hooks/useConversionState';
import useConversionStore from '../../../store/conversion-state';
import { useError } from '../../../hooks';
import { ProcessingOptions } from '../../../types';
import { ErrorCategory } from '../../../utils';

export interface ConversionOptions {
  outputFormat: string;
  resolution: string;
  bitrate: number;
  fps: string;
  use_gpu: boolean;
  outputPath: string;
}

export const useConversionLogic = () => {
  const [isConverting, setIsConverting] = useState<boolean>(false);
  const [showSuccessDialog, setShowSuccessDialog] = useState<boolean>(false);
  const [outputPath, setOutputPath] = useState<string>('');
  const { error, setError } = useError();
  const { conversionState, addTask, markTaskFailed } = useConversionState();

  // Start conversion
  const startConversion = async (options: ConversionOptions, files: any[]) => {
    setIsConverting(true);
    setError(null);

    try {
      // 1. Lấy ID của file đang được chọn từ state
      const selectedFileId = conversionState?.selected_file_id;

      if (!selectedFileId) {
        setError({ message: 'Please select a file first', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 2. Tìm đối tượng file đầy đủ dựa trên ID
      const fileToConvert = files.find(f => f.id === selectedFileId);

      if (!fileToConvert || !fileToConvert.path) {
        setError({ message: 'Selected file path not found. Please try adding the file again.', category: ErrorCategory.Validation, timestamp: new Date() });
        setIsConverting(false);
        return;
      }

      // 3. Lấy đường dẫn file đầu vào thực tế
      const inputFilePath = fileToConvert.path;

      // --- Xử lý đường dẫn đầu ra ---
      // 4. Sử dụng giá trị outputPath từ state
      let finalOutputPath: string = options.outputPath || '';

      // 5. Nếu outputPath chưa có (người dùng chưa chọn hoặc nhập), tạo đường dẫn mặc định
      if (!finalOutputPath) {
        console.log("Output path not set, generating default..."); // Thêm log để debug
        const generatedPath = await videoService.generateOutputPath(inputFilePath, options.outputFormat);
        if (generatedPath) {
          finalOutputPath = generatedPath;
          console.log("Generated default output path:", finalOutputPath); // Thêm log để debug
        }

        // Nếu vẫn không tạo được đường dẫn mặc định -> lỗi
        if (!finalOutputPath) {
          setError({ message: 'Could not generate default output path', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
          return;
        }
        // Cập nhật state của form trên UI
        setOutputPath(finalOutputPath);
      }
      // --- Kết thúc xử lý đường dẫn đầu ra ---

      // 6. Đảm bảo đường dẫn đầu ra có phần mở rộng phù hợp với định dạng đầu ra
      if (!finalOutputPath.toLowerCase().endsWith(`.${options.outputFormat.toLowerCase()}`)) {
        // Nếu đường dẫn không kết thúc bằng phần mở rộng phù hợp, thêm vào
        finalOutputPath = `${finalOutputPath}.${options.outputFormat}`;
        console.log("Added extension to output path:", finalOutputPath);
      }

      // Xây dựng đối tượng options cho backend
      const processingOptions: ProcessingOptions = {
        output_format: options.outputFormat,
        output_path: finalOutputPath,
        resolution: options.resolution !== 'original' ? presetService.resolutionToArray(options.resolution) : undefined,
        bitrate: options.bitrate ? options.bitrate * 1000 : undefined, // Giả sử backend cần bps
        framerate: options.fps !== 'original' ? parseFloat(options.fps) : undefined,
        use_gpu: options.use_gpu,
        gpu_codec: undefined, // Sẽ được set bên dưới nếu use_gpu là true
        cpu_codec: undefined, // Sẽ được set bên dưới nếu use_gpu là false
      };

      // 7. Xác định codec dựa trên lựa chọn GPU/CPU
      if (processingOptions.use_gpu) {
        processingOptions.gpu_codec = await videoService.getGpuCodec(processingOptions.output_format);
        console.log("Using GPU codec:", processingOptions.gpu_codec); // Thêm log để debug
      } else {
        processingOptions.cpu_codec = videoService.getCpuCodec(processingOptions.output_format);
        console.log("Using CPU codec:", processingOptions.cpu_codec); // Thêm log để debug
      }

      // 8. Gọi service để tạo task ở backend với đường dẫn và options chính xác
      console.log("Creating conversion task with input:", inputFilePath, "and options:", processingOptions); // Thêm log để debug
      const taskId = await videoService.createConversionTask(inputFilePath, processingOptions);

      // 9. Xử lý kết quả tạo task và bắt đầu chạy task
      if (taskId) {
        console.log("Task created with ID:", taskId); // Thêm log để debug
        await addTask(taskId);
        const success = await videoService.startConversion(taskId);
        if (success) {
          console.log("Conversion started successfully for task:", taskId); // Thêm log để debug
          // Logic xử lý tiến độ... (giữ nguyên)
          // Sử dụng store trực tiếp để lấy state mới nhất
          const checkProgress = setInterval(() => {
            // Lấy trạng thái chuyển đổi hiện tại trực tiếp từ store
            const currentState = useConversionStore.getState().data;
            if (currentState) {
              // Tìm đúng task nếu có nhiều task? Hoặc giả định chỉ có 1 task chạy?
              // Hiện tại đang dựa vào current_progress chung
              const taskProgress = currentState.current_progress; // Hoặc tìm progress của task cụ thể
              console.log(`Task ${taskId} progress: ${taskProgress}`); // Thêm log để debug
              if (taskProgress >= 100) {
                clearInterval(checkProgress);
                setIsConverting(false);
                setShowSuccessDialog(true);
                setOutputPath(processingOptions.output_path); // Có thể không cần thiết nếu đã set ở trên
                console.log("Conversion complete for task:", taskId); // Thêm log để debug
              }
            }
          }, 1000); // Tăng interval lên 1 giây để giảm log
        } else {
          console.error("Failed to start conversion for task:", taskId); // Thêm log để debug
          markTaskFailed(taskId);
          setError({ message: 'Cannot start conversion process', category: ErrorCategory.Task, timestamp: new Date() });
          setIsConverting(false);
        }
      } else {
        console.error("Failed to create conversion task."); // Thêm log để debug
        setError({ message: 'Cannot create conversion task', category: ErrorCategory.Task, timestamp: new Date() });
        setIsConverting(false);
      }
    } catch (error: any) {
      console.error('Error during conversion process:', error); // Log lỗi chi tiết hơn
      const errorMessage = error.message || 'An unknown error occurred during conversion.';
      setError({ message: errorMessage, category: ErrorCategory.Task, timestamp: new Date() });
      setIsConverting(false);
    }
  };

  // Load video information and update file object
  const loadVideoInfo = async (filePath: string) => {
    try {
      return await videoService.getVideoInfo(filePath);
    } catch (err) {
      console.error('Error loading video information:', err);
      setError({ message: 'Error loading video information', category: ErrorCategory.Task, timestamp: new Date() });
      return null;
    }
  };

  return {
    isConverting,
    showSuccessDialog,
    setShowSuccessDialog,
    outputPath,
    setOutputPath,
    startConversion,
    loadVideoInfo,
    error
  };
};
