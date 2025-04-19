import { useRef, useState, useEffect } from 'react';
import { OverlayPanel } from 'primereact/overlaypanel';
import { Button } from 'primereact/button';
import { SelectButton } from 'primereact/selectbutton';
import { Message } from 'primereact/message';

import { ScrollPanel } from 'primereact/scrollpanel';
import styled from '@emotion/styled';
import useNotificationStore, { NotificationType } from '../../store/notification-store';

const PanelContainer = styled.div`
  display: flex;
  flex-direction: column;
  height: 100%;
  max-height: 70vh;
  overflow: hidden;
`;

const NotificationHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding: 0.75rem 1rem 0.5rem;
  border-bottom: 1px solid var(--surface-border);
`;

const FilterContainer = styled.div`
  margin: 0 1rem 1rem;
`;

const NotificationList = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin: 0 1rem 1rem;
`;

const NotificationItem = styled.div<{ read: boolean }>`
  padding: 0.5rem;
  border-radius: 4px;
  background-color: ${props => props.read ? 'var(--surface-ground)' : 'var(--surface-50)'};
  border-left: 3px solid var(--primary-color);

  &.error {
    border-left-color: var(--red-500);
  }

  &.warn {
    border-left-color: var(--yellow-500);
  }

  &.info {
    border-left-color: var(--blue-500);
  }

  &.success {
    border-left-color: var(--green-500);
  }
`;

const NotificationTime = styled.div`
  font-size: 0.75rem;
  color: var(--text-color-secondary);
  margin-top: 0.25rem;
`;

const NotificationSource = styled.span`
  font-size: 0.75rem;
  background-color: var(--surface-200);
  padding: 0.125rem 0.375rem;
  border-radius: 4px;
  margin-left: 0.5rem;
`;

const NotificationDetails = styled.div`
  margin-top: 0.5rem;
  padding: 0.5rem;
  background-color: var(--surface-100);
  border-radius: 4px;
  font-family: monospace;
  font-size: 0.75rem;
  white-space: pre-wrap;
  max-height: 100px;
  overflow-y: auto;
`;

const EmptyMessage = styled.div`
  text-align: center;
  padding: 2rem;
  color: var(--text-color-secondary);
`;

export function NotificationPanel() {
  const {
    notifications,
    showPanel,
    filter,
    togglePanel,
    markAllAsRead,
    clearNotifications,
    markAsRead,
    removeNotification,
    setFilter
  } = useNotificationStore();

  const [expandedDetails, setExpandedDetails] = useState<string[]>([]);

  const filterOptions = [
    { label: 'All', value: 'all' },
    { label: 'Errors', value: NotificationType.Error },
    { label: 'Warnings', value: NotificationType.Warning },
    { label: 'Info', value: NotificationType.Info },
    { label: 'Success', value: NotificationType.Success }
  ];

  const toggleDetails = (id: string) => {
    setExpandedDetails(prev =>
      prev.includes(id)
        ? prev.filter(item => item !== id)
        : [...prev, id]
    );
  };

  const filteredNotifications = notifications.filter(notification =>
    filter === 'all' || notification.type === filter
  );

  const formatTimestamp = (date: Date) => {
    return new Date(date).toLocaleString();
  };

  const handleNotificationClick = (id: string) => {
    markAsRead(id);
  };

  const overlayPanelRef = useRef<OverlayPanel>(null);

  // Update panel visibility when showPanel changes
  useEffect(() => {
    if (showPanel && overlayPanelRef.current) {
      // We need a target element to show the panel
      // Since we don't have a direct reference to the button,
      // we'll use a dummy element or find the button by class
      const notificationButton = document.querySelector('.notification-button');
      if (notificationButton) {
        overlayPanelRef.current.show(null, notificationButton);
      }
    } else if (!showPanel && overlayPanelRef.current) {
      overlayPanelRef.current.hide();
    }
  }, [showPanel]);

  return (
    <OverlayPanel
      ref={overlayPanelRef}
      onHide={togglePanel}
      style={{ width: '450px', maxHeight: '80vh' }}
      className="notification-panel-overlay"
      showCloseIcon
      dismissable
      breakpoints={{ '960px': '75vw', '640px': '90vw' }}
      pt={{
        root: { className: 'notification-panel-root' },
        content: { className: 'notification-panel-content' }
      }}
    >
      <PanelContainer>
        <NotificationHeader>
          <h3 style={{ margin: 0 }}>Notifications</h3>
          <div className="notification-actions">
            <Button
              icon="pi pi-check"
              className="p-button-text p-button-rounded p-button-sm"
              tooltip="Mark all as read"
              tooltipOptions={{ position: 'top' }}
              onClick={markAllAsRead}
              disabled={notifications.length === 0 || notifications.every(n => n.read)}
            />
            <Button
              icon="pi pi-trash"
              className="p-button-text p-button-rounded p-button-sm"
              tooltip="Clear all notifications"
              tooltipOptions={{ position: 'top' }}
              onClick={clearNotifications}
              disabled={notifications.length === 0}
            />
          </div>
        </NotificationHeader>

        <FilterContainer>
          <SelectButton
            value={filter}
            options={filterOptions}
            onChange={(e) => setFilter(e.value)}
            className="p-buttonset-sm"
          />
        </FilterContainer>

        <ScrollPanel style={{ width: '100%', height: '350px' }}>
          {filteredNotifications.length > 0 ? (
            <NotificationList>
              {filteredNotifications.map(notification => (
                <NotificationItem
                  key={notification.id}
                  read={notification.read}
                  className={notification.type}
                  onClick={() => handleNotificationClick(notification.id)}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                    <Message
                      severity={notification.type}
                      text={notification.message}
                      style={{ width: '100%', marginBottom: '0.25rem' }}
                    />
                    <Button
                      icon="pi pi-times"
                      className="p-button-text p-button-rounded p-button-sm"
                      onClick={(e) => {
                        e.stopPropagation();
                        removeNotification(notification.id);
                      }}
                    />
                  </div>

                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <NotificationTime>
                      {formatTimestamp(notification.timestamp)}
                      {notification.source && (
                        <NotificationSource>{notification.source}</NotificationSource>
                      )}
                      {notification.code && (
                        <NotificationSource>Code: {notification.code}</NotificationSource>
                      )}
                    </NotificationTime>

                    {notification.details && (
                      <Button
                        label={expandedDetails.includes(notification.id) ? "Hide Details" : "Show Details"}
                        icon={expandedDetails.includes(notification.id) ? "pi pi-chevron-up" : "pi pi-chevron-down"}
                        className="p-button-text p-button-sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleDetails(notification.id);
                        }}
                      />
                    )}
                  </div>

                  {expandedDetails.includes(notification.id) && notification.details && (
                    <NotificationDetails>
                      {notification.details}
                    </NotificationDetails>
                  )}
                </NotificationItem>
              ))}
            </NotificationList>
          ) : (
            <EmptyMessage>
              <i className="pi pi-inbox" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block' }}></i>
              <p>No notifications to display</p>
            </EmptyMessage>
          )}
        </ScrollPanel>


      </PanelContainer>
    </OverlayPanel>
  );
}
