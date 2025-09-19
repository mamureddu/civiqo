'use client';

import React from 'react';
import AuthProvider from './AuthProvider';
import I18nProvider from './I18nProvider';
import { ThemeProvider } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';
import theme from '@/theme/theme';
import { CommunityProvider } from '@/contexts/CommunityContext';

interface ProvidersProps {
  children: React.ReactNode;
}

export default function Providers({ children }: ProvidersProps) {
  return (
    <I18nProvider>
      <AuthProvider>
        <CommunityProvider>
          <ThemeProvider theme={theme}>
            <CssBaseline />
            {children}
          </ThemeProvider>
        </CommunityProvider>
      </AuthProvider>
    </I18nProvider>
  );
}