import styled from '@emotion/styled';

interface DropZoneContainerProps {
  isDragging: boolean;
  hasFile: boolean;
}

export const DropZoneContainer = styled.div<DropZoneContainerProps>`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  border: 2px dashed ${props => (props.isDragging ? 'var(--primary-color)' : 'var(--surface-border)')};
  border-radius: 8px;
  background-color: ${props => (props.isDragging ? 'var(--surface-hover)' : 'var(--surface-ground)')};
  cursor: pointer;
  transition: all 0.2s;
  height: 100%;
  min-height: 200px;

  &:hover {
    background-color: var(--surface-hover);
    border-color: var(--primary-color);
  }
`;

export const UploadIcon = styled.i`
  font-size: 3rem;
  color: var(--primary-color);
  margin-bottom: 1rem;
`;

export const UploadText = styled.p`
  font-size: 1rem;
  color: var(--text-color-secondary);
  text-align: center;
  margin: 0;
`;

export const UploadingContainer = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`;
