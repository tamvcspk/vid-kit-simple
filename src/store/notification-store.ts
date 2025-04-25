import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { listen } from '@tauri-apps/api/event';
import { AppError, ErrorCategory } from '../utils';

export enum NotificationType {
  Error = 'error',
  Warning = 'warn',
  Info = 'info',
  Success = 'success'
}

export interface Notification {
  id: string;
  type: NotificationType;
  message: string;
  details?: string;
  timestamp: Date;
  read: boolean;
  source?: string;
  code?: number;
}

interface NotificationStore {
  // State
  notifications: Notification[];
  unreadCount: number;
  showPanel: boolean;
  filter: NotificationType | 'all';

  // Actions
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  clearNotifications: () => void;
  removeNotification: (id: string) => void;
  togglePanel: () => void;
  setFilter: (filter: NotificationType | 'all') => void;
}

export const useNotificationStore = create<NotificationStore>()(
  devtools(
    (set) => ({
      // Initial state
      notifications: [],
      unreadCount: 0,
      showPanel: false,
      filter: 'all',

      // Actions
      addNotification: (notification) => {
        const id = `notification-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
        const newNotification: Notification = {
          ...notification,
          id,
          timestamp: new Date(),
          read: false
        };

        set((state) => ({
          notifications: [newNotification, ...state.notifications],
          unreadCount: state.unreadCount + 1
        }));
      },

      markAsRead: (id) => {
        set((state) => {
          const notifications = state.notifications.map(notification =>
            notification.id === id ? { ...notification, read: true } : notification
          );

          const unreadCount = notifications.filter(n => !n.read).length;

          return { notifications, unreadCount };
        });
      },

      markAllAsRead: () => {
        set((state) => ({
          notifications: state.notifications.map(notification => ({ ...notification, read: true })),
          unreadCount: 0
        }));
      },

      clearNotifications: () => {
        set({ notifications: [], unreadCount: 0 });
      },

      removeNotification: (id) => {
        set((state) => {
          const notification = state.notifications.find(n => n.id === id);
          const isUnread = notification && !notification.read;

          return {
            notifications: state.notifications.filter(n => n.id !== id),
            unreadCount: isUnread ? state.unreadCount - 1 : state.unreadCount
          };
        });
      },

      togglePanel: () => {
        set((state) => ({ showPanel: !state.showPanel }));
      },

      setFilter: (filter) => {
        set({ filter });
      }
    }),
    { name: 'notification-store' }
  )
);

// Map error category to notification type
const mapErrorCategoryToNotificationType = (category: ErrorCategory): NotificationType => {
  switch (category) {
    case ErrorCategory.FFmpeg:
    case ErrorCategory.IO:
      return NotificationType.Error;
    case ErrorCategory.Validation:
    case ErrorCategory.Network:
      return NotificationType.Warning;
    case ErrorCategory.Other:
    default:
      return NotificationType.Info;
  }
};

// Listen for backend errors
listen<{ code: number; message: string; details?: string }>('backend-error', (event) => {
  const { code, message, details } = event.payload;

  useNotificationStore.getState().addNotification({
    type: NotificationType.Error,
    message,
    details,
    source: 'Backend',
    code
  });
}).catch(console.error);

// Listen for backend notifications
listen<{ level: string; message: string; details?: string }>('backend-notification', (event) => {
  const { level, message, details } = event.payload;

  let type: NotificationType;
  switch (level) {
    case 'error':
      type = NotificationType.Error;
      break;
    case 'warn':
      type = NotificationType.Warning;
      break;
    case 'success':
      type = NotificationType.Success;
      break;
    case 'info':
    default:
      type = NotificationType.Info;
      break;
  }

  useNotificationStore.getState().addNotification({
    type,
    message,
    details,
    source: 'Backend'
  });
}).catch(console.error);

// Helper function to add error notification from AppError
export const addErrorNotification = (error: AppError): void => {
  useNotificationStore.getState().addNotification({
    type: mapErrorCategoryToNotificationType(error.category),
    message: error.message,
    details: error.details,
    source: 'Frontend',
    code: error.code
  });
};

