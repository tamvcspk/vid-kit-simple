import { ReactNode } from 'react';
import { CustomError } from '../../../../types';

export interface ConversionFormProps {
  error: CustomError | null;
  selectedFile: string | null;
  children: ReactNode;
}
