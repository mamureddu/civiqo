'use client';

import React, { useState, useEffect } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Avatar,
  Chip,
  Stack,
  Divider,
  Button,
  IconButton,
  Grid,
  Paper,
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
  Groups as GroupsIcon,
} from '@mui/icons-material';
import { format } from 'date-fns';
import { it, enUS } from 'date-fns/locale';
import Link from 'next/link';
import { useCommunity } from '@/contexts/CommunityContext';
import { useLocale } from '@/contexts/LocaleContext';

interface FeedItem {
  id: string;
  type: 'business_post' | 'community_announcement' | 'new_member' | 'poll' | 'event' | 'chat_activity';
  title: string;
  content: string;
  author: {
    id: string;
    name: string;
    avatar?: string;
    type: 'business' | 'member' | 'admin';
  };
  timestamp: string;
  metadata?: {
    business_id?: string;
    event_date?: string;
    poll_id?: string;
    member_count?: number;
    engagement_count?: number;
  };
  image_url?: string;
  actions?: {
    canLike: boolean;
    canShare: boolean;
    canComment: boolean;
  };
}

interface CommunityFeedProps {
  communityId: string;
}

// Mock feed data
const mockFeedItems: FeedItem[] = [
  {
    id: '1',
    type: 'business_post',
    title: 'Sconto Speciale del 20% sui Pranzi',
    content: 'Milano Coffee Roasters offre uno sconto del 20% su tutti i pranzi speciali questa settimana! Vieni a provare i nostri nuovi panini artigianali.',
    author: {
      id: 'business-1',
      name: 'Milano Coffee Roasters',
      avatar: 'https://images.unsplash.com/photo-1447933601403-0c6688de566e?w=100',
      type: 'business'
    },
    timestamp: '2024-01-15T10:30:00Z',
    metadata: {
      business_id: 'business-1',
      engagement_count: 12
    },
    image_url: 'https://images.unsplash.com/photo-1565299624946-b28f40a0ca4b?w=400',
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '2',
    type: 'community_announcement',
    title: 'Nuove Linee Guida della Comunità',
    content: 'Abbiamo aggiornato le linee guida della comunità per garantire un ambiente più sicuro e inclusivo per tutti i membri. Vi preghiamo di leggerle.',
    author: {
      id: 'admin-1',
      name: 'Amministrazione Comunità',
      type: 'admin'
    },
    timestamp: '2024-01-15T09:15:00Z',
    metadata: {
      engagement_count: 45
    },
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '3',
    type: 'new_member',
    title: 'Nuovi Membri si Sono Uniti',
    content: '5 nuovi membri si sono uniti alla nostra comunità oggi! Diamo loro il benvenuto: Marco R., Sara L., Giuseppe M., Anna C., e Luca P.',
    author: {
      id: 'system',
      name: 'Sistema Comunità',
      type: 'admin'
    },
    timestamp: '2024-01-15T08:45:00Z',
    metadata: {
      member_count: 5
    },
    actions: {
      canLike: true,
      canShare: false,
      canComment: true
    }
  },
  {
    id: '4',
    type: 'event',
    title: 'Riunione Mensile della Comunità',
    content: 'Non perdete la riunione mensile di domani sera alle 19:00 presso il Centro Civico. Discuteremo i prossimi progetti per il miglioramento del quartiere.',
    author: {
      id: 'admin-1',
      name: 'Amministrazione Comunità',
      type: 'admin'
    },
    timestamp: '2024-01-14T16:20:00Z',
    metadata: {
      event_date: '2024-01-16T19:00:00Z',
      engagement_count: 28
    },
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '5',
    type: 'poll',
    title: 'Votazione: Nuovo Parco della Comunità',
    content: 'È tempo di votare per la proposta del nuovo parco nel centro del quartiere. La vostra opinione è importante per prendere questa decisione insieme.',
    author: {
      id: 'admin-1',
      name: 'Amministrazione Comunità',
      type: 'admin'
    },
    timestamp: '2024-01-14T14:10:00Z',
    metadata: {
      poll_id: 'poll-1',
      engagement_count: 67
    },
    actions: {
      canLike: false,
      canShare: true,
      canComment: true
    }
  }
];

export default function CommunityFeed({ communityId }: CommunityFeedProps) {
  const { activeCommunity } = useCommunity();
  const { t, locale } = useLocale();
  const [feedItems, setFeedItems] = useState<FeedItem[]>([]);
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

  const getItemIcon = (type: FeedItem['type']) => {
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

  const getItemChipColor = (type: FeedItem['type']) => {
    switch (type) {
      case 'business_post':
        return 'primary';
      case 'community_announcement':
        return 'warning';
      case 'new_member':
        return 'success';
      case 'poll':
        return 'secondary';
      case 'event':
        return 'info';
      case 'chat_activity':
        return 'default';
      default:
        return 'default';
    }
  };

  const getItemTypeLabel = (type: FeedItem['type']) => {
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
    const dateLocale = locale === 'it' ? it : enUS;
    return format(date, 'PPp', { locale: dateLocale });
  };

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
              {activeCommunity.name.charAt(0).toUpperCase()}
            </Avatar>
            <Box flex={1}>
              <Typography variant="h5" fontWeight="bold">
                {activeCommunity.name}
              </Typography>
              <Typography variant="body2" color="text.secondary" mb={1}>
                {activeCommunity.description}
              </Typography>
              <Stack direction="row" spacing={2} alignItems="center">
                <Chip
                  icon={<GroupsIcon />}
                  label={`${activeCommunity.member_count.toLocaleString()} ${t('pages.communities.members')}`}
                  size="small"
                  variant="outlined"
                />
                {activeCommunity.subscription_status && activeCommunity.subscription_status !== 'free' && (
                  <Chip
                    label={t(`subscription.community.tiers.${activeCommunity.subscription_status}.name`)}
                    size="small"
                    color="primary"
                  />
                )}
              </Stack>
            </Box>
            <Button
              component={Link}
              href={`/communities/${activeCommunity.id}/subscription`}
              variant="outlined"
              size="small"
            >
              {t('subscription.billing.manageSuscription')}
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Feed Items */}
      <Stack spacing={3}>
        {feedItems.map((item) => (
          <Card key={item.id}>
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
                      {item.author.name}
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
                {item.title}
              </Typography>
              <Typography variant="body2" color="text.secondary" paragraph>
                {item.content}
              </Typography>

              {/* Item Image */}
              {item.image_url && (
                <Box
                  component="img"
                  src={item.image_url}
                  alt={item.title}
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
                        <strong>Data Evento:</strong> {formatTimestamp(item.metadata.event_date)}
                      </Typography>
                    </Alert>
                  )}
                  {item.metadata.member_count && (
                    <Typography variant="body2" color="text.secondary">
                      {item.metadata.member_count} nuovi membri
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
                    Mi Piace
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
                    Commenta
                  </Button>
                )}
                {item.metadata?.engagement_count && (
                  <Box ml="auto">
                    <Stack direction="row" spacing={0.5} alignItems="center">
                      <TrendingUpIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                      <Typography variant="caption" color="text.secondary">
                        {item.metadata.engagement_count} interazioni
                      </Typography>
                    </Stack>
                  </Box>
                )}
              </Stack>
            </CardContent>
          </Card>
        ))}
      </Stack>

      {feedItems.length === 0 && (
        <Box textAlign="center" py={8}>
          <AnnouncementIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            Nessuna Attività Recente
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Quando ci sarà nuova attività nella tua comunità, apparirà qui.
          </Typography>
        </Box>
      )}
    </Box>
  );
}