'use client';

import React from 'react';
import {
  Box,
  Card,
  CardContent,
  Skeleton,
  Stack,
  Avatar,
} from '@mui/material';

// Feed Item Skeleton
export function FeedItemSkeleton() {
  return (
    <Card>
      <CardContent>
        <Stack direction="row" spacing={2} alignItems="center" mb={2}>
          <Skeleton variant="circular">
            <Avatar />
          </Skeleton>
          <Stack spacing={1} sx={{ flex: 1 }}>
            <Skeleton variant="text" width="40%" height={20} />
            <Skeleton variant="text" width="25%" height={16} />
          </Stack>
        </Stack>

        <Skeleton variant="text" width="90%" height={24} sx={{ mb: 1 }} />
        <Skeleton variant="text" width="75%" height={20} sx={{ mb: 2 }} />

        <Stack direction="row" spacing={2} alignItems="center">
          <Skeleton variant="rounded" width={80} height={32} />
          <Skeleton variant="rounded" width={100} height={32} />
          <Skeleton variant="text" width="20%" height={16} />
        </Stack>
      </CardContent>
    </Card>
  );
}

// Community Card Skeleton
export function CommunityCardSkeleton() {
  return (
    <Card>
      <CardContent>
        <Stack direction="row" spacing={2} alignItems="center" mb={2}>
          <Skeleton variant="circular" width={56} height={56} />
          <Stack spacing={1} sx={{ flex: 1 }}>
            <Skeleton variant="text" width="60%" height={24} />
            <Skeleton variant="text" width="40%" height={16} />
          </Stack>
        </Stack>

        <Skeleton variant="text" width="100%" height={20} sx={{ mb: 1 }} />
        <Skeleton variant="text" width="80%" height={20} sx={{ mb: 2 }} />

        <Stack direction="row" spacing={1} alignItems="center" mb={2}>
          <Skeleton variant="rectangular" width={16} height={16} />
          <Skeleton variant="text" width="30%" height={16} />
        </Stack>

        <Skeleton variant="rounded" width="100%" height={36} />
      </CardContent>
    </Card>
  );
}

// Business Card Skeleton
export function BusinessCardSkeleton() {
  return (
    <Card>
      <CardContent>
        <Stack direction="row" spacing={2} alignItems="center" mb={2}>
          <Skeleton variant="rounded" width={48} height={48} />
          <Stack spacing={1} sx={{ flex: 1 }}>
            <Skeleton variant="text" width="70%" height={20} />
            <Skeleton variant="text" width="50%" height={16} />
          </Stack>
        </Stack>

        <Skeleton variant="text" width="100%" height={18} sx={{ mb: 1 }} />
        <Skeleton variant="text" width="90%" height={18} sx={{ mb: 2 }} />

        <Stack direction="row" spacing={2}>
          <Skeleton variant="rounded" width={100} height={32} />
          <Skeleton variant="rounded" width={120} height={32} />
        </Stack>
      </CardContent>
    </Card>
  );
}

// List Item Skeleton
export function ListItemSkeleton() {
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', py: 2, px: 2 }}>
      <Skeleton variant="circular" width={40} height={40} sx={{ mr: 2 }} />
      <Stack spacing={1} sx={{ flex: 1 }}>
        <Skeleton variant="text" width="60%" height={18} />
        <Skeleton variant="text" width="40%" height={14} />
      </Stack>
      <Skeleton variant="rounded" width={24} height={24} />
    </Box>
  );
}

// Table Row Skeleton
export function TableRowSkeleton({ columns = 4 }: { columns?: number }) {
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', py: 2, px: 2 }}>
      {Array.from({ length: columns }).map((_, index) => (
        <Box key={index} sx={{ flex: 1, mr: index < columns - 1 ? 2 : 0 }}>
          <Skeleton variant="text" width="80%" height={18} />
        </Box>
      ))}
    </Box>
  );
}

// Header Skeleton
export function HeaderSkeleton() {
  return (
    <Box sx={{ mb: 3 }}>
      <Skeleton variant="text" width="40%" height={40} sx={{ mb: 1 }} />
      <Skeleton variant="text" width="60%" height={24} />
    </Box>
  );
}

// Statistics Skeleton
export function StatisticsSkeleton() {
  return (
    <Stack direction="row" spacing={2}>
      {Array.from({ length: 4 }).map((_, index) => (
        <Card key={index} sx={{ flex: 1, textAlign: 'center' }}>
          <CardContent>
            <Skeleton variant="text" width="60%" height={32} sx={{ mx: 'auto', mb: 1 }} />
            <Skeleton variant="text" width="80%" height={18} sx={{ mx: 'auto' }} />
          </CardContent>
        </Card>
      ))}
    </Stack>
  );
}

// Generic Content Skeleton
export function ContentSkeleton({
  lines = 3,
  showHeader = true,
  showButton = false
}: {
  lines?: number;
  showHeader?: boolean;
  showButton?: boolean;
}) {
  return (
    <Box>
      {showHeader && (
        <Skeleton variant="text" width="50%" height={32} sx={{ mb: 2 }} />
      )}

      {Array.from({ length: lines }).map((_, index) => (
        <Skeleton
          key={index}
          variant="text"
          width={index === lines - 1 ? "70%" : "100%"}
          height={20}
          sx={{ mb: 1 }}
        />
      ))}

      {showButton && (
        <Skeleton variant="rounded" width={120} height={36} sx={{ mt: 2 }} />
      )}
    </Box>
  );
}