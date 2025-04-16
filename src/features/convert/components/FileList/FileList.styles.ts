import styled from '@emotion/styled';

export const FileListPanel = styled.div`
  display: flex;
  flex-direction: column;
  flex: 7;
  background-color: var(--surface-ground);
  border-radius: 8px;
  padding: 1rem;
  margin-right: 1rem;
  height: 100%;
  min-height: 400px;
  overflow: hidden;
`;

export const FileListHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;

  h3 {
    margin: 0;
  }
`;

export const FileListContainer = styled.div`
  overflow-y: auto;
  flex: 1;
  padding-right: 0.5rem;

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
