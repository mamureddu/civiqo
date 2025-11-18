'use client';

import React from 'react';
import AuthProvider from './AuthProvider';
import I18nProvider from './I18nProvider';
import ThemeContextProvider from '@/contexts/ThemeContext';
import { CommunityProvider } from '@/contexts/CommunityContext';
import ErrorBoundary from '@/components/common/ErrorBoundary';

interface ProvidersProps {
  children: React.ReactNode;
}

export default function Providers({ children }: ProvidersProps) {
  return (
    <ErrorBoundary showDetails={process.env.NODE_ENV === 'development'}>
      <I18nProvider>
        <AuthProvider>
          <CommunityProvider>
            <ThemeContextProvider>
              {children}
            </ThemeContextProvider>
          </CommunityProvider>
        </AuthProvider>
      </I18nProvider>
    </ErrorBoundary>
  );
}