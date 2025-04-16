import React from 'react';
import { ErrorDisplay } from '../../../../components/common';
import { ConversionFormProps } from './types';
import { SettingsPanelContainer } from './ConversionForm.styles';

export const ConversionForm: React.FC<ConversionFormProps> = ({
  error,
  selectedFile,
  children
}) => {
  return (
    <SettingsPanelContainer>
      {/* Error message */}
      {error && (
        <ErrorDisplay
          error={error}
          showDetails={true}
          style={{ marginBottom: '1rem' }}
        />
      )}

      {/* Conversion options form */}
      {selectedFile && children}
    </SettingsPanelContainer>
  );
};
