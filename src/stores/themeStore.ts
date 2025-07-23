
import { defineStore } from 'pinia';

export const useThemeStore = defineStore('theme', {
  state: () => ({
    currentTheme: localStorage.getItem('theme') || 'geek',
  }),
  actions: {
    setTheme(theme: string) {
      this.currentTheme = theme;
      localStorage.setItem('theme', theme);
      document.documentElement.setAttribute('data-theme', theme);
    },
    initializeTheme() {
      document.documentElement.setAttribute('data-theme', this.currentTheme);
    }
  },
});
