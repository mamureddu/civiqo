'use client';

import { useSession, signIn, signOut } from 'next-auth/react';
import Link from 'next/link';
import dynamic from 'next/dynamic';
import {
  Box,
  Container,
  Typography,
  Button,
  Card,
  CardContent,
  Grid,
  Chip,
  Stack,
  CircularProgress,
  Alert,
  Paper,
  TextField,
  InputAdornment,
  Fab,
  Badge,
  Avatar,
  CardMedia,
} from '@mui/material';
import {
  Groups as GroupsIcon,
  Business as BusinessIcon,
  HowToVote as GovernanceIcon,
  Chat as ChatIcon,
  LocationOn as LocationIcon,
  Security as SecurityIcon,
  Login as LoginIcon,
  Dashboard as DashboardIcon,
  Search as SearchIcon,
  Map as MapIcon,
  Explore as ExploreIcon,
  People as PeopleIcon,
  Store as StoreIcon,
  Museum as MuseumIcon,
} from '@mui/icons-material';

// Dynamically import map to avoid SSR issues
const CommunityDiscoveryMap = dynamic(() => import('@/components/map/CommunityDiscoveryMap'), {
  ssr: false,
  loading: () => (
    <Box
      sx={{
        height: '500px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        bgcolor: 'grey.100',
        borderRadius: 2,
      }}
    >
      <CircularProgress />
    </Box>
  ),
});

// Mock data for community discovery
const mockCommunities = [
  {
    id: '1',
    name: 'Downtown Milan',
    description: 'The heart of Milan\'s business and cultural district',
    memberCount: 2847,
    businessCount: 156,
    location: { lat: 45.4642, lng: 9.1900 },
    image: 'https://images.unsplash.com/photo-1513475382585-d06e58bcb0e0?w=400',
    subscription: { hasSubscription: true, startingPrice: 15 },
    category: 'Urban'
  },
  {
    id: '2',
    name: 'Brera Arts District',
    description: 'Creative hub with galleries, cafes and artistic communities',
    memberCount: 1204,
    businessCount: 89,
    location: { lat: 45.4719, lng: 9.1881 },
    image: 'https://images.unsplash.com/photo-1576013551627-0cc20b96c2a7?w=400',
    subscription: { hasSubscription: false },
    category: 'Arts'
  },
  {
    id: '3',
    name: 'Navigli Canals',
    description: 'Historic canals area with nightlife and local businesses',
    memberCount: 3156,
    businessCount: 203,
    location: { lat: 45.4481, lng: 9.1803 },
    image: 'https://images.unsplash.com/photo-1551632436-cbf8dd35adfa?w=400',
    subscription: { hasSubscription: true, startingPrice: 12 },
    category: 'Entertainment'
  }
];

export default function HomePage() {
  const { data: session, status } = useSession();
  const user = session?.user;
  const isLoading = status === 'loading';

  const CommunityCard = ({ community }: { community: typeof mockCommunities[0] }) => (
    <Card
      sx={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        transition: 'transform 0.2s, box-shadow 0.2s',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow: 6,
        },
      }}
    >
      <CardMedia
        component="img"
        height="200"
        image={community.image}
        alt={community.name}
      />
      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={1}>
          <Typography variant="h6" fontWeight="bold">
            {community.name}
          </Typography>
          <Chip
            label={community.category}
            size="small"
            color="primary"
            variant="outlined"
          />
        </Box>

        <Typography variant="body2" color="text.secondary" paragraph>
          {community.description}
        </Typography>

        <Stack direction="row" spacing={2} mb={2}>
          <Stack direction="row" spacing={0.5} alignItems="center">
            <PeopleIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
            <Typography variant="caption" color="text.secondary">
              {community.memberCount.toLocaleString()} members
            </Typography>
          </Stack>
          <Stack direction="row" spacing={0.5} alignItems="center">
            <StoreIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
            <Typography variant="caption" color="text.secondary">
              {community.businessCount} businesses
            </Typography>
          </Stack>
        </Stack>

        {community.subscription.hasSubscription && (
          <Chip
            label={`From €${community.subscription.startingPrice}/month`}
            size="small"
            color="secondary"
            sx={{ mb: 2 }}
          />
        )}
      </CardContent>

      <Box sx={{ p: 2, pt: 0 }}>
        <Stack direction="row" spacing={1}>
          <Button
            variant="outlined"
            size="small"
            component={Link}
            href={`/communities/${community.id}`}
            sx={{ flexGrow: 1 }}
          >
            Explore
          </Button>
          <Button
            variant="contained"
            size="small"
            component={user ? 'a' : 'button'}
            href={user ? `/communities/${community.id}/subscription` : undefined}
            onClick={user ? undefined : () => signIn('auth0')}
            sx={{ flexGrow: 1 }}
          >
            {user ? 'View Membership' : 'Sign Up'}
          </Button>
        </Stack>
      </Box>
    </Card>
  );

  if (isLoading) {
    return (
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          minHeight: '100vh',
          bgcolor: 'background.default',
        }}
      >
        <CircularProgress size={60} />
      </Box>
    );
  }

  return (
    <Box sx={{ minHeight: '100vh', bgcolor: 'background.default' }}>
      {/* Navigation Bar */}
      <Paper elevation={1} sx={{ py: 2, px: 3, position: 'sticky', top: 0, zIndex: 100 }}>
        <Container maxWidth="lg">
          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Typography variant="h5" fontWeight="bold" color="primary">
              Community Manager
            </Typography>
            <Stack direction="row" spacing={2}>
              {user ? (
                <>
                  <Chip
                    avatar={
                      user.picture ? (
                        <Avatar src={user.picture} sx={{ width: 24, height: 24 }} />
                      ) : undefined
                    }
                    label={user.name || user.email}
                    variant="outlined"
                  />
                  <Button
                    component={Link}
                    href="/dashboard"
                    variant="contained"
                    startIcon={<DashboardIcon />}
                  >
                    My Communities
                  </Button>
                  <Button onClick={() => signOut()} variant="outlined">
                    Logout
                  </Button>
                </>
              ) : (
                <>
                  <Button
                    component={Link}
                    href="/explore"
                    variant="outlined"
                    startIcon={<ExploreIcon />}
                  >
                    Explore
                  </Button>
                  <Button
                    onClick={() => signIn('auth0')}
                    variant="contained"
                    startIcon={<LoginIcon />}
                  >
                    Join Community
                  </Button>
                </>
              )}
            </Stack>
          </Box>
        </Container>
      </Paper>

      {/* Hero Section with Search */}
      <Box
        sx={{
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
          color: 'white',
          py: { xs: 6, md: 8 },
          px: 3,
        }}
      >
        <Container maxWidth="lg">
          <Box textAlign="center" mb={4}>
            <Typography variant="h2" fontWeight="bold" gutterBottom>
              Discover Your Local Community
            </Typography>
            <Typography variant="h5" sx={{ mb: 4, opacity: 0.9 }}>
              Connect with neighbors, support local businesses, and participate in your community
            </Typography>

            <Box maxWidth="600px" mx="auto">
              <TextField
                fullWidth
                placeholder="Search communities by location or name..."
                variant="outlined"
                sx={{
                  bgcolor: 'white',
                  borderRadius: 2,
                  '& .MuiOutlinedInput-root': {
                    borderRadius: 2,
                  },
                }}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <SearchIcon />
                    </InputAdornment>
                  ),
                  endAdornment: (
                    <InputAdornment position="end">
                      <Button variant="contained" sx={{ borderRadius: 1 }}>
                        Search
                      </Button>
                    </InputAdornment>
                  ),
                }}
              />
            </Box>
          </Box>
        </Container>
      </Box>

      {/* Interactive Map Section */}
      <Container maxWidth="lg" sx={{ py: 4 }}>
        <Box mb={4}>
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Explore Communities Near You
          </Typography>
          <Typography variant="body1" color="text.secondary" mb={3}>
            Discover vibrant local communities with businesses, events, and engaged members
          </Typography>
        </Box>

        <Paper elevation={3} sx={{ borderRadius: 3, overflow: 'hidden', mb: 4 }}>
          <CommunityDiscoveryMap communities={mockCommunities} />
        </Paper>
      </Container>

      {/* Featured Communities */}
      <Container maxWidth="lg" sx={{ py: 4 }}>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={4}>
          <Typography variant="h4" fontWeight="bold">
            Featured Communities
          </Typography>
          <Button
            component={Link}
            href="/explore"
            variant="outlined"
            startIcon={<ExploreIcon />}
          >
            View All
          </Button>
        </Box>

        <Grid container spacing={3}>
          {mockCommunities.map((community) => (
            <Grid item xs={12} sm={6} lg={4} key={community.id}>
              <CommunityCard community={community} />
            </Grid>
          ))}
        </Grid>
      </Container>

      {/* Stats Section */}
      <Box sx={{ bgcolor: 'grey.50', py: 6 }}>
        <Container maxWidth="lg">
          <Grid container spacing={4} textAlign="center">
            <Grid item xs={12} sm={3}>
              <Typography variant="h3" fontWeight="bold" color="primary.main">
                50+
              </Typography>
              <Typography variant="h6" color="text.secondary">
                Active Communities
              </Typography>
            </Grid>
            <Grid item xs={12} sm={3}>
              <Typography variant="h3" fontWeight="bold" color="primary.main">
                10k+
              </Typography>
              <Typography variant="h6" color="text.secondary">
                Community Members
              </Typography>
            </Grid>
            <Grid item xs={12} sm={3}>
              <Typography variant="h3" fontWeight="bold" color="primary.main">
                2k+
              </Typography>
              <Typography variant="h6" color="text.secondary">
                Local Businesses
              </Typography>
            </Grid>
            <Grid item xs={12} sm={3}>
              <Typography variant="h3" fontWeight="bold" color="primary.main">
                500+
              </Typography>
              <Typography variant="h6" color="text.secondary">
                Points of Interest
              </Typography>
            </Grid>
          </Grid>
        </Container>
      </Box>

      {/* Call to Action */}
      <Box sx={{ py: 8 }}>
        <Container maxWidth="md" textAlign="center">
          <Typography variant="h4" fontWeight="bold" gutterBottom>
            Ready to Join Your Community?
          </Typography>
          <Typography variant="h6" color="text.secondary" sx={{ mb: 4 }}>
            Connect with your neighbors and discover what's happening around you
          </Typography>
          <Stack direction={{ xs: 'column', sm: 'row' }} spacing={2} justifyContent="center">
            {user ? (
              <Button
                component={Link}
                href="/dashboard"
                variant="contained"
                size="large"
                startIcon={<DashboardIcon />}
              >
                Go to My Communities
              </Button>
            ) : (
              <>
                <Button
                  onClick={() => signIn('auth0')}
                  variant="contained"
                  size="large"
                  startIcon={<LoginIcon />}
                >
                  Join a Community
                </Button>
                <Button
                  component={Link}
                  href="/explore"
                  variant="outlined"
                  size="large"
                  startIcon={<MapIcon />}
                >
                  Explore Communities
                </Button>
              </>
            )}
          </Stack>
        </Container>
      </Box>

      {/* Footer */}
      <Box
        component="footer"
        sx={{
          bgcolor: 'grey.900',
          color: 'grey.400',
          py: 4,
          px: 3,
        }}
      >
        <Container maxWidth="lg">
          <Grid container spacing={4}>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Community Manager
              </Typography>
              <Typography variant="body2" sx={{ mb: 2 }}>
                Connecting communities through secure, local social networks
              </Typography>
            </Grid>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Features
              </Typography>
              <Stack spacing={1}>
                <Typography variant="body2">Local Business Discovery</Typography>
                <Typography variant="body2">Secure Community Chat</Typography>
                <Typography variant="body2">Points of Interest</Typography>
                <Typography variant="body2">Community Governance</Typography>
              </Stack>
            </Grid>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Support
              </Typography>
              <Stack spacing={1}>
                <Typography variant="body2">Help Center</Typography>
                <Typography variant="body2">Privacy Policy</Typography>
                <Typography variant="body2">Terms of Service</Typography>
                <Typography variant="body2">Contact Us</Typography>
              </Stack>
            </Grid>
          </Grid>
          <Box sx={{ mt: 4, pt: 4, borderTop: 1, borderColor: 'grey.800' }}>
            <Typography variant="body2" align="center">
              © 2024 Community Manager. Building stronger local connections.
            </Typography>
          </Box>
        </Container>
      </Box>
    </Box>
  );
}