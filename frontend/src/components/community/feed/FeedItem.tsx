'use client';

import React from 'react';
import {
  Card,
  CardContent,
  Typography,
  Avatar,
  Chip,
  Stack,
  Divider,
  Button,
  IconButton,
  Box,
  Alert,
} from '@mui/material';
import {
  Business as BusinessIcon,
  Event as EventIcon,
  Announcement as AnnouncementIcon,
  Person as PersonIcon,
  HowToVote as PollIcon,
  Chat as ChatIcon,
  Favorite as FavoriteIcon,
  Share as ShareIcon,
  MoreVert as MoreIcon,
  TrendingUp as TrendingUpIcon,
} from '@mui/icons-material';
import { format } from 'date-fns';
import { it, enUS } from 'date-fns/locale';
import { useTranslation } from 'react-i18next';
import { FeedItem as FeedItemType } from '@/data/mockFeedData';

interface FeedItemProps {
  item: FeedItemType;
}

export default function FeedItem({ item }: FeedItemProps) {
  const { t, i18n } = useTranslation('common');

  const getItemIcon = (type: FeedItemType['type']) => {
    switch (type) {
      case 'business_post':
        return <BusinessIcon />;
      case 'community_announcement':
        return <AnnouncementIcon />;
      case 'new_member':
        return <PersonIcon />;
      case 'poll':
        return <PollIcon />;
      case 'event':
        return <EventIcon />;
      case 'chat_activity':
        return <ChatIcon />;
      default:
        return <AnnouncementIcon />;
    }
  };

  const getItemChipColor = (type: FeedItemType['type']) => {
    switch (type) {
      case 'business_post':
        return 'primary' as const;
      case 'community_announcement':
        return 'warning' as const;
      case 'new_member':
        return 'success' as const;
      case 'poll':
        return 'secondary' as const;
      case 'event':
        return 'info' as const;
      case 'chat_activity':
        return 'default' as const;
      default:
        return 'default' as const;
    }
  };

  const getItemTypeLabel = (type: FeedItemType['type']) => {
    switch (type) {
      case 'business_post':
        return t('common.business');
      case 'community_announcement':
        return t('common.announcement');
      case 'new_member':
        return t('common.newMember');
      case 'poll':
        return t('common.poll');
      case 'event':
        return t('common.event');
      case 'chat_activity':
        return t('common.chat');
      default:
        return t('common.activity');
    }
  };

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    const dateLocale = i18n.language === 'it' ? it : enUS;
    return format(date, 'PPp', { locale: dateLocale });
  };

  const getTranslatedTitle = (title: string) => {
    // Check if title is a translation key
    if (title.includes('.')) {
      return t(title);
    }
    return title;
  };

  const getTranslatedContent = (content: string) => {
    // Check if content is a translation key
    if (content.includes('.')) {
      return t(content, { count: item.metadata?.member_count || 5 });
    }
    return content;
  };

  const getTranslatedAuthor = (authorName: string) => {
    // Check if author name is a translation key
    if (authorName.includes('.')) {
      return t(authorName);
    }
    return authorName;
  };

  return (
    <Card>
      <CardContent>
        {/* Item Header */}
        <Box display="flex" alignItems="center" gap={2} mb={2}>
          <Avatar
            src={item.author.avatar}
            sx={{ width: 40, height: 40 }}
          >
            {getItemIcon(item.type)}
          </Avatar>
          <Box flex={1}>
            <Stack direction="row" spacing={1} alignItems="center" mb={0.5}>
              <Typography variant="subtitle2" fontWeight="bold">
                {getTranslatedAuthor(item.author.name)}
              </Typography>
              <Chip
                label={getItemTypeLabel(item.type)}
                size="small"
                color={getItemChipColor(item.type)}
                variant="outlined"
              />
            </Stack>
            <Typography variant="caption" color="text.secondary">
              {formatTimestamp(item.timestamp)}
            </Typography>
          </Box>
          <IconButton size="small">
            <MoreIcon />
          </IconButton>
        </Box>

        {/* Item Content */}
        <Typography variant="h6" fontWeight="bold" gutterBottom>
          {getTranslatedTitle(item.title)}
        </Typography>
        <Typography variant="body2" color="text.secondary" paragraph>
          {getTranslatedContent(item.content)}
        </Typography>

        {/* Item Image */}
        {item.image_url && (
          <Box
            component="img"
            src={item.image_url}
            alt={getTranslatedTitle(item.title)}
            sx={{
              width: '100%',
              maxHeight: 300,
              objectFit: 'cover',
              borderRadius: 1,
              mb: 2
            }}
          />
        )}

        {/* Item Metadata */}
        {item.metadata && (
          <Box mb={2}>
            {item.metadata.event_date && (
              <Alert severity="info" sx={{ mb: 1 }}>
                <Typography variant="body2">
                  <strong>{t('community.feed.actions.eventDate')}:</strong> {formatTimestamp(item.metadata.event_date)}
                </Typography>
              </Alert>
            )}
            {item.metadata.member_count && (
              <Typography variant="body2" color="text.secondary">
                {t('community.feed.actions.newMembers', { count: item.metadata.member_count })}
              </Typography>
            )}
          </Box>
        )}

        <Divider sx={{ my: 2 }} />

        {/* Item Actions */}
        <Stack direction="row" spacing={2} alignItems="center">
          {item.actions?.canLike && (
            <Button
              startIcon={<FavoriteIcon />}
              size="small"
              variant="text"
            >
              {t('community.feed.actions.like')}
            </Button>
          )}
          {item.actions?.canShare && (
            <Button
              startIcon={<ShareIcon />}
              size="small"
              variant="text"
            >
              {t('actions.share')}
            </Button>
          )}
          {item.actions?.canComment && (
            <Button
              startIcon={<ChatIcon />}
              size="small"
              variant="text"
            >
              {t('community.feed.actions.comment')}
            </Button>
          )}
          {item.metadata?.engagement_count && (
            <Box ml="auto">
              <Stack direction="row" spacing={0.5} alignItems="center">
                <TrendingUpIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                <Typography variant="caption" color="text.secondary">
                  {t('community.feed.actions.interactions', { count: item.metadata.engagement_count })}
                </Typography>
              </Stack>
            </Box>
          )}
        </Stack>
      </CardContent>
    </Card>
  );
}