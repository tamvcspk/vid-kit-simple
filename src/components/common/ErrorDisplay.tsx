import React from 'react';
import { Message } from 'primereact/message';
import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { AppError, formatErrorForUser } from '../../utils';

interface ErrorDisplayProps {
  error: AppError | null;
  onDismiss?: () => void;
  showDetails?: boolean;
  showDismissButton?: boolean;
  severity?: 'error' | 'warn' | 'info' | 'success';
  style?: React.CSSProperties;
  className?: string;
}

/**
 * Reusable error display component
 */
export const ErrorDisplay: React.FC<ErrorDisplayProps> = ({
  error,
  onDismiss,
  showDetails = false,
  showDismissButton = true,
  severity = 'error',
  style,
  className
}) => {
  const [detailsVisible, setDetailsVisible] = React.useState(false);

  if (!error) return null;

  const formattedMessage = formatErrorForUser(error);
  
  const toggleDetails = () => {
    setDetailsVisible(!detailsVisible);
  };

  return (
    <div className={`error-display ${className || ''}`} style={{ ...style }}>
      <Message
        severity={severity}
        text={formattedMessage}
        style={{ width: '100%', marginBottom: '0.5rem' }}
      />
      
      <div className="error-actions" style={{ display: 'flex', gap: '0.5rem', justifyContent: 'flex-end' }}>
        {showDetails && error.details && (
          <Button
            label="Details"
            icon="pi pi-info-circle"
            onClick={toggleDetails}
            className="p-button-text p-button-sm"
          />
        )}
        
        {showDismissButton && onDismiss && (
          <Button
            label="Dismiss"
            icon="pi pi-times"
            onClick={onDismiss}
            className="p-button-text p-button-sm"
          />
        )}
      </div>
      
      {showDetails && error.details && (
        <Dialog
          header="Error Details"
          visible={detailsVisible}
          style={{ width: '50vw' }}
          onHide={() => setDetailsVisible(false)}
        >
          <div className="error-details-content">
            <h4>Error Message</h4>
            <p>{error.message}</p>
            
            <h4>Category</h4>
            <p>{error.category.toUpperCase()}</p>
            
            {error.code && (
              <>
                <h4>Error Code</h4>
                <p>{error.code}</p>
              </>
            )}
            
            <h4>Details</h4>
            <div style={{ 
              background: '#f8f9fa', 
              padding: '1rem', 
              borderRadius: '4px',
              maxHeight: '300px',
              overflow: 'auto',
              fontFamily: 'monospace',
              fontSize: '0.9rem'
            }}>
              <pre>{error.details}</pre>
            </div>
            
            <h4>Timestamp</h4>
            <p>{error.timestamp.toLocaleString()}</p>
          </div>
        </Dialog>
      )}
    </div>
  );
};

export default ErrorDisplay;
