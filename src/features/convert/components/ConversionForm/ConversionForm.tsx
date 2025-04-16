import React from 'react';
import { Message } from 'primereact/message';
import { formatErrorForUser } from '../../../../utils/errorUtils';
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
        <Message
          severity="error"
          text={formatErrorForUser(error)}
          style={{ width: '100%', marginBottom: '1rem' }}
        />
      )}

      {/* Conversion options form */}
      {selectedFile && children}
    </SettingsPanelContainer>
  );
};
