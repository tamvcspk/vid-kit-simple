import { useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Menu } from 'primereact/menu';
import { Button } from 'primereact/button';
import './AppFooter.scss';

interface GpuInfo {
  name: string;
  vendor: string;
  is_available: boolean;
  supported_codecs: string[];
}

interface GpuList {
  gpus: GpuInfo[];
}

export function AppFooter() {
  const [gpuList, setGpuList] = useState<GpuList | null>(null);
  const [selectedGpu, setSelectedGpu] = useState<GpuInfo | null>(null);
  const menuRef = useRef<Menu>(null);

  useEffect(() => {
    // Kiểm tra GPU khi component được mount
    invoke<GpuList>('check_gpu_availability')
      .then(info => {
        setGpuList(info);
      })
      .catch(error => {
        console.error('Failed to check GPU:', error);
      });
  }, []);

  const menuItems = [
    {
      label: 'CPU Only',
      icon: 'pi pi-microchip',
      command: () => setSelectedGpu(null),
    },
    ...(gpuList?.gpus.map((gpu, index) => ({
      label: `${gpu.name}`,
      icon: gpu.is_available ? 'pi pi-check' : 'pi pi-times',
      command: () => setSelectedGpu(gpu),
    })) || []),
  ];

  return (
    <footer className="app-footer">
      <div className="gpu-status">
        <Button
          className="gpu-selector-btn"
          icon={selectedGpu?.is_available ? 'pi pi-desktop' : 'pi pi-microchip'}
          severity={selectedGpu?.is_available ? 'success' : 'info'}
          onClick={e => menuRef.current?.toggle(e)}
          aria-label="Select GPU"
          label={selectedGpu ? `${selectedGpu.vendor}` : 'CPU Only'}
          tooltip={selectedGpu ? `${selectedGpu.name}` : 'CPU Only'}
          tooltipOptions={{ position: 'top' }}
        />
        <Menu ref={menuRef} model={menuItems} popup />
      </div>
    </footer>
  );
}
