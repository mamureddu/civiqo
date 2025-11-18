'use client';

import React, { useState } from 'react';
import {
  Box,
  Button,
  Menu,
  MenuItem,
  Avatar,
  Typography,
  Chip,
  Stack,
  Divider,
  ListItemIcon,
  ListItemText,
} from '@mui/material';
import {
  ExpandMore as ExpandMoreIcon,
  Groups as GroupsIcon,
  Add as AddIcon,
  Check as CheckIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import { useCommunity, Community } from '@/contexts/CommunityContext';
import { useTranslation } from 'react-i18next';

interface CommunitySelectorProps {
  compact?: boolean;
}

export default function CommunitySelector({ compact = false }: CommunitySelectorProps) {
  const { activeCommunity, userCommunities, setActiveCommunity } = useCommunity();
  const { t } = useTranslation('common');
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleCommunitySelect = (community: Community) => {
    setActiveCommunity(community);
    handleClose();
  };

  if (compact) {
    return (
      <>
        <Button
          onClick={handleClick}
          endIcon={<ExpandMoreIcon />}
          sx={{
            textTransform: 'none',
            color: 'inherit',
            minWidth: 'auto',
          }}
        >
          {activeCommunity ? (
            <Box display="flex" alignItems="center" gap={1}>
              <Avatar
                sx={{
                  width: 24,
                  height: 24,
                  bgcolor: 'primary.main',
                  fontSize: '0.75rem',
                }}
              >
                {activeCommunity.name.charAt(0).toUpperCase()}
              </Avatar>
              <Typography variant="body2" noWrap sx={{ maxWidth: 120 }}>
                {activeCommunity.name}
              </Typography>
            </Box>
          ) : (
            <Typography variant="body2" color="text.secondary">
              {t('pages.dashboard.selectCommunity')}
            </Typography>
          )}
        </Button>

        <Menu
          anchorEl={anchorEl}
          open={open}
          onClose={handleClose}
          PaperProps={{
            sx: { minWidth: 280, maxWidth: 400 }
          }}
        >
          <Box sx={{ p: 2, pb: 1 }}>
            <Typography variant="subtitle2" fontWeight="bold" color="text.secondary">
              {t('pages.communities.title')}
            </Typography>
          </Box>

          {userCommunities.map((community) => (
            <MenuItem
              key={community.id}
              onClick={() => handleCommunitySelect(community)}
              sx={{ py: 1.5 }}
            >
              <ListItemIcon>
                <Avatar
                  sx={{
                    width: 32,
                    height: 32,
                    bgcolor: 'primary.main',
                  }}
                >
                  {community.name.charAt(0).toUpperCase()}
                </Avatar>
              </ListItemIcon>
              <ListItemText
                primary={community.name}
                secondary={`${community.member_count.toLocaleString()} ${t('pages.communities.members')}`}
              />
              {activeCommunity?.id === community.id && (
                <CheckIcon color="primary" sx={{ ml: 1 }} />
              )}
            </MenuItem>
          ))}

          {userCommunities.length === 0 && (
            <MenuItem disabled>
              <ListItemText
                primary={t('pages.communities.noCommunitiesTitle')}
                secondary={t('pages.communities.noCommunitiesText')}
              />
            </MenuItem>
          )}

          <Divider sx={{ my: 1 }} />

          <MenuItem component={Link} href="/communities" onClick={handleClose}>
            <ListItemIcon>
              <AddIcon />
            </ListItemIcon>
            <ListItemText primary={t('pages.communities.joinNew')} />
          </MenuItem>
        </Menu>
      </>
    );
  }

  return (
    <>
      <Button
        onClick={handleClick}
        endIcon={<ExpandMoreIcon />}
        variant="outlined"
        sx={{
          textTransform: 'none',
          justifyContent: 'space-between',
          minWidth: 200,
        }}
      >
        {activeCommunity ? (
          <Box display="flex" alignItems="center" gap={2}>
            <Avatar
              sx={{
                width: 32,
                height: 32,
                bgcolor: 'primary.main',
              }}
            >
              {activeCommunity.name.charAt(0).toUpperCase()}
            </Avatar>
            <Box>
              <Typography variant="body2" fontWeight="bold" textAlign="left">
                {activeCommunity.name}
              </Typography>
              <Typography variant="caption" color="text.secondary" textAlign="left">
                {activeCommunity.member_count.toLocaleString()} {t('pages.communities.members')}
              </Typography>
            </Box>
          </Box>
        ) : (
          <Box display="flex" alignItems="center" gap={2}>
            <GroupsIcon color="action" />
            <Typography variant="body2" color="text.secondary">
              {t('pages.dashboard.selectCommunity')}
            </Typography>
          </Box>
        )}
      </Button>

      <Menu
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        PaperProps={{
          sx: { minWidth: 350, maxWidth: 500 }
        }}
      >
        <Box sx={{ p: 2, pb: 1 }}>
          <Typography variant="h6" fontWeight="bold">
            {t('pages.communities.title')}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {t('pages.communities.subtitle')}
          </Typography>
        </Box>

        {userCommunities.map((community) => (
          <MenuItem
            key={community.id}
            onClick={() => handleCommunitySelect(community)}
            sx={{ py: 2, px: 2 }}
          >
            <Box display="flex" alignItems="center" gap={2} width="100%">
              <Avatar
                sx={{
                  width: 40,
                  height: 40,
                  bgcolor: 'primary.main',
                }}
              >
                {community.name.charAt(0).toUpperCase()}
              </Avatar>
              <Box flex={1}>
                <Typography variant="subtitle2" fontWeight="bold">
                  {community.name}
                </Typography>
                <Typography variant="caption" color="text.secondary" display="block">
                  {community.description}
                </Typography>
                <Stack direction="row" spacing={1} alignItems="center" mt={0.5}>
                  <Typography variant="caption" color="text.secondary">
                    {community.member_count.toLocaleString()} {t('pages.communities.members')}
                  </Typography>
                  {community.subscription_status && community.subscription_status !== 'free' && (
                    <Chip
                      label={t(`subscription.community.tiers.${community.subscription_status}.name`)}
                      size="small"
                      color="primary"
                      variant="outlined"
                    />
                  )}
                </Stack>
              </Box>
              {activeCommunity?.id === community.id && (
                <CheckIcon color="primary" />
              )}
            </Box>
          </MenuItem>
        ))}

        {userCommunities.length === 0 && (
          <Box sx={{ p: 3, textAlign: 'center' }}>
            <GroupsIcon sx={{ fontSize: 48, color: 'text.secondary', mb: 1 }} />
            <Typography variant="body2" color="text.secondary" gutterBottom>
              {t('pages.communities.noCommunitiesText')}
            </Typography>
          </Box>
        )}

        <Divider sx={{ my: 1 }} />

        <MenuItem component={Link} href="/communities" onClick={handleClose} sx={{ py: 1.5 }}>
          <ListItemIcon>
            <AddIcon color="primary" />
          </ListItemIcon>
          <ListItemText
            primary={t('pages.communities.joinNew')}
            primaryTypographyProps={{ color: 'primary' }}
          />
        </MenuItem>
      </Menu>
    </>
  );
}