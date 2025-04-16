import styled from '@emotion/styled';

export const SettingsPanelContainer = styled.div`
  display: flex;
  flex-direction: column;
  flex: 3;
  background-color: var(--surface-card);
  border-radius: 8px;
  padding: 1rem;
  overflow-y: auto;
  max-height: 100%;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: var(--surface-ground);
  }

  &::-webkit-scrollbar-thumb {
    background-color: var(--surface-border);
    border-radius: 6px;
  }
`;
