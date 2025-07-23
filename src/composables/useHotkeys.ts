
import { onMounted, onUnmounted } from 'vue';

type HotkeyCallback = (event: KeyboardEvent) => void;

interface HotkeyOptions {
  ctrl?: boolean;
  meta?: boolean;
  shift?: boolean;
  alt?: boolean;
}

export function useHotkeys(key: string, callback: HotkeyCallback, options: HotkeyOptions = {}) {
  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key.toLowerCase() === key.toLowerCase()) {
      const { ctrl = false, meta = false, shift = false, alt = false } = options;
      if (
        (ctrl ? event.ctrlKey : !event.ctrlKey) &&
        (meta ? event.metaKey : !event.metaKey) &&
        (shift ? event.shiftKey : !event.shiftKey) &&
        (alt ? event.altKey : !event.altKey)
      ) {
        event.preventDefault();
        callback(event);
      }
    }
  };

  onMounted(() => {
    window.addEventListener('keydown', handleKeyDown);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });
}
