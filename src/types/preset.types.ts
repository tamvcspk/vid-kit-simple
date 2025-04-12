import { ProcessingOptions } from './video.types';

export interface Preset {
  name: string;
  description: string;
  options: ProcessingOptions;
}
