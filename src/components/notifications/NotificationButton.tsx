import { Button } from 'primereact/button';
import styled from '@emotion/styled';
import useNotificationStore from '../../store/notification-store';

const NotificationButtonContainer = styled.div`
  position: relative;
  margin-left: auto;
  margin-right: 1rem;
  display: flex;
  align-items: center;
`;

const StyledButton = styled(Button)<{ hasUnread: boolean }>`
  &.p-button {
    background: transparent;
    border: 1px solid var(--surface-border);
    color: ${props => props.hasUnread ? 'var(--red-500)' : 'var(--text-color)'};
    padding: 0.25rem 0.5rem;
    transition: all 0.2s ease;
    font-size: 0.75rem;

    &:hover {
      background: var(--surface-hover);
      border-color: ${props => props.hasUnread ? 'var(--red-500)' : 'var(--primary-color)'};
    }

    &:focus {
      box-shadow: none;
    }

    .p-button-icon {
      font-size: 0.875rem;
      color: ${props => props.hasUnread ? 'var(--red-500)' : 'inherit'};
    }

    .p-button-label {
      font-size: 0.75rem;
    }
  }
`;

export function NotificationButton() {
  const { unreadCount, togglePanel } = useNotificationStore();

  return (
    <NotificationButtonContainer>
      <StyledButton
        icon="pi pi-bell"
        onClick={togglePanel}
        aria-label="Notifications"
        tooltip="View Notifications"
        tooltipOptions={{ position: 'top' }}
        className="notification-button"
        hasUnread={unreadCount > 0}
      />
    </NotificationButtonContainer>
  );
}
