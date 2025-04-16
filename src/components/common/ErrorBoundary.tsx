import React, { Component, ErrorInfo, ReactNode } from 'react';
import { Message } from 'primereact/message';
import { Button } from 'primereact/button';
import { AppError, ErrorCategory, createError, logError } from '../../utils';

interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: AppError) => void;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: AppError | null;
}

/**
 * Error boundary component to catch rendering errors
 */
export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null
    };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    // Update state so the next render will show the fallback UI
    return {
      hasError: true,
      error: createError(
        ErrorCategory.Other,
        `Rendering error: ${error.message}`,
        error.stack,
        error
      )
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    // Log the error
    const appError = createError(
      ErrorCategory.Other,
      `Rendering error: ${error.message}`,
      errorInfo.componentStack,
      error
    );
    
    logError(appError);
    
    // Call the onError callback if provided
    if (this.props.onError) {
      this.props.onError(appError);
    }
  }

  handleReset = (): void => {
    this.setState({
      hasError: false,
      error: null
    });
  };

  render(): ReactNode {
    if (this.state.hasError) {
      // Render custom fallback UI if provided
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Render default fallback UI
      return (
        <div className="error-boundary-container" style={{ padding: '1rem', textAlign: 'center' }}>
          <Message
            severity="error"
            text={this.state.error?.message || 'An unexpected error occurred'}
            style={{ width: '100%', marginBottom: '1rem' }}
          />
          
          {this.state.error?.details && (
            <div className="error-details" style={{ 
              textAlign: 'left', 
              padding: '1rem', 
              background: '#f8f9fa', 
              borderRadius: '4px',
              marginBottom: '1rem',
              maxHeight: '200px',
              overflow: 'auto',
              fontSize: '0.8rem',
              fontFamily: 'monospace'
            }}>
              <pre>{this.state.error.details}</pre>
            </div>
          )}
          
          <Button 
            label="Try Again" 
            icon="pi pi-refresh" 
            onClick={this.handleReset} 
            className="p-button-sm"
          />
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
