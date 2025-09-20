'use client';

import React, { Component, ErrorInfo, ReactNode } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  Stack,
  Alert,
  Chip,
} from '@mui/material';
import {
  ErrorOutline as ErrorIcon,
  Refresh as RefreshIcon,
  Home as HomeIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import { useTranslation } from 'react-i18next';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  showDetails?: boolean;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

// Translation hook wrapper component
function ErrorBoundaryContent({ error, errorInfo, showDetails, onReset, onReload }: {
  error: Error | null;
  errorInfo: ErrorInfo | null;
  showDetails?: boolean;
  onReset: () => void;
  onReload: () => void;
}) {
  const { t } = useTranslation('common');

  return (
    <Box
      sx={{
        minHeight: '50vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        p: 3,
      }}
    >
      <Card sx={{ maxWidth: 600, width: '100%' }}>
        <CardContent sx={{ textAlign: 'center', p: 4 }}>
          <ErrorIcon
            sx={{
              fontSize: 64,
              color: 'error.main',
              mb: 2,
            }}
          />

          <Typography variant="h5" fontWeight="bold" gutterBottom>
            {t('errors.somethingWentWrong')}
          </Typography>

          <Typography variant="body1" color="text.secondary" sx={{ mb: 3 }}>
            {t('errors.unexpectedError')}
          </Typography>

          <Stack direction="row" spacing={2} justifyContent="center" sx={{ mb: 3 }}>
            <Button
              variant="contained"
              startIcon={<RefreshIcon />}
              onClick={onReload}
            >
              {t('actions.refreshPage')}
            </Button>

            <Button
              variant="outlined"
              startIcon={<HomeIcon />}
              component={Link}
              href="/"
            >
              {t('actions.goHome')}
            </Button>
          </Stack>

          {showDetails && error && (
            <Alert severity="error" sx={{ textAlign: 'left', mt: 2 }}>
              <Typography variant="subtitle2" gutterBottom>
                {t('errors.errorDetails')}:
              </Typography>
              <Typography variant="body2" sx={{ fontFamily: 'monospace', fontSize: '0.8rem' }}>
                {error.toString()}
              </Typography>
              {errorInfo && (
                <Typography variant="body2" sx={{ fontFamily: 'monospace', fontSize: '0.7rem', mt: 1 }}>
                  {errorInfo.componentStack}
                </Typography>
              )}
            </Alert>
          )}

          {process.env.NODE_ENV === 'development' && (
            <Chip
              label={t('common.developmentMode')}
              size="small"
              color="warning"
              sx={{ mt: 2 }}
            />
          )}
        </CardContent>
      </Card>
    </Box>
  );
}


class ErrorBoundary extends Component<Props, ErrorBoundaryState> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return {
      hasError: true,
      error,
      errorInfo: null,
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    this.setState({
      error,
      errorInfo,
    });

    // Log to error reporting service if available
    if (process.env.NODE_ENV === 'production') {
      // Example: reportError(error, errorInfo);
    }
  }

  handleReset = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  };

  handleReload = () => {
    window.location.reload();
  };

  render() {
    if (this.state.hasError) {
      // Custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default error UI with translations
      return (
        <ErrorBoundaryContent
          error={this.state.error}
          errorInfo={this.state.errorInfo}
          showDetails={this.props.showDetails}
          onReset={this.handleReset}
          onReload={this.handleReload}
        />
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;