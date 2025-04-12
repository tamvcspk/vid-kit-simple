import styled from '@emotion/styled';
import { Button } from 'primereact/button';

// Main footer container
export const FooterContainer = styled.footer`
  // Footer styles can be added here if needed
`;

// GPU status container
export const GpuStatus = styled.div`
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-color);
  font-size: 0.75rem;
`;

// GPU selector button
export const GpuSelectorButton = styled(Button)`
  &.p-button {
    background: transparent;
    border: 1px solid var(--surface-border);
    color: var(--text-color);
    padding: 0.25rem 0.5rem;
    transition: all 0.2s ease;
    font-size: 0.75rem;

    &:hover {
      background: var(--surface-hover);
      border-color: var(--primary-color);
    }

    &:focus {
      box-shadow: none;
    }

    .p-button-icon {
      font-size: 0.875rem;
    }
    
    .p-button-label {
      font-size: 0.75rem;
    }
  }
`;
