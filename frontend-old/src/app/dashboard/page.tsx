'use client';

import { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  Stack,
  Paper,
  Avatar,
  Divider,
  CircularProgress,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  IconButton,
} from '@mui/material';
import {
  Groups as GroupsIcon,
  Business as BusinessIcon,
  Museum as MuseumIcon,
  HowToVote as GovernanceIcon,
  Chat as ChatIcon,
  Add as AddIcon,
  TrendingUp as TrendingUpIcon,
  Event as EventIcon,
  Announcement as AnnouncementIcon,
  Notifications as NotificationsIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import CommunityFeed from '@/components/community/CommunityFeed';
import CommunitySelector from '@/components/community/CommunitySelector';
import { useSession } from 'next-auth/react';
import { useCommunity } from '@/contexts/CommunityContext';
import { useTranslation } from 'react-i18next';
import { mockUserCommunities } from '@/data/mockCommunities';
import { mockStats, mockRecentActivity } from '@/data/mockFeedData';


export default function Dashboard() {
  const { data: session } = useSession();
  const user = session?.user;
  const { activeCommunity, userCommunities, setUserCommunities } = useCommunity();
  const { t } = useTranslation('common');
  const [loading, setLoading] = useState(false);
  const [communities, setCommunities] = useState<any[]>([]);

  // Load user communities - in real app, this would come from API
  useEffect(() => {
    setUserCommunities(mockUserCommunities);
    setCommunities(mockUserCommunities);

    // Set first community as active if none selected
    if (!activeCommunity && mockUserCommunities.length > 0) {
      setTimeout(() => {
        // Small delay to avoid hydration issues
      }, 100);
    }
  }, [setUserCommunities, activeCommunity]);

  const StatCard = ({ title, value, icon, color, action }: {
    title: string;
    value: number;
    icon: React.ReactNode;
    color: string;
    action?: () => void;
  }) => (
    <Card sx={{ height: '100%', cursor: action ? 'pointer' : 'default' }} onClick={action}>
      <CardContent>
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Box>
            <Typography color="textSecondary" gutterBottom variant="h6">
              {title}
            </Typography>
            <Typography variant="h4" component="h2" fontWeight="bold">
              {value}
            </Typography>
          </Box>
          <Box sx={{ color }}>
            {icon}
          </Box>
        </Box>
      </CardContent>
    </Card>
  );

  // If no active community, show community selection
  if (!activeCommunity) {
    return (
      <DashboardLayout>
        <Box>
          {/* Welcome Section */}
          <Box mb={4} textAlign="center">
            <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
              {t('pages.dashboard.welcome')}, {user?.name || user?.email}!
            </Typography>
            <Typography variant="body1" color="text.secondary" mb={3}>
              {t('pages.dashboard.selectCommunityDescription')}
            </Typography>
            <CommunitySelector />
          </Box>

          {/* My Communities List */}
          {userCommunities.length > 0 && (
            <Card sx={{ maxWidth: 800, mx: 'auto' }}>
              <CardContent>
                <Typography variant="h6" component="h2" fontWeight="bold" mb={2}>
                  {t('pages.communities.title')}
                </Typography>
                <List>
                  {userCommunities.map((community, index) => (
                    <div key={community.id}>
                      <ListItem
                        sx={{
                          '&:hover': { backgroundColor: 'action.hover' },
                          cursor: 'pointer',
                        }}
                        onClick={() => {
                          setUserCommunities(userCommunities);
                        }}
                      >
                        <ListItemIcon>
                          <Avatar sx={{ bgcolor: 'primary.main' }}>
                            <GroupsIcon />
                          </Avatar>
                        </ListItemIcon>
                        <ListItemText
                          primary={community.name}
                          secondary={`${community.member_count.toLocaleString()} ${t('pages.communities.members')} • ${community.description}`}
                        />
                        <Chip
                          label={community.subscription_status === 'free' ? t('subscription.community.tiers.free.name') : t(`subscription.community.tiers.${community.subscription_status}.name`)}
                          size="small"
                          color={community.subscription_status === 'free' ? 'default' : 'primary'}
                        />
                      </ListItem>
                      {index < userCommunities.length - 1 && <Divider />}
                    </div>
                  ))}
                </List>
              </CardContent>
            </Card>
          )}
        </Box>
      </DashboardLayout>
    );
  }

  // Show community feed when community is selected
  return (
    <DashboardLayout>
      <Box>
        {/* Community Feed */}
        <CommunityFeed communityId={activeCommunity.id} />
      </Box>
    </DashboardLayout>
  );
}