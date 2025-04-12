import styled from '@emotion/styled';

// Main container
export const Container = styled.div`
  background-color: var(--surface-50);
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;

  h3 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    color: var(--primary-color);
  }

  .p-field {
    margin-bottom: 1.5rem;

    label {
      display: block;
      margin-bottom: 0.5rem;
      font-weight: 500;
    }
  }
`;

// Advanced options
export const AdvancedOptions = styled.div`
  background-color: var(--surface-100);
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
`;

// Actions container
export const ActionsContainer = styled.div`
  margin-top: 1.5rem;
  display: flex;
  justify-content: flex-end;
`;

// Progress container
export const ProgressContainer = styled.div`
  margin-top: 1.5rem;

  h4 {
    margin: 0 0 0.5rem 0;
  }
`;
