import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

// Configuration
const CONSOLE_FORWARDING_ENABLED = true; // Set to false to disable console forwarding
const MAX_MESSAGE_LENGTH = 10000; // Truncate messages longer than this to avoid excessive logging

/**
 * Forward console methods to Tauri logger
 * This will send all console logs to the Tauri log file
 */
export function setupConsoleForwarding(): void {
  // Skip if disabled
  if (!CONSOLE_FORWARDING_ENABLED) {
    console.info('Console forwarding to Tauri logs is disabled');
    return;
  }
  forwardConsole('log', trace);
  forwardConsole('debug', debug);
  forwardConsole('info', info);
  forwardConsole('warn', warn);
  forwardConsole('error', error);

  console.info('Console forwarding to Tauri logs enabled');
}

/**
 * Replace a console method with a version that also calls the Tauri logger
 *
 * @param fnName The console method to forward
 * @param logger The Tauri logger function to call
 */
// Flag to prevent infinite recursion
let isForwarding = false;

function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = function(...args) {
    // Call the original console method
    original.apply(console, args);

    // Prevent infinite recursion
    if (isForwarding) return;

    try {
      isForwarding = true;

      // Convert all arguments to string and join them
      const message = args
        .map(arg => {
          if (typeof arg === 'object') {
            try {
              return JSON.stringify(arg);
            } catch (e) {
              return String(arg);
            }
          }
          return String(arg);
        })
        .join(' ');

      // Truncate message if it's too long
      const truncatedMessage = message.length > MAX_MESSAGE_LENGTH
        ? message.substring(0, MAX_MESSAGE_LENGTH) + `... [truncated, ${message.length - MAX_MESSAGE_LENGTH} more characters]`
        : message;

      // Send to Tauri logger
      logger(truncatedMessage).catch(e => {
        original.call(console, 'Failed to forward log to Tauri:', e);
      });
    } catch (e) {
      original.call(console, 'Error in console forwarding:', e);
    } finally {
      isForwarding = false;
    }
  };
}
