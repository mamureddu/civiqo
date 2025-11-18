'use client';

import React from 'react';
import {
  Box,
  Typography,
  Button,
  Alert,
  Stack,
  Card,
  CardContent,
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  ErrorOutline as ErrorIcon,
} from '@mui/icons-material';
import { useTranslation } from 'react-i18next';

interface ErrorRetryProps {
  onRetry: () => void;
  title?: string;
  message?: string;
  showCard?: boolean;
  severity?: 'error' | 'warning' | 'info';
}

export default function ErrorRetry({
  onRetry,
  title,
  message,
  showCard = true,
  severity = 'error'
}: ErrorRetryProps) {
  const { t } = useTranslation('common');

  const content = (
    <Stack spacing={2} alignItems="center" textAlign="center">
      <ErrorIcon sx={{ fontSize: 48, color: `${severity}.main` }} />

      <Typography variant="h6" fontWeight="bold">
        {title || t('errors.loadingFailed')}
      </Typography>

      <Typography variant="body2" color="text.secondary">
        {message || t('errors.retryMessage')}
      </Typography>

      <Button
        variant="contained"
        startIcon={<RefreshIcon />}
        onClick={onRetry}
        sx={{ mt: 2 }}
      >
        {t('actions.retry')}
      </Button>
    </Stack>
  );

  if (showCard) {
    return (
      <Card>
        <CardContent sx={{ p: 4 }}>
          {content}
        </CardContent>
      </Card>
    );
  }

  return (
    <Box sx={{ p: 4 }}>
      {content}
    </Box>
  );
}

// Compact inline error component
export function InlineError({
  message,
  onRetry,
  showRetry = true
}: {
  message?: string;
  onRetry?: () => void;
  showRetry?: boolean;
}) {
  const { t } = useTranslation('common');

  return (
    <Alert
      severity="error"
      action={
        showRetry && onRetry ? (
          <Button
            color="inherit"
            size="small"
            onClick={onRetry}
            startIcon={<RefreshIcon />}
          >
            {t('actions.retry')}
          </Button>
        ) : undefined
      }
    >
      {message || t('errors.generic')}
    </Alert>
  );
}

// Empty state with retry
export function EmptyStateWithRetry({
  title,
  message,
  onRetry,
  showRetry = false
}: {
  title: string;
  message: string;
  onRetry?: () => void;
  showRetry?: boolean;
}) {
  const { t } = useTranslation('common');

  return (
    <Box textAlign="center" py={8}>
      <Typography variant="h6" color="text.secondary" gutterBottom>
        {title}
      </Typography>
      <Typography variant="body2" color="text.secondary" mb={3}>
        {message}
      </Typography>
      {showRetry && onRetry && (
        <Button
          variant="outlined"
          onClick={onRetry}
          startIcon={<RefreshIcon />}
        >
          {t('actions.retry')}
        </Button>
      )}
    </Box>
  );
}