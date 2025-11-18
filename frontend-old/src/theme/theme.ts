import { createTheme } from '@mui/material/styles';

// Italian Modern Color Palette - Ispirato alla costa e cultura italiana
const italianPalette = {
  // Blu Mediterraneo - Mare italiano
  mediterranean: {
    50: '#E3F2FD',
    100: '#BBDEFB',
    200: '#90CAF9',
    300: '#64B5F6',
    400: '#42A5F5',
    500: '#0066CC', // Main
    600: '#1E88E5',
    700: '#1976D2',
    800: '#1565C0',
    900: '#003D7A', // Dark
  },
  // Terracotta - Architettura italiana
  terracotta: {
    50: '#FDF2F0',
    100: '#FCE4DE',
    200: '#F9C5B8',
    300: '#F5A892',
    400: '#F18A6C',
    500: '#E67E22', // Main
    600: '#D35400', // Dark
    700: '#C44B00',
    800: '#B54200',
    900: '#9A3600',
  },
  // Verde Oliva - Paesaggio toscano
  oliva: {
    50: '#F5F7F0',
    100: '#E8EDD9',
    200: '#D4DCB8',
    300: '#C0CB97',
    400: '#ACBA76',
    500: '#6B8E23', // Main
    600: '#5F7D1F',
    700: '#536C1C',
    800: '#475B18',
    900: '#3B4A14',
  },
  // Crema - Marmo di Carrara
  crema: {
    50: '#FEFDFB',
    100: '#FDFBF7',
    200: '#FBF8F0',
    300: '#F9F5E8',
    400: '#F7F2E1',
    500: '#F5EFDA',
    600: '#E8DFC4',
    700: '#DBCFAE',
    800: '#CEBF98',
    900: '#C1AF82',
  },
};

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: italianPalette.mediterranean[500], // Blu mediterraneo
      light: italianPalette.mediterranean[300],
      dark: italianPalette.mediterranean[900],
      contrastText: '#ffffff',
    },
    secondary: {
      main: italianPalette.terracotta[500], // Terracotta italiana
      light: italianPalette.terracotta[300],
      dark: italianPalette.terracotta[600],
      contrastText: '#ffffff',
    },
    tertiary: {
      main: italianPalette.oliva[500], // Verde oliva
      light: italianPalette.oliva[300],
      dark: italianPalette.oliva[700],
      contrastText: '#ffffff',
    },
    error: {
      main: '#E53E3E',
      light: '#FC8181',
      dark: '#C53030',
    },
    warning: {
      main: '#DD6B20',
      light: '#F6AD55',
      dark: '#C05621',
    },
    info: {
      main: italianPalette.mediterranean[400],
      light: italianPalette.mediterranean[200],
      dark: italianPalette.mediterranean[700],
    },
    success: {
      main: '#38A169',
      light: '#68D391',
      dark: '#2F855A',
    },
    background: {
      default: italianPalette.crema[50], // Sfondo crema elegante
      paper: '#FFFFFF',
      subtle: italianPalette.crema[100],
    },
    text: {
      primary: '#1A202C', // Quasi nero per leggibilità
      secondary: '#4A5568', // Grigio caldo
      disabled: 'rgba(26, 32, 44, 0.4)',
    },
    action: {
      hover: 'rgba(0, 102, 204, 0.04)',
      selected: 'rgba(0, 102, 204, 0.08)',
      disabled: 'rgba(26, 32, 44, 0.26)',
      disabledBackground: 'rgba(26, 32, 44, 0.12)',
    },
    divider: 'rgba(0, 102, 204, 0.12)',
    grey: {
      50: '#F7FAFC',
      100: '#EDF2F7',
      200: '#E2E8F0',
      300: '#CBD5E0',
      400: '#A0AEC0',
      500: '#718096',
      600: '#4A5568',
      700: '#2D3748',
      800: '#1A202C',
      900: '#171923',
    },
  },
  typography: {
    // Famiglia font primaria - Inter per leggibilità moderna
    fontFamily: [
      'Inter',
      '-apple-system',
      'BlinkMacSystemFont',
      '"Segoe UI"',
      'Roboto',
      '"Helvetica Neue"',
      'Arial',
      'sans-serif',
    ].join(','),

    // Font famiglia per titoli eleganti
    h1: {
      fontFamily: [
        'Playfair Display',
        'Georgia',
        'Times New Roman',
        'serif',
      ].join(','),
      fontSize: '3rem',
      fontWeight: 700,
      lineHeight: 1.2,
      letterSpacing: '-0.02em',
      color: '#1A202C',
    },
    h2: {
      fontFamily: [
        'Playfair Display',
        'Georgia',
        'Times New Roman',
        'serif',
      ].join(','),
      fontSize: '2.25rem',
      fontWeight: 600,
      lineHeight: 1.3,
      letterSpacing: '-0.01em',
      color: '#1A202C',
    },
    h3: {
      fontFamily: 'Inter',
      fontSize: '1.875rem',
      fontWeight: 600,
      lineHeight: 1.4,
      letterSpacing: '-0.005em',
      color: '#1A202C',
    },
    h4: {
      fontFamily: 'Inter',
      fontSize: '1.5rem',
      fontWeight: 600,
      lineHeight: 1.4,
      color: '#1A202C',
    },
    h5: {
      fontFamily: 'Inter',
      fontSize: '1.25rem',
      fontWeight: 600,
      lineHeight: 1.5,
      color: '#2D3748',
    },
    h6: {
      fontFamily: 'Inter',
      fontSize: '1.125rem',
      fontWeight: 600,
      lineHeight: 1.6,
      color: '#2D3748',
    },
    subtitle1: {
      fontFamily: 'Inter',
      fontSize: '1rem',
      fontWeight: 500,
      lineHeight: 1.5,
      color: '#4A5568',
    },
    subtitle2: {
      fontFamily: 'Inter',
      fontSize: '0.875rem',
      fontWeight: 600,
      lineHeight: 1.4,
      color: '#4A5568',
      textTransform: 'uppercase',
      letterSpacing: '0.05em',
    },
    body1: {
      fontFamily: 'Inter',
      fontSize: '1rem',
      fontWeight: 400,
      lineHeight: 1.6,
      color: '#1A202C',
    },
    body2: {
      fontFamily: 'Inter',
      fontSize: '0.875rem',
      fontWeight: 400,
      lineHeight: 1.5,
      color: '#4A5568',
    },
    caption: {
      fontFamily: 'Inter',
      fontSize: '0.75rem',
      fontWeight: 400,
      lineHeight: 1.4,
      color: '#718096',
    },
    overline: {
      fontFamily: 'Inter',
      fontSize: '0.75rem',
      fontWeight: 600,
      lineHeight: 1.4,
      textTransform: 'uppercase',
      letterSpacing: '0.1em',
      color: '#718096',
    },
    button: {
      fontFamily: 'Inter',
      fontSize: '0.875rem',
      fontWeight: 600,
      lineHeight: 1.2,
      textTransform: 'none',
      letterSpacing: '0.025em',
    },
  },
  shape: {
    borderRadius: 12, // Bordi più morbidi e moderni
  },
  spacing: 8,

  // Ombre italiane - ispirate all'architettura mediterranea
  shadows: [
    'none',
    '0px 2px 4px rgba(0, 102, 204, 0.05)', // shadow 1
    '0px 4px 8px rgba(0, 102, 204, 0.08)', // shadow 2
    '0px 8px 16px rgba(0, 102, 204, 0.10)', // shadow 3
    '0px 12px 24px rgba(0, 102, 204, 0.12)', // shadow 4
    '0px 16px 32px rgba(0, 102, 204, 0.14)', // shadow 5
    '0px 20px 40px rgba(0, 102, 204, 0.16)', // shadow 6
    '0px 24px 48px rgba(0, 102, 204, 0.18)', // shadow 7
    '0px 32px 64px rgba(0, 102, 204, 0.20)', // shadow 8
    '0px 40px 80px rgba(0, 102, 204, 0.22)', // shadow 9
    '0px 48px 96px rgba(0, 102, 204, 0.24)', // shadow 10
    // Continuano fino a 25...
    ...Array(15).fill('0px 48px 96px rgba(0, 102, 204, 0.24)'),
  ],

  // Breakpoints italiani - responsive design
  breakpoints: {
    values: {
      xs: 0,
      sm: 640,
      md: 768,
      lg: 1024,
      xl: 1280,
    },
  },

  // Z-index layers
  zIndex: {
    mobileStepper: 1000,
    fab: 1050,
    speedDial: 1050,
    appBar: 1100,
    drawer: 1200,
    modal: 1300,
    snackbar: 1400,
    tooltip: 1500,
  },
  components: {
    // Bottoni eleganti italiani
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          textTransform: 'none',
          fontWeight: 600,
          padding: '12px 24px',
          fontSize: '0.875rem',
          transition: 'all 0.2s ease-in-out',
          letterSpacing: '0.025em',
          '&:hover': {
            transform: 'translateY(-1px)',
            boxShadow: '0px 8px 16px rgba(0, 102, 204, 0.20)',
          },
        },
        contained: {
          background: 'linear-gradient(135deg, #0066CC 0%, #003D7A 100%)',
          color: '#ffffff',
          '&:hover': {
            background: 'linear-gradient(135deg, #003D7A 0%, #002952 100%)',
          },
        },
        outlined: {
          borderColor: '#0066CC',
          color: '#0066CC',
          borderWidth: '2px',
          '&:hover': {
            borderColor: '#003D7A',
            backgroundColor: 'rgba(0, 102, 204, 0.04)',
            borderWidth: '2px',
          },
        },
        text: {
          color: '#0066CC',
          '&:hover': {
            backgroundColor: 'rgba(0, 102, 204, 0.04)',
          },
        },
        sizeLarge: {
          padding: '16px 32px',
          fontSize: '1rem',
        },
        sizeSmall: {
          padding: '8px 16px',
          fontSize: '0.75rem',
        },
      },
    },

    // Card italiane con glassmorphism
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 16,
          border: '1px solid rgba(255, 255, 255, 0.18)',
          backdropFilter: 'blur(10px)',
          background: 'rgba(255, 255, 255, 0.95)',
          boxShadow: '0px 8px 32px rgba(0, 102, 204, 0.08)',
          transition: 'all 0.3s ease-in-out',
          '&:hover': {
            transform: 'translateY(-4px)',
            boxShadow: '0px 20px 40px rgba(0, 102, 204, 0.12)',
          },
        },
      },
    },

    // AppBar mediterranea
    MuiAppBar: {
      styleOverrides: {
        root: {
          background: 'linear-gradient(135deg, #0066CC 0%, #003D7A 100%)',
          backdropFilter: 'blur(10px)',
          borderBottom: '1px solid rgba(255, 255, 255, 0.12)',
          boxShadow: '0px 4px 20px rgba(0, 102, 204, 0.15)',
        },
      },
    },

    // Drawer elegante
    MuiDrawer: {
      styleOverrides: {
        paper: {
          background: 'linear-gradient(180deg, #FFFFFF 0%, #FEFDFB 100%)',
          borderRight: '1px solid rgba(0, 102, 204, 0.08)',
          backdropFilter: 'blur(10px)',
        },
      },
    },

    // Chip italiani
    MuiChip: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          fontWeight: 600,
          fontSize: '0.75rem',
          letterSpacing: '0.025em',
        },
        filled: {
          background: 'linear-gradient(135deg, #E67E22 0%, #D35400 100%)',
          color: '#ffffff',
        },
        outlined: {
          borderColor: '#0066CC',
          color: '#0066CC',
          '&:hover': {
            backgroundColor: 'rgba(0, 102, 204, 0.04)',
          },
        },
      },
    },

    // TextField elegant
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root': {
            borderRadius: 12,
            background: 'rgba(255, 255, 255, 0.8)',
            backdropFilter: 'blur(10px)',
            transition: 'all 0.2s ease-in-out',
            '&:hover': {
              '& .MuiOutlinedInput-notchedOutline': {
                borderColor: '#0066CC',
              },
            },
            '&.Mui-focused': {
              '& .MuiOutlinedInput-notchedOutline': {
                borderColor: '#0066CC',
                borderWidth: '2px',
              },
            },
          },
        },
      },
    },

    // Avatar italiano
    MuiAvatar: {
      styleOverrides: {
        root: {
          background: 'linear-gradient(135deg, #E67E22 0%, #D35400 100%)',
          color: '#ffffff',
          fontWeight: 600,
        },
      },
    },

    // Badge elegante
    MuiBadge: {
      styleOverrides: {
        badge: {
          background: 'linear-gradient(135deg, #E67E22 0%, #D35400 100%)',
          color: '#ffffff',
          fontWeight: 600,
          fontSize: '0.75rem',
        },
      },
    },

    // Paper con glassmorphism
    MuiPaper: {
      styleOverrides: {
        root: {
          borderRadius: 16,
          backdropFilter: 'blur(10px)',
          background: 'rgba(255, 255, 255, 0.95)',
          border: '1px solid rgba(255, 255, 255, 0.18)',
        },
        elevation1: {
          boxShadow: '0px 4px 16px rgba(0, 102, 204, 0.06)',
        },
        elevation2: {
          boxShadow: '0px 8px 24px rgba(0, 102, 204, 0.08)',
        },
        elevation3: {
          boxShadow: '0px 12px 32px rgba(0, 102, 204, 0.10)',
        },
      },
    },

    // Lista items moderna
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          margin: '4px 8px',
          transition: 'all 0.2s ease-in-out',
          '&:hover': {
            backgroundColor: 'rgba(0, 102, 204, 0.04)',
            transform: 'translateX(4px)',
          },
          '&.Mui-selected': {
            backgroundColor: 'rgba(0, 102, 204, 0.08)',
            borderLeft: '4px solid #0066CC',
            '&:hover': {
              backgroundColor: 'rgba(0, 102, 204, 0.12)',
            },
          },
        },
      },
    },

    // Dialog moderno
    MuiDialog: {
      styleOverrides: {
        paper: {
          borderRadius: 20,
          background: 'rgba(255, 255, 255, 0.95)',
          backdropFilter: 'blur(20px)',
          border: '1px solid rgba(255, 255, 255, 0.18)',
        },
      },
    },

    // Fab italiano
    MuiFab: {
      styleOverrides: {
        root: {
          background: 'linear-gradient(135deg, #E67E22 0%, #D35400 100%)',
          color: '#ffffff',
          boxShadow: '0px 8px 24px rgba(230, 126, 34, 0.25)',
          '&:hover': {
            background: 'linear-gradient(135deg, #D35400 0%, #B8410E 100%)',
            transform: 'translateY(-2px)',
            boxShadow: '0px 12px 32px rgba(230, 126, 34, 0.35)',
          },
        },
      },
    },
  },
});

export default theme;
export { theme as italianTheme };