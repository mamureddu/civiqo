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
  LinearProgress,
  Grid,
} from '@mui/material';
import {
  ArrowBack as BackIcon,
  Business as BusinessIcon,
  Star as StarIcon,
  TrendingUp as TrendingUpIcon,
  Analytics as AnalyticsIcon,
  CheckCircle as CheckIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import BusinessSubscriptionTiers from '@/components/subscription/BusinessSubscriptionTiers';
import apiClient from '@/lib/api-client';
import type { Business, ApiResponse } from '@/types/api';

// Mock business data for demonstration
const mockBusiness: Business = {
  id: '1',
  name: 'Milano Coffee Roasters',
  description: 'Artisanal coffee roastery in the heart of Milan, serving premium single-origin beans and handcrafted beverages.',
  category: 'Restaurant',
  address: 'Via Brera 12, 20121 Milano, Italy',
  phone: '+39 02 1234 5678',
  website: 'https://milanocoffee.it',
  image_url: 'https://images.unsplash.com/photo-1447933601403-0c6688de566e?w=400',
  rating: 4.7,
  business_hours: {
    monday: '7:00-19:00',
    tuesday: '7:00-19:00',
    wednesday: '7:00-19:00',
    thursday: '7:00-19:00',
    friday: '7:00-20:00',
    saturday: '8:00-20:00',
    sunday: '8:00-18:00'
  },
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z'
};

export default function BusinessSubscriptionPage() {
  const params = useParams();
  const businessId = params.id as string;
  const { data: session } = useSession();
  const user = session?.user;

  const [business, setBusiness] = useState<Business | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentTier, setCurrentTier] = useState<string>('basic');
  const [subscriptionSuccess, setSubscriptionSuccess] = useState(false);
  const [usageStats, setUsageStats] = useState({
    postsThisMonth: 3,
    postsLimit: 5,
    imagesThisMonth: 8,
    imagesLimit: 10,
    viewsThisMonth: 142,
    engagementRate: 8.5
  });

  // Fetch business data
  useEffect(() => {
    const fetchBusiness = async () => {
      try {
        setLoading(true);
        setError(null);

        // For now, use mock data. Later connect to real API
        setTimeout(() => {
          setBusiness(mockBusiness);
          setLoading(false);
        }, 1000);

        // TODO: Replace with real API call
        // const response: ApiResponse<Business> = await apiClient.getBusiness(businessId);
        // if (response.success && response.data) {
        //   setBusiness(response.data);
        // } else {
        //   setError('Business not found');
        // }

      } catch (err) {
        console.error('Failed to fetch business:', err);
        setError('Failed to load business details. Please try again.');
      } finally {
        // setLoading(false);
      }
    };

    if (businessId) {
      fetchBusiness();
    }
  }, [businessId]);

  const handleSubscribe = async (tierId: string) => {
    try {
      // TODO: Implement real subscription logic
      console.log('Business subscribing to tier:', tierId);

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));

      setCurrentTier(tierId);
      setSubscriptionSuccess(true);

      // Hide success message after 5 seconds
      setTimeout(() => setSubscriptionSuccess(false), 5000);

    } catch (err) {
      console.error('Business subscription failed:', err);
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

  if (error || !business) {
    return (
      <DashboardLayout>
        <Container maxWidth="lg">
          <Alert severity="error" sx={{ mt: 4 }}>
            {error || 'Business not found'}
          </Alert>
        </Container>
      </DashboardLayout>
    );
  }

  const usagePercentage = (usageStats.postsThisMonth / usageStats.postsLimit) * 100;
  const isNearLimit = usagePercentage > 80;

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box display="flex" alignItems="center" gap={2} mb={4}>
          <IconButton
            component={Link}
            href={`/businesses/${businessId}`}
          >
            <BackIcon />
          </IconButton>
          <Box>
            <Typography variant="h4" fontWeight="bold">
              Bacheca Subscription
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Manage your subscription for {business.name}
            </Typography>
          </Box>
        </Box>

        {/* Success Message */}
        {subscriptionSuccess && (
          <Alert severity="success" sx={{ mb: 4 }}>
            Subscription updated successfully! Your new plan is now active.
          </Alert>
        )}

        {/* Business Info Card */}
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
              <BusinessIcon sx={{ fontSize: 40, color: 'white' }} />
            </Box>

            <Box flex={1}>
              <Typography variant="h5" fontWeight="bold" gutterBottom>
                {business.name}
              </Typography>
              <Typography variant="body1" color="text.secondary" paragraph>
                {business.description}
              </Typography>
              <Stack direction="row" spacing={2} alignItems="center">
                <Chip
                  icon={<BusinessIcon />}
                  label={business.category}
                  variant="outlined"
                />
                <Chip
                  icon={<StarIcon />}
                  label={`${business.rating} stars`}
                  color="primary"
                  variant="outlined"
                />
                <Chip
                  icon={<TrendingUpIcon />}
                  label={`${usageStats.viewsThisMonth} views this month`}
                  color="success"
                  variant="outlined"
                />
              </Stack>
            </Box>
          </Box>
        </Paper>

        {/* Current Usage Stats */}
        <Card sx={{ mb: 4 }}>
          <CardContent>
            <Typography variant="h6" fontWeight="bold" gutterBottom>
              Current Month Usage
            </Typography>

            <Stack spacing={3}>
              {/* Posts Usage */}
              <Box>
                <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
                  <Typography variant="body2" fontWeight="medium">
                    Bacheca Posts
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {usageStats.postsThisMonth} / {usageStats.postsLimit}
                  </Typography>
                </Box>
                <LinearProgress
                  variant="determinate"
                  value={usagePercentage}
                  color={isNearLimit ? 'warning' : 'primary'}
                />
                {isNearLimit && (
                  <Typography variant="caption" color="warning.main" sx={{ mt: 0.5 }}>
                    You're approaching your monthly limit. Consider upgrading for unlimited posts.
                  </Typography>
                )}
              </Box>

              {/* Analytics Summary */}
              <Box>
                <Typography variant="body2" fontWeight="medium" gutterBottom>
                  This Month's Performance
                </Typography>
                <Grid container spacing={2}>
                  <Grid xs={6} sm={3}>
                    <Paper sx={{ p: 2, textAlign: 'center' }}>
                      <Typography variant="h5" fontWeight="bold" color="primary.main">
                        {usageStats.viewsThisMonth}
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        Profile Views
                      </Typography>
                    </Paper>
                  </Grid>
                  <Grid xs={6} sm={3}>
                    <Paper sx={{ p: 2, textAlign: 'center' }}>
                      <Typography variant="h5" fontWeight="bold" color="success.main">
                        {usageStats.engagementRate}%
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        Engagement Rate
                      </Typography>
                    </Paper>
                  </Grid>
                  <Grid xs={6} sm={3}>
                    <Paper sx={{ p: 2, textAlign: 'center' }}>
                      <Typography variant="h5" fontWeight="bold" color="info.main">
                        {usageStats.postsThisMonth}
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        Posts Published
                      </Typography>
                    </Paper>
                  </Grid>
                  <Grid xs={6} sm={3}>
                    <Paper sx={{ p: 2, textAlign: 'center' }}>
                      <Typography variant="h5" fontWeight="bold" color="secondary.main">
                        12
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        Chat Inquiries
                      </Typography>
                    </Paper>
                  </Grid>
                </Grid>
              </Box>
            </Stack>
          </CardContent>
        </Card>

        {/* Current Subscription Status */}
        {currentTier !== 'basic' && (
          <Card sx={{ mb: 4 }}>
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center">
                <Box>
                  <Typography variant="h6" fontWeight="bold" gutterBottom>
                    Current Subscription
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    You are currently subscribed to the{' '}
                    <strong>
                      {currentTier === 'professional' ? 'Professional Bacheca' : 'Premium Bacheca Plus'}
                    </strong> plan
                  </Typography>
                  <Stack direction="row" spacing={1} sx={{ mt: 1 }}>
                    <Chip
                      icon={<CheckIcon />}
                      label="Active"
                      color="success"
                      size="small"
                    />
                    <Chip
                      label="Next billing: Feb 15, 2024"
                      variant="outlined"
                      size="small"
                    />
                  </Stack>
                </Box>
                <Button
                  variant="outlined"
                  color="error"
                  onClick={() => {
                    // TODO: Implement cancellation
                    console.log('Cancel business subscription');
                  }}
                >
                  Manage Subscription
                </Button>
              </Box>
            </CardContent>
          </Card>
        )}

        {/* Benefits Overview */}
        <Box mb={4}>
          <Typography variant="h5" fontWeight="bold" gutterBottom>
            Bacheca Subscription Benefits
          </Typography>
          <Typography variant="body1" color="text.secondary" paragraph>
            Enhance your business visibility and attract more customers with our premium bacheca features.
            Choose the plan that fits your business needs.
          </Typography>

          <Paper sx={{ p: 3, bgcolor: 'action.hover' }}>
            <Typography variant="h6" fontWeight="bold" gutterBottom>
              Why Upgrade Your Bacheca?
            </Typography>
            <Stack spacing={1}>
              <Typography variant="body2">
                • <strong>Unlimited Posts:</strong> Share as many updates, offers, and announcements as you need
              </Typography>
              <Typography variant="body2">
                • <strong>Priority Placement:</strong> Your business appears higher in community searches
              </Typography>
              <Typography variant="body2">
                • <strong>Advanced Analytics:</strong> Track your performance and understand customer engagement
              </Typography>
              <Typography variant="body2">
                • <strong>Rich Media Support:</strong> Add high-quality images and videos to your posts
              </Typography>
              <Typography variant="body2">
                • <strong>Event Promotion:</strong> Create and promote special events and offers
              </Typography>
              <Typography variant="body2">
                • <strong>Customer Reviews:</strong> Manage and respond to customer feedback directly
              </Typography>
            </Stack>
          </Paper>
        </Box>

        {/* Subscription Tiers */}
        <BusinessSubscriptionTiers
          businessId={businessId}
          businessName={business.name}
          currentTier={currentTier}
          onSubscribe={handleSubscribe}
        />

        {/* Revenue Model Info */}
        <Box mt={6}>
          <Divider sx={{ mb: 3 }} />
          <Alert severity="info">
            <Typography variant="body2">
              <strong>Transparent Pricing:</strong> Your subscription helps maintain and improve the community platform.
              All plans include access to encrypted chat, community engagement tools, and secure customer communications.
            </Typography>
          </Alert>
        </Box>
      </Container>
    </DashboardLayout>
  );
}