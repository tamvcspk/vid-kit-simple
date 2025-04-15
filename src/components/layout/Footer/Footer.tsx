import { useRef } from 'react';
import { Menu } from 'primereact/menu';
import { invoke } from '@tauri-apps/api/core';

// Import styled components
import { FooterContainer, GpuStatus, GpuSelectorButton } from './Footer.styles';

// Import debug component
import { StateDebugger } from '../../debug/StateDebugger';

// Import hooks and types
import { useAppState } from '../../../hooks/useAppState';
import { GpuInfo } from '../../../types/state.types';

export function Footer() {
  const { appState } = useAppState();
  const menuRef = useRef<Menu>(null);

  // Lấy GPU được chọn từ global state
  const selectedGpuIndex = appState?.selected_gpu_index ?? -1;
  const selectedGpu = selectedGpuIndex >= 0 && appState?.gpus ?
    appState.gpus[selectedGpuIndex] : null;

  // Hàm để cập nhật GPU được chọn
  const updateSelectedGpu = async (index: number) => {
    try {
      await invoke('set_selected_gpu', { gpuIndex: index });
    } catch (error) {
      console.error('Failed to set selected GPU:', error);
    }
  };

  const menuItems = [
    {
      label: 'CPU Only',
      icon: 'pi pi-microchip',
      command: () => updateSelectedGpu(-1),
    },
    ...(appState?.gpus?.map((gpu: GpuInfo, index: number) => ({
      label: `${gpu.name}`,
      icon: gpu.is_available ? 'pi pi-check' : 'pi pi-times',
      command: () => updateSelectedGpu(index),
    })) || []),
  ];

  return (
    <FooterContainer className='app-footer'>
      <GpuStatus>
        <GpuSelectorButton
          icon={selectedGpu?.is_available ? 'pi pi-desktop' : 'pi pi-microchip'}
          severity={selectedGpu?.is_available ? 'success' : 'info'}
          onClick={e => menuRef.current?.toggle(e)}
          aria-label="Select GPU"
          label={selectedGpu ? `${selectedGpu.vendor}` : 'CPU Only'}
          tooltip={selectedGpu ? `${selectedGpu.name}` : 'CPU Only'}
          tooltipOptions={{ position: 'top' }}
        />
        <Menu ref={menuRef} model={menuItems} popup />
      </GpuStatus>

      {/* Debug component */}
      <StateDebugger />
    </FooterContainer>
  );
}
