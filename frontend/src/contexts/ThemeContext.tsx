'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { italianTheme } from '@/theme/theme';

type ThemeMode = 'light' | 'dark' | 'auto';

interface ThemeContextType {
  mode: ThemeMode;
  toggleTheme: () => void;
  setThemeMode: (mode: ThemeMode) => void;
  isDarkMode: boolean;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const useThemeMode = () => {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useThemeMode must be used within a ThemeContextProvider');
  }
  return context;
};

interface ThemeContextProviderProps {
  children: ReactNode;
}

export default function ThemeContextProvider({ children }: ThemeContextProviderProps) {
  const [mode, setMode] = useState<ThemeMode>('auto');
  const [systemPrefersDark, setSystemPrefersDark] = useState(false);
  const [mounted, setMounted] = useState(false);

  // Calculate actual dark mode state
  const isDarkMode = mode === 'dark' || (mode === 'auto' && systemPrefersDark);

  // Load saved theme preference on mount
  useEffect(() => {
    const savedMode = localStorage.getItem('themeMode') as ThemeMode;
    if (savedMode && ['light', 'dark', 'auto'].includes(savedMode)) {
      setMode(savedMode);
    }

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    setSystemPrefersDark(mediaQuery.matches);

    const handleChange = (e: MediaQueryListEvent) => {
      setSystemPrefersDark(e.matches);
    };

    mediaQuery.addEventListener('change', handleChange);
    setMounted(true);

    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  }, []);

  // Save theme preference when it changes
  useEffect(() => {
    if (mounted) {
      localStorage.setItem('themeMode', mode);
    }
  }, [mode, mounted]);

  // Apply theme class to document
  useEffect(() => {
    if (mounted) {
      document.documentElement.classList.toggle('dark', isDarkMode);
      document.documentElement.setAttribute('data-theme', isDarkMode ? 'dark' : 'light');
    }
  }, [isDarkMode, mounted]);

  const toggleTheme = () => {
    setMode(current => {
      switch (current) {
        case 'light':
          return 'dark';
        case 'dark':
          return 'auto';
        case 'auto':
          return 'light';
        default:
          return 'light';
      }
    });
  };

  const setThemeMode = (newMode: ThemeMode) => {
    setMode(newMode);
  };

  // Create theme based on current mode
  const currentTheme = createTheme({
    ...italianTheme,
    palette: {
      ...italianTheme.palette,
      mode: isDarkMode ? 'dark' : 'light',
      ...(isDarkMode && {
        // Dark mode overrides
        background: {
          default: '#1A1A1A',
          paper: 'rgba(42, 42, 42, 0.95)',
        },
        text: {
          primary: '#FEFDFB', // crema-50
          secondary: '#F5E6D3', // crema-300
        },
        divider: 'rgba(255, 255, 255, 0.1)',
        action: {
          hover: 'rgba(255, 255, 255, 0.08)',
          selected: 'rgba(255, 255, 255, 0.12)',
          disabled: 'rgba(255, 255, 255, 0.3)',
        },
      }),
    },
    components: {
      ...italianTheme.components,
      ...(isDarkMode && {
        // Dark mode component overrides
        MuiCard: {
          styleOverrides: {
            root: {
              background: 'rgba(42, 42, 42, 0.95)',
              backdropFilter: 'blur(10px)',
              border: '1px solid rgba(255, 255, 255, 0.1)',
            },
          },
        },
        MuiPaper: {
          styleOverrides: {
            root: {
              background: 'rgba(42, 42, 42, 0.95)',
              backdropFilter: 'blur(10px)',
              border: '1px solid rgba(255, 255, 255, 0.1)',
            },
          },
        },
        MuiAppBar: {
          styleOverrides: {
            root: {
              background: 'linear-gradient(135deg, #003D7A, #1565C0)',
              backdropFilter: 'blur(10px)',
              borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
            },
          },
        },
        MuiDrawer: {
          styleOverrides: {
            paper: {
              background: 'rgba(42, 42, 42, 0.98)',
              backdropFilter: 'blur(15px)',
              borderRight: '1px solid rgba(255, 255, 255, 0.1)',
            },
          },
        },
        MuiTextField: {
          styleOverrides: {
            root: {
              '& .MuiOutlinedInput-root': {
                backgroundColor: 'rgba(42, 42, 42, 0.8)',
                '& fieldset': {
                  borderColor: 'rgba(255, 255, 255, 0.2)',
                },
                '&:hover fieldset': {
                  borderColor: 'rgba(255, 255, 255, 0.4)',
                },
                '&.Mui-focused fieldset': {
                  borderColor: '#42A5F5',
                },
              },
              '& .MuiInputBase-input': {
                color: '#FEFDFB',
              },
              '& .MuiInputLabel-root': {
                color: '#F5E6D3',
                '&.Mui-focused': {
                  color: '#42A5F5',
                },
              },
            },
          },
        },
        MuiButton: {
          styleOverrides: {
            outlined: {
              borderColor: 'rgba(255, 255, 255, 0.3)',
              color: '#FEFDFB',
              '&:hover': {
                borderColor: '#42A5F5',
                backgroundColor: 'rgba(66, 165, 245, 0.1)',
              },
            },
          },
        },
      }),
    },
  });

  const contextValue: ThemeContextType = {
    mode,
    toggleTheme,
    setThemeMode,
    isDarkMode,
  };

  // Prevent hydration mismatch by not rendering until mounted
  if (!mounted) {
    return null;
  }

  return (
    <ThemeContext.Provider value={contextValue}>
      <ThemeProvider theme={currentTheme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </ThemeContext.Provider>
  );
}