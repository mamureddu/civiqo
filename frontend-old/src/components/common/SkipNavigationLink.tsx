'use client';

import React from 'react';
import { Box, Link } from '@mui/material';
import { useTranslation } from 'react-i18next';

export default function SkipNavigationLink() {
  const { t } = useTranslation('common');

  const handleSkipToMain = (event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    const mainContent = document.getElementById('main-content');
    if (mainContent) {
      mainContent.focus();
      mainContent.scrollIntoView();
    }
  };

  return (
    <Box
      sx={{
        position: 'absolute',
        top: -40,
        left: 6,
        zIndex: 9999,
        backgroundColor: 'primary.main',
        color: 'primary.contrastText',
        padding: '8px 16px',
        textDecoration: 'none',
        borderRadius: '0 0 4px 4px',
        fontSize: '14px',
        fontWeight: 'bold',
        transition: 'top 0.3s',
        '&:focus': {
          top: 0,
        },
      }}
    >
      <Link
        href="#main-content"
        onClick={handleSkipToMain}
        sx={{
          color: 'inherit',
          textDecoration: 'none',
          '&:hover': {
            textDecoration: 'underline',
          },
        }}
      >
        {t('navigation.skipToMain')}
      </Link>
    </Box>
  );
}