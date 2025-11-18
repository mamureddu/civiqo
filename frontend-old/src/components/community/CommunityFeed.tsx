'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Stack,
  Typography,
  Button,
} from '@mui/material';
import {
  Groups as GroupsIcon,
  Announcement as AnnouncementIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import { useTranslation } from 'react-i18next';
import { useCommunity } from '@/contexts/CommunityContext';
import { mockFeedItems } from '@/data/mockFeedData';
import FeedItem from './feed/FeedItem';
import CommunityHeader from './feed/CommunityHeader';
import { FeedItemSkeleton, HeaderSkeleton } from '@/components/common/SkeletonLoaders';
import ErrorRetry from '@/components/common/ErrorRetry';

interface CommunityFeedProps {
  communityId: string;
}

export default function CommunityFeed({ communityId }: CommunityFeedProps) {
  const { activeCommunity } = useCommunity();
  const { t } = useTranslation('common');
  const [feedItems, setFeedItems] = useState(mockFeedItems);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadFeed = async () => {
    try {
      setLoading(true);
      setError(null);
      // Simulate API call with potential error
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Simulate random error for testing
      if (Math.random() < 0.1) {
        throw new Error('Failed to load feed');
      }

      setFeedItems(mockFeedItems);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (communityId) {
      loadFeed();
    }
  }, [communityId]);

  if (!activeCommunity) {
    return (
      <Box textAlign="center" py={8}>
        <GroupsIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
        <Typography variant="h6" color="text.secondary" gutterBottom>
          {t('pages.dashboard.noCommunity')}
        </Typography>
        <Typography variant="body2" color="text.secondary" mb={3}>
          {t('pages.dashboard.selectCommunity')}
        </Typography>
        <Button
          component={Link}
          href="/communities"
          variant="contained"
        >
          {t('pages.communities.title')}
        </Button>
      </Box>
    );
  }

  if (loading) {
    return (
      <Box>
        <HeaderSkeleton />
        <Stack spacing={3}>
          {Array.from({ length: 3 }).map((_, index) => (
            <FeedItemSkeleton key={index} />
          ))}
        </Stack>
      </Box>
    );
  }

  if (error) {
    return (
      <ErrorRetry
        onRetry={loadFeed}
        title={t('errors.loadingFailed')}
        message={error}
      />
    );
  }

  return (
    <Box>
      {/* Community Header */}
      <CommunityHeader community={activeCommunity} />

      {/* Feed Items */}
      <Stack spacing={3}>
        {feedItems.map((item) => (
          <FeedItem key={item.id} item={item} />
        ))}
      </Stack>

      {feedItems.length === 0 && (
        <Box textAlign="center" py={8}>
          <AnnouncementIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            {t('community.feed.empty.title')}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {t('community.feed.empty.description')}
          </Typography>
        </Box>
      )}
    </Box>
  );
}