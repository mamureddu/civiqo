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
  IconButton,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  Divider,
  Avatar,
  CircularProgress,
} from '@mui/material';
import {
  Groups as GroupsIcon,
  Business as BusinessIcon,
  HowToVote as GovernanceIcon,
  Chat as ChatIcon,
  Add as AddIcon,
  TrendingUp as TrendingUpIcon,
  Notifications as NotificationsIcon,
  Event as EventIcon,
} from '@mui/icons-material';
import DashboardLayout from '@/components/layout/DashboardLayout';
import { useUser } from '@auth0/nextjs-auth0';
import apiClient from '@/lib/api-client';
import type { Community, ApiResponse } from '@/types/api';

// Mock data for demonstration
const mockStats = {
  communities: 3,
  businesses: 12,
  activePolls: 2,
  unreadMessages: 5,
};

const mockRecentActivity = [
  {
    id: 1,
    type: 'poll',
    title: 'New community park proposal',
    description: 'Vote on the new park construction in downtown area',
    time: '2 hours ago',
    icon: <GovernanceIcon />,
  },
  {
    id: 2,
    type: 'business',
    title: 'New business: Green Coffee Shop',
    description: 'A new local coffee shop has joined our community',
    time: '4 hours ago',
    icon: <BusinessIcon />,
  },
  {
    id: 3,
    type: 'message',
    title: 'Message from Sarah M.',
    description: 'Thanks for organizing the community cleanup!',
    time: '1 day ago',
    icon: <ChatIcon />,
  },
  {
    id: 4,
    type: 'event',
    title: 'Community Meeting Tomorrow',
    description: 'Monthly community meeting at 7 PM',
    time: '2 days ago',
    icon: <EventIcon />,
  },
];

export default function Dashboard() {
  const { user } = useUser();
  const [communities, setCommunities] = useState<Community[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchCommunities = async () => {
      try {
        const response: ApiResponse<Community[]> = await apiClient.getCommunities({ limit: 5 });
        if (response.success && response.data) {
          setCommunities(response.data);
        }
      } catch (error) {
        console.error('Failed to fetch communities:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchCommunities();
  }, []);

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

  return (
    <DashboardLayout>
      <Box>
        {/* Welcome Section */}
        <Box mb={4}>
          <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
            Welcome back, {user?.name || user?.email}!
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Here's what's happening in your communities today.
          </Typography>
        </Box>

        {/* Stats Cards */}
        <Grid container spacing={3} mb={4}>
          <Grid item xs={12} sm={6} md={3}>
            <StatCard
              title="Communities"
              value={mockStats.communities}
              icon={<GroupsIcon sx={{ fontSize: 40 }} />}
              color="primary.main"
              action={() => window.location.href = '/communities'}
            />
          </Grid>
          <Grid item xs={12} sm={6} md={3}>
            <StatCard
              title="Local Businesses"
              value={mockStats.businesses}
              icon={<BusinessIcon sx={{ fontSize: 40 }} />}
              color="secondary.main"
              action={() => window.location.href = '/businesses'}
            />
          </Grid>
          <Grid item xs={12} sm={6} md={3}>
            <StatCard
              title="Active Polls"
              value={mockStats.activePolls}
              icon={<GovernanceIcon sx={{ fontSize: 40 }} />}
              color="success.main"
              action={() => window.location.href = '/governance'}
            />
          </Grid>
          <Grid item xs={12} sm={6} md={3}>
            <StatCard
              title="Unread Messages"
              value={mockStats.unreadMessages}
              icon={<ChatIcon sx={{ fontSize: 40 }} />}
              color="warning.main"
              action={() => window.location.href = '/chat'}
            />
          </Grid>
        </Grid>

        <Grid container spacing={3}>
          {/* My Communities */}
          <Grid item xs={12} md={6}>
            <Card>
              <CardContent>
                <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                  <Typography variant="h6" component="h2" fontWeight="bold">
                    My Communities
                  </Typography>
                  <Button
                    variant="outlined"
                    size="small"
                    startIcon={<AddIcon />}
                    href="/communities/create"
                  >
                    Create
                  </Button>
                </Box>

                {loading ? (
                  <Box display="flex" justifyContent="center" py={2}>
                    <CircularProgress />
                  </Box>
                ) : communities.length > 0 ? (
                  <List>
                    {communities.slice(0, 3).map((community, index) => (
                      <div key={community.id}>
                        <ListItem
                          sx={{
                            px: 0,
                            '&:hover': { backgroundColor: 'action.hover' },
                            cursor: 'pointer',
                          }}
                          onClick={() => window.location.href = `/communities/${community.id}`}
                        >
                          <ListItemIcon>
                            <Avatar sx={{ bgcolor: 'primary.main' }}>
                              <GroupsIcon />
                            </Avatar>
                          </ListItemIcon>
                          <ListItemText
                            primary={community.name}
                            secondary={`${community.member_count} members • ${community.community_type}`}
                          />
                          <Chip
                            label={community.is_public ? 'Public' : 'Private'}
                            size="small"
                            color={community.is_public ? 'success' : 'default'}
                          />
                        </ListItem>
                        {index < Math.min(communities.length, 3) - 1 && <Divider />}
                      </div>
                    ))}
                  </List>
                ) : (
                  <Typography color="text.secondary" textAlign="center" py={3}>
                    No communities yet. Create or join your first community!
                  </Typography>
                )}

                <Box mt={2}>
                  <Button variant="text" size="small" href="/communities" fullWidth>
                    View All Communities
                  </Button>
                </Box>
              </CardContent>
            </Card>
          </Grid>

          {/* Recent Activity */}
          <Grid item xs={12} md={6}>
            <Card>
              <CardContent>
                <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
                  <Typography variant="h6" component="h2" fontWeight="bold">
                    Recent Activity
                  </Typography>
                  <IconButton size="small">
                    <NotificationsIcon />
                  </IconButton>
                </Box>

                <List>
                  {mockRecentActivity.map((activity, index) => (
                    <div key={activity.id}>
                      <ListItem sx={{ px: 0 }}>
                        <ListItemIcon>
                          <Avatar sx={{ bgcolor: 'secondary.main', width: 32, height: 32 }}>
                            {activity.icon}
                          </Avatar>
                        </ListItemIcon>
                        <ListItemText
                          primary={activity.title}
                          secondary={
                            <Box>
                              <Typography variant="body2" color="text.secondary">
                                {activity.description}
                              </Typography>
                              <Typography variant="caption" color="text.secondary">
                                {activity.time}
                              </Typography>
                            </Box>
                          }
                        />
                      </ListItem>
                      {index < mockRecentActivity.length - 1 && <Divider />}
                    </div>
                  ))}
                </List>
              </CardContent>
            </Card>
          </Grid>
        </Grid>

        {/* Quick Actions */}
        <Box mt={4}>
          <Typography variant="h6" component="h2" fontWeight="bold" mb={2}>
            Quick Actions
          </Typography>
          <Grid container spacing={2}>
            <Grid item xs={12} sm={6} md={3}>
              <Button
                variant="outlined"
                fullWidth
                startIcon={<GroupsIcon />}
                href="/communities/create"
                sx={{ py: 1.5 }}
              >
                Create Community
              </Button>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Button
                variant="outlined"
                fullWidth
                startIcon={<BusinessIcon />}
                href="/businesses/create"
                sx={{ py: 1.5 }}
              >
                Add Business
              </Button>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Button
                variant="outlined"
                fullWidth
                startIcon={<GovernanceIcon />}
                href="/governance/polls/create"
                sx={{ py: 1.5 }}
              >
                Create Poll
              </Button>
            </Grid>
            <Grid item xs={12} sm={6} md={3}>
              <Button
                variant="outlined"
                fullWidth
                startIcon={<ChatIcon />}
                href="/chat"
                sx={{ py: 1.5 }}
              >
                Open Chat
              </Button>
            </Grid>
          </Grid>
        </Box>
      </Box>
    </DashboardLayout>
  );
}