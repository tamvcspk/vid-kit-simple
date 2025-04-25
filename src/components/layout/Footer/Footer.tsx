import { useRef } from 'react';
import { Menu } from 'primereact/menu';

// Import styled components
import { FooterContainer, GpuStatus, GpuSelectorButton } from './Footer.styles';

// Import debug component
import { StateDebugger } from '../../debug/StateDebugger';

// Import notification components
import { NotificationButton } from '../../notifications';
import { NotificationPanel } from '../../notifications';

// Import log buttons
import { LogButtons } from './LogButtons';

// Import hooks and types
import { useAppState } from '../../../hooks/useAppState';

export function Footer() {
  const { appState, setSelectedGpu } = useAppState();
  const menuRef = useRef<Menu>(null);

  // Get selected GPU from global state
  const selectedGpuIndex = appState?.selected_gpu_index ?? -1;
  const selectedGpu = selectedGpuIndex >= 0 && appState?.gpus && appState.gpus.length > 0 ?
    appState.gpus[selectedGpuIndex] : null;

  // Function to update selected GPU
  const updateSelectedGpu = async (index: number) => {
    try {
      await setSelectedGpu(index);
    } catch (error) {
      console.error('Failed to set selected GPU:', error);
    }
  };

  // Create menu items for all GPUs
  const menuItems = [
    {
      label: 'CPU Only',
      icon: 'pi pi-microchip',
      command: () => updateSelectedGpu(-1),
    },
    ...(appState?.gpus?.map((gpu, index: number) => ({
      label: `${gpu.name}`,
      icon: gpu.is_available ? 'pi pi-check' : 'pi pi-times',
      command: () => updateSelectedGpu(index),
    })) || []),
  ];

  return (
    <FooterContainer className='app-footer'>
      <GpuStatus className='gpu-status'>
        <GpuSelectorButton
          icon={selectedGpu?.is_available ? 'pi pi-desktop' : 'pi pi-microchip'}
          severity={selectedGpu?.is_available ? 'success' : 'info'}
          onClick={e => menuRef.current?.toggle(e)}
          aria-label="Select GPU"
          label={selectedGpu ? (selectedGpu.name || 'GPU') : 'CPU Only'}
          tooltip={selectedGpu ? `${selectedGpu.name}` : 'CPU Only'}
          tooltipOptions={{ position: 'top' }}
        />
        <Menu ref={menuRef} model={menuItems} popup />
      </GpuStatus>

      {/* Log buttons */}
      <LogButtons />

      {/* Notification button */}
      <NotificationButton />

      {/* Notification panel */}
      <NotificationPanel />

      {/* Debug component */}
      <StateDebugger />
    </FooterContainer>
  );
}
