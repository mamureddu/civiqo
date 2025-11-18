'use client';

import React from 'react';
import {
  Card,
  CardContent,
  Typography,
  Avatar,
  Chip,
  Stack,
  Button,
  Box,
} from '@mui/material';
import {
  Groups as GroupsIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import { useTranslation } from 'react-i18next';
import { Community } from '@/contexts/CommunityContext';

interface CommunityHeaderProps {
  community: Community;
}

export default function CommunityHeader({ community }: CommunityHeaderProps) {
  const { t } = useTranslation('common');

  return (
    <Card sx={{ mb: 3 }}>
      <CardContent>
        <Box display="flex" alignItems="center" gap={2} mb={2}>
          <Avatar
            sx={{
              width: 60,
              height: 60,
              bgcolor: 'primary.main',
              fontSize: '1.5rem',
              fontWeight: 'bold'
            }}
          >
            {community.name.charAt(0).toUpperCase()}
          </Avatar>
          <Box flex={1}>
            <Typography variant="h5" fontWeight="bold">
              {community.name}
            </Typography>
            <Typography variant="body2" color="text.secondary" mb={1}>
              {community.description}
            </Typography>
            <Stack direction="row" spacing={2} alignItems="center">
              <Chip
                icon={<GroupsIcon />}
                label={`${community.member_count.toLocaleString()} ${t('pages.communities.members')}`}
                size="small"
                variant="outlined"
              />
              {community.subscription_status && community.subscription_status !== 'free' && (
                <Chip
                  label={t(`subscription.community.tiers.${community.subscription_status}.name`)}
                  size="small"
                  color="primary"
                />
              )}
            </Stack>
          </Box>
          <Button
            component={Link}
            href={`/communities/${community.id}/subscription`}
            variant="outlined"
            size="small"
          >
            {t('subscription.billing.manageSuscription')}
          </Button>
        </Box>
      </CardContent>
    </Card>
  );
}