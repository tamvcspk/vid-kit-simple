import {
  useNotificationStore,
  NotificationType,
  addErrorNotification
} from '../store/notification-store';

/**
 * Custom hook to use the notification system
 */
export function useNotifications() {
  const {
    notifications,
    unreadCount,
    showPanel,
    filter,
    addNotification,
    markAsRead,
    markAllAsRead,
    clearNotifications,
    removeNotification,
    togglePanel,
    setFilter
  } = useNotificationStore();

  /**
   * Add an info notification
   */
  const addInfo = (message: string, details?: string) => {
    addNotification({
      type: NotificationType.Info,
      message,
      details,
      source: 'Frontend'
    });
  };

  /**
   * Add a success notification
   */
  const addSuccess = (message: string, details?: string) => {
    addNotification({
      type: NotificationType.Success,
      message,
      details,
      source: 'Frontend'
    });
  };

  /**
   * Add a warning notification
   */
  const addWarning = (message: string, details?: string) => {
    addNotification({
      type: NotificationType.Warning,
      message,
      details,
      source: 'Frontend'
    });
  };

  /**
   * Add an error notification
   */
  const addError = (message: string, details?: string) => {
    addNotification({
      type: NotificationType.Error,
      message,
      details,
      source: 'Frontend'
    });
  };

  return {
    // State
    notifications,
    unreadCount,
    showPanel,
    filter,

    // Actions
    addNotification,
    addInfo,
    addSuccess,
    addWarning,
    addError,
    addErrorNotification,
    markAsRead,
    markAllAsRead,
    clearNotifications,
    removeNotification,
    togglePanel,
    setFilter
  };
}
