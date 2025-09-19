'use client';

import { useState, useEffect } from 'react';
import { useParams } from 'next/navigation';
import { useSession } from 'next-auth/react';
import {
  Box,
  Container,
  Typography,
  Alert,
  CircularProgress,
  Card,
  CardContent,
  Button,
  Chip,
  Stack,
  Divider,
  Paper,
  IconButton,
} from '@mui/material';
import {
  ArrowBack as BackIcon,
  Groups as GroupsIcon,
  Star as StarIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import CommunitySubscriptionTiers from '@/components/subscription/CommunitySubscriptionTiers';
import apiClient from '@/lib/api-client';
import type { Community, ApiResponse } from '@/types/api';

// Mock community data for demonstration
const mockCommunity: Community = {
  id: '1',
  name: 'Downtown Milan',
  description: 'The heart of Milan\'s business and cultural district with vibrant community life',
  member_count: 2847,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  location: {
    latitude: 45.4642,
    longitude: 9.1900,
    address: 'Downtown Milan, Italy'
  },
  settings: {
    is_public: true,
    requires_approval: false,
    subscription_enabled: true,
    subscription_tiers: [
      {
        id: 'free',
        name: 'Community Member',
        price: 0,
        features: ['Basic access', 'Community discussions', 'Public events']
      },
      {
        id: 'supporter',
        name: 'Community Supporter',
        price: 12,
        features: ['All free features', 'Priority support', 'Exclusive events', 'Community badge']
      },
      {
        id: 'premium',
        name: 'Community VIP',
        price: 25,
        features: ['All supporter features', 'VIP badge', 'Private channels', 'Personal concierge']
      }
    ]
  }
};

export default function CommunitySubscriptionPage() {
  const params = useParams();
  const communityId = params.id as string;
  const { data: session } = useSession();
  const user = session?.user;

  const [community, setCommunity] = useState<Community | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentTier, setCurrentTier] = useState<string>('free');
  const [subscriptionSuccess, setSubscriptionSuccess] = useState(false);

  // Fetch community data
  useEffect(() => {
    const fetchCommunity = async () => {
      try {
        setLoading(true);
        setError(null);

        // For now, use mock data. Later connect to real API
        setTimeout(() => {
          setCommunity(mockCommunity);
          setLoading(false);
        }, 1000);

        // TODO: Replace with real API call
        // const response: ApiResponse<Community> = await apiClient.getCommunity(communityId);
        // if (response.success && response.data) {
        //   setCommunity(response.data);
        // } else {
        //   setError('Community not found');
        // }

      } catch (err) {
        console.error('Failed to fetch community:', err);
        setError('Failed to load community details. Please try again.');
      } finally {
        // setLoading(false);
      }
    };

    if (communityId) {
      fetchCommunity();
    }
  }, [communityId]);

  const handleSubscribe = async (tierId: string) => {
    try {
      // TODO: Implement real subscription logic
      console.log('Subscribing to tier:', tierId);

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));

      setCurrentTier(tierId);
      setSubscriptionSuccess(true);

      // Hide success message after 5 seconds
      setTimeout(() => setSubscriptionSuccess(false), 5000);

    } catch (err) {
      console.error('Subscription failed:', err);
      setError('Subscription failed. Please try again.');
    }
  };

  if (loading) {
    return (
      <DashboardLayout>
        <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
          <CircularProgress size={60} />
        </Box>
      </DashboardLayout>
    );
  }

  if (error || !community) {
    return (
      <DashboardLayout>
        <Container maxWidth="lg">
          <Alert severity="error" sx={{ mt: 4 }}>
            {error || 'Community not found'}
          </Alert>
        </Container>
      </DashboardLayout>
    );
  }

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box display="flex" alignItems="center" gap={2} mb={4}>
          <IconButton
            component={Link}
            href={`/communities/${communityId}`}
          >
            <BackIcon />
          </IconButton>
          <Box>
            <Typography variant="h4" fontWeight="bold">
              Community Membership
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Choose your membership level for {community.name}
            </Typography>
          </Box>
        </Box>

        {/* Success Message */}
        {subscriptionSuccess && (
          <Alert severity="success" sx={{ mb: 4 }}>
            Subscription successful! Welcome to your new membership tier.
          </Alert>
        )}

        {/* Community Info Card */}
        <Paper elevation={2} sx={{ p: 4, mb: 4 }}>
          <Box display="flex" alignItems="center" gap={3}>
            <Box
              sx={{
                width: 80,
                height: 80,
                bgcolor: 'primary.main',
                borderRadius: 2,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <GroupsIcon sx={{ fontSize: 40, color: 'white' }} />
            </Box>

            <Box flex={1}>
              <Typography variant="h5" fontWeight="bold" gutterBottom>
                {community.name}
              </Typography>
              <Typography variant="body1" color="text.secondary" paragraph>
                {community.description}
              </Typography>
              <Stack direction="row" spacing={2} alignItems="center">
                <Chip
                  icon={<GroupsIcon />}
                  label={`${community.member_count.toLocaleString()} members`}
                  variant="outlined"
                />
                <Chip
                  icon={<StarIcon />}
                  label="Premium Community"
                  color="primary"
                  variant="outlined"
                />
              </Stack>
            </Box>
          </Box>
        </Paper>

        {/* Current Subscription Status */}
        {currentTier !== 'free' && (
          <Card sx={{ mb: 4 }}>
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center">
                <Box>
                  <Typography variant="h6" fontWeight="bold" gutterBottom>
                    Current Subscription
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    You are currently subscribed to the{' '}
                    <strong>{currentTier === 'supporter' ? 'Community Supporter' : 'Community VIP'}</strong> tier
                  </Typography>
                </Box>
                <Button
                  variant="outlined"
                  color="error"
                  onClick={() => {
                    // TODO: Implement cancellation
                    console.log('Cancel subscription');
                  }}
                >
                  Cancel Subscription
                </Button>
              </Box>
            </CardContent>
          </Card>
        )}

        {/* Benefits Overview */}
        <Box mb={4}>
          <Typography variant="h5" fontWeight="bold" gutterBottom>
            Membership Benefits
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            Support {community.name} and unlock exclusive community features with a membership subscription.
            Your subscription helps maintain and improve the community experience for everyone.
          </Typography>

          <Paper sx={{ p: 3, bgcolor: 'action.hover' }}>
            <Typography variant="h6" fontWeight="bold" gutterBottom>
              Why Subscribe?
            </Typography>
            <Stack spacing={1}>
              <Typography variant="body2">
                • <strong>Support Local Community:</strong> Your subscription directly supports community initiatives and improvements
              </Typography>
              <Typography variant="body2">
                • <strong>Exclusive Access:</strong> Get access to member-only events, channels, and features
              </Typography>
              <Typography variant="body2">
                • <strong>Priority Support:</strong> Receive faster response times and dedicated community support
              </Typography>
              <Typography variant="body2">
                • <strong>Community Recognition:</strong> Display special badges and show your commitment to the community
              </Typography>
              <Typography variant="body2">
                • <strong>Enhanced Features:</strong> Access premium tools and advanced community functionality
              </Typography>
            </Stack>
          </Paper>
        </Box>

        {/* Subscription Tiers */}
        <CommunitySubscriptionTiers
          communityId={communityId}
          communityName={community.name}
          currentTier={currentTier}
          onSubscribe={handleSubscribe}
        />

        {/* Community Revenue Model Info */}
        <Box mt={6}>
          <Divider sx={{ mb: 3 }} />
          <Alert severity="info">
            <Typography variant="body2">
              <strong>Transparent Community Funding:</strong> {community.name} receives 80% of subscription revenue
              to fund community improvements and initiatives. The platform retains 20% to maintain and develop
              the community management tools.
            </Typography>
          </Alert>
        </Box>
      </Container>
    </DashboardLayout>
  );
}