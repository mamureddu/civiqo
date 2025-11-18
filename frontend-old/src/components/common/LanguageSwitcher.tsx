'use client';

import React, { useState } from 'react';
import {
  Button,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
  Typography,
} from '@mui/material';
import {
  Language as LanguageIcon,
  ExpandMore as ExpandMoreIcon,
} from '@mui/icons-material';
import { useTranslation } from 'react-i18next';

type Locale = 'it' | 'en';

interface LanguageSwitcherProps {
  compact?: boolean;
}

const languages = {
  it: {
    name: 'Italiano',
    flag: '🇮🇹',
  },
  en: {
    name: 'English',
    flag: '🇺🇸',
  },
};

export default function LanguageSwitcher({ compact = false }: LanguageSwitcherProps) {
  const { i18n } = useTranslation();
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleLanguageSelect = async (newLocale: Locale) => {
    try {
      await i18n.changeLanguage(newLocale);
      // Save to localStorage with SSR safety
      if (typeof window !== 'undefined') {
        localStorage.setItem('i18nextLng', newLocale);
      }
    } catch (error) {
      console.error('Failed to change language:', error);
    }
    handleClose();
  };

  const currentLocale = i18n.language as Locale;
  const currentLanguage = languages[currentLocale] || languages.it;

  if (compact) {
    return (
      <>
        <Button
          onClick={handleClick}
          endIcon={<ExpandMoreIcon />}
          sx={{
            textTransform: 'none',
            color: 'inherit',
            minWidth: 'auto',
          }}
        >
          <Typography variant="body2" sx={{ mr: 0.5 }}>
            {currentLanguage.flag}
          </Typography>
          <Typography variant="body2" sx={{ display: { xs: 'none', sm: 'block' } }}>
            {currentLanguage.name}
          </Typography>
        </Button>

        <Menu
          anchorEl={anchorEl}
          open={open}
          onClose={handleClose}
          PaperProps={{
            sx: { minWidth: 150 }
          }}
        >
          {Object.entries(languages).map(([code, lang]) => (
            <MenuItem
              key={code}
              onClick={() => handleLanguageSelect(code as Locale)}
              selected={i18n.language === code}
            >
              <ListItemIcon>
                <Typography variant="body1">{lang.flag}</Typography>
              </ListItemIcon>
              <ListItemText primary={lang.name} />
            </MenuItem>
          ))}
        </Menu>
      </>
    );
  }

  return (
    <>
      <Button
        onClick={handleClick}
        startIcon={<LanguageIcon />}
        endIcon={<ExpandMoreIcon />}
        variant="outlined"
        sx={{
          textTransform: 'none',
          minWidth: 120,
        }}
      >
        <Typography variant="body2" sx={{ mr: 1 }}>
          {currentLanguage.flag}
        </Typography>
        {currentLanguage.name}
      </Button>

      <Menu
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        PaperProps={{
          sx: { minWidth: 180 }
        }}
      >
        {Object.entries(languages).map(([code, lang]) => (
          <MenuItem
            key={code}
            onClick={() => handleLanguageSelect(code as Locale)}
            selected={i18n.language === code}
          >
            <ListItemIcon>
              <Typography variant="h6">{lang.flag}</Typography>
            </ListItemIcon>
            <ListItemText
              primary={lang.name}
              secondary={code === i18n.language ? 'Current' : undefined}
            />
          </MenuItem>
        ))}
      </Menu>
    </>
  );
}