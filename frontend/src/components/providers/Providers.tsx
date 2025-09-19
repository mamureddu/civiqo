'use client';

import React from 'react';
import AuthProvider from './AuthProvider';
import { ThemeProvider } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';
import theme from '@/theme/theme';
import { LocaleProvider } from '@/contexts/LocaleContext';
import { CommunityProvider } from '@/contexts/CommunityContext';

interface ProvidersProps {
  children: React.ReactNode;
}

export default function Providers({ children }: ProvidersProps) {
  return (
    <LocaleProvider>
      <AuthProvider>
        <CommunityProvider>
          <ThemeProvider theme={theme}>
            <CssBaseline />
            {children}
          </ThemeProvider>
        </CommunityProvider>
      </AuthProvider>
    </LocaleProvider>
  );
}