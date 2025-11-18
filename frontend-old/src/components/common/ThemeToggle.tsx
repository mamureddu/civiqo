'use client';

import React from 'react';
import {
  IconButton,
  Tooltip,
  Box,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
  Chip,
} from '@mui/material';
import {
  Brightness6 as ThemeIcon,
  LightMode as LightModeIcon,
  DarkMode as DarkModeIcon,
  SettingsBrightness as AutoModeIcon,
} from '@mui/icons-material';
import { useTranslation } from 'react-i18next';
import { useThemeMode } from '@/contexts/ThemeContext';

interface ThemeToggleProps {
  compact?: boolean;
}

export default function ThemeToggle({ compact = false }: ThemeToggleProps) {
  const { t } = useTranslation('common');
  const { mode, toggleTheme, setThemeMode, isDarkMode } = useThemeMode();
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    if (compact) {
      toggleTheme();
    } else {
      setAnchorEl(event.currentTarget);
    }
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleModeSelect = (newMode: 'light' | 'dark' | 'auto') => {
    setThemeMode(newMode);
    handleClose();
  };

  const getCurrentIcon = () => {
    switch (mode) {
      case 'light':
        return <LightModeIcon />;
      case 'dark':
        return <DarkModeIcon />;
      case 'auto':
        return <AutoModeIcon />;
      default:
        return <ThemeIcon />;
    }
  };

  const getModeLabel = (modeType: 'light' | 'dark' | 'auto') => {
    switch (modeType) {
      case 'light':
        return t('theme.light');
      case 'dark':
        return t('theme.dark');
      case 'auto':
        return t('theme.auto');
      default:
        return t('theme.mode');
    }
  };

  if (compact) {
    return (
      <Tooltip title={t('theme.toggle')} arrow>
        <IconButton
          onClick={handleClick}
          color="inherit"
          className="hover-glow"
          sx={{
            background: 'rgba(255, 255, 255, 0.1)',
            backdropFilter: 'blur(5px)',
            border: '1px solid rgba(255, 255, 255, 0.2)',
            transition: 'all var(--transition-fast)',
            '&:hover': {
              background: 'rgba(255, 255, 255, 0.2)',
              transform: 'scale(1.05)',
            },
          }}
          aria-label={t('theme.toggle')}
        >
          {getCurrentIcon()}
        </IconButton>
      </Tooltip>
    );
  }

  return (
    <Box>
      <Tooltip title={t('theme.selectMode')} arrow>
        <IconButton
          onClick={handleClick}
          color="inherit"
          className="hover-glow"
          sx={{
            background: 'rgba(255, 255, 255, 0.1)',
            backdropFilter: 'blur(5px)',
            border: '1px solid rgba(255, 255, 255, 0.2)',
            transition: 'all var(--transition-fast)',
            '&:hover': {
              background: 'rgba(255, 255, 255, 0.2)',
              transform: 'scale(1.05)',
            },
          }}
          aria-controls={open ? 'theme-menu' : undefined}
          aria-haspopup="true"
          aria-expanded={open ? 'true' : undefined}
          aria-label={t('theme.selectMode')}
        >
          {getCurrentIcon()}
        </IconButton>
      </Tooltip>

      <Menu
        id="theme-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        className="glass"
        PaperProps={{
          sx: {
            background: 'var(--glass-bg)',
            backdropFilter: 'var(--glass-blur)',
            border: '1px solid var(--glass-border)',
            borderRadius: 2,
            minWidth: 180,
            mt: 1,
          },
        }}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
      >
        {(['light', 'dark', 'auto'] as const).map((modeType) => (
          <MenuItem
            key={modeType}
            onClick={() => handleModeSelect(modeType)}
            selected={mode === modeType}
            sx={{
              borderRadius: 1,
              mx: 0.5,
              my: 0.25,
              transition: 'all var(--transition-fast)',
              '&:hover': {
                background: 'linear-gradient(135deg, rgba(0, 102, 204, 0.1), rgba(230, 126, 34, 0.1))',
              },
              '&.Mui-selected': {
                background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
                color: 'white',
                '&:hover': {
                  background: 'linear-gradient(135deg, var(--mediterranean-600), var(--mediterranean-700))',
                },
              },
            }}
          >
            <ListItemIcon
              sx={{
                color: mode === modeType ? 'white' : 'inherit',
                minWidth: 36,
              }}
            >
              {modeType === 'light' && <LightModeIcon fontSize="small" />}
              {modeType === 'dark' && <DarkModeIcon fontSize="small" />}
              {modeType === 'auto' && <AutoModeIcon fontSize="small" />}
            </ListItemIcon>

            <ListItemText
              primary={getModeLabel(modeType)}
              primaryTypographyProps={{
                className: 'font-body',
                sx: {
                  fontWeight: mode === modeType ? 600 : 500,
                  fontSize: '0.9rem',
                },
              }}
            />

            {mode === modeType && (
              <Chip
                label={t('common.active')}
                size="small"
                sx={{
                  height: 20,
                  fontSize: '0.7rem',
                  background: 'rgba(255, 255, 255, 0.2)',
                  color: 'white',
                  fontWeight: 600,
                }}
              />
            )}
          </MenuItem>
        ))}

        <Box sx={{ px: 2, py: 1, mt: 1, borderTop: '1px solid var(--glass-border)' }}>
          <Box display="flex" alignItems="center" justifyContent="space-between">
            <Box
              component="span"
              className="font-body"
              sx={{
                fontSize: '0.75rem',
                color: 'text.secondary',
                fontWeight: 500,
              }}
            >
              {t('theme.currentlyUsing')}
            </Box>
            <Chip
              icon={isDarkMode ? <DarkModeIcon /> : <LightModeIcon />}
              label={isDarkMode ? t('theme.dark') : t('theme.light')}
              size="small"
              variant="outlined"
              sx={{
                height: 22,
                fontSize: '0.7rem',
                fontWeight: 600,
                borderColor: isDarkMode ? 'var(--mediterranean-400)' : 'var(--terracotta-400)',
                color: isDarkMode ? 'var(--mediterranean-500)' : 'var(--terracotta-500)',
                '& .MuiChip-icon': {
                  fontSize: '0.8rem',
                },
              }}
            />
          </Box>
        </Box>
      </Menu>
    </Box>
  );
}