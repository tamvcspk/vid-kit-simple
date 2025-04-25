import { Button } from 'primereact/button';
import styled from '@emotion/styled';
import { useNotificationStore } from '../../store/notification-store';

const NotificationButtonContainer = styled.div`
  position: relative;
  display: flex;
  align-items: center;
`;

const StyledButton = styled(Button)`
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

  &.p-button.has-unread {
    color: var(--red-500);

    &:hover {
      border-color: var(--red-500);
    }

    .p-button-icon {
      color: var(--red-500);
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
        className={`notification-button ${unreadCount > 0 ? 'has-unread' : ''}`}
      />
    </NotificationButtonContainer>
  );
}
