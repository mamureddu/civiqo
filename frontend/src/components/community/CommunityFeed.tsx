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

interface CommunityFeedProps {
  communityId: string;
}

export default function CommunityFeed({ communityId }: CommunityFeedProps) {
  const { activeCommunity } = useCommunity();
  const { t } = useTranslation('common');
  const [feedItems, setFeedItems] = useState(mockFeedItems);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate loading feed items
    const loadFeed = async () => {
      setLoading(true);
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      setFeedItems(mockFeedItems);
      setLoading(false);
    };

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
      <Box display="flex" justifyContent="center" py={4}>
        <Typography variant="body2" color="text.secondary">
          {t('actions.loading')}
        </Typography>
      </Box>
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