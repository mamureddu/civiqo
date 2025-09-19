'use client';

import { useUser } from '@auth0/nextjs-auth0';
import Link from 'next/link';
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
} from '@mui/icons-material';

export default function HomePage() {
  const { user, error, isLoading } = useUser();

  const features = [
    {
      icon: <GroupsIcon sx={{ fontSize: 40 }} />,
      title: 'Community Management',
      description: 'Create and join location-based communities with role-based access control',
      color: 'primary',
    },
    {
      icon: <BusinessIcon sx={{ fontSize: 40 }} />,
      title: 'Local Business Directory',
      description: 'Discover and support local businesses in your community',
      color: 'secondary',
    },
    {
      icon: <GovernanceIcon sx={{ fontSize: 40 }} />,
      title: 'Democratic Governance',
      description: 'Participate in community decisions through polls and voting systems',
      color: 'success',
    },
    {
      icon: <ChatIcon sx={{ fontSize: 40 }} />,
      title: 'Secure Chat',
      description: 'End-to-end encrypted messaging with no server-side storage',
      color: 'info',
    },
    {
      icon: <LocationIcon sx={{ fontSize: 40 }} />,
      title: 'Geographic Communities',
      description: 'Location-based communities with interactive mapping features',
      color: 'warning',
    },
    {
      icon: <SecurityIcon sx={{ fontSize: 40 }} />,
      title: 'Privacy First',
      description: 'Your data stays secure with enterprise-grade encryption',
      color: 'error',
    },
  ];

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

  if (error) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="error">
          Authentication error: {error.message}
        </Alert>
      </Box>
    );
  }

  return (
    <Box sx={{ minHeight: '100vh', bgcolor: 'background.default' }}>
      {/* Navigation Bar */}
      <Paper elevation={1} sx={{ py: 2, px: 3 }}>
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
                        <Box
                          component="img"
                          src={user.picture}
                          sx={{ width: 24, height: 24, borderRadius: '50%' }}
                        />
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
                    Dashboard
                  </Button>
                  <Button
                    component="a"
                    href="/api/auth/logout"
                    variant="outlined"
                  >
                    Logout
                  </Button>
                </>
              ) : (
                <Button
                  component="a"
                  href="/api/auth/login"
                  variant="contained"
                  startIcon={<LoginIcon />}
                >
                  Login / Sign Up
                </Button>
              )}
            </Stack>
          </Box>
        </Container>
      </Paper>

      {/* Hero Section */}
      <Box
        sx={{
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
          color: 'white',
          py: { xs: 8, md: 12 },
          px: 3,
        }}
      >
        <Container maxWidth="lg">
          <Grid container spacing={4} alignItems="center">
            <Grid item xs={12} md={6}>
              <Typography variant="h2" fontWeight="bold" gutterBottom>
                Connect Your Local Community
              </Typography>
              <Typography variant="h5" sx={{ mb: 4, opacity: 0.9 }}>
                Empowering communities with secure communication, local business support,
                and democratic decision-making tools.
              </Typography>
              <Stack direction={{ xs: 'column', sm: 'row' }} spacing={2}>
                {user ? (
                  <Button
                    component={Link}
                    href="/dashboard"
                    variant="contained"
                    size="large"
                    sx={{
                      bgcolor: 'white',
                      color: 'primary.main',
                      '&:hover': { bgcolor: 'grey.100' },
                    }}
                    startIcon={<DashboardIcon />}
                  >
                    Go to Dashboard
                  </Button>
                ) : (
                  <>
                    <Button
                      component="a"
                      href="/api/auth/login"
                      variant="contained"
                      size="large"
                      sx={{
                        bgcolor: 'white',
                        color: 'primary.main',
                        '&:hover': { bgcolor: 'grey.100' },
                      }}
                    >
                      Get Started Free
                    </Button>
                    <Button
                      variant="outlined"
                      size="large"
                      sx={{
                        borderColor: 'white',
                        color: 'white',
                        '&:hover': {
                          borderColor: 'white',
                          bgcolor: 'rgba(255, 255, 255, 0.1)',
                        },
                      }}
                    >
                      Learn More
                    </Button>
                  </>
                )}
              </Stack>
            </Grid>
            <Grid item xs={12} md={6}>
              <Box
                sx={{
                  display: 'flex',
                  justifyContent: 'center',
                  opacity: 0.9,
                }}
              >
                <GroupsIcon sx={{ fontSize: 300 }} />
              </Box>
            </Grid>
          </Grid>
        </Container>
      </Box>

      {/* Features Section */}
      <Container maxWidth="lg" sx={{ py: 8 }}>
        <Typography variant="h3" align="center" fontWeight="bold" gutterBottom>
          Everything Your Community Needs
        </Typography>
        <Typography variant="h6" align="center" color="text.secondary" sx={{ mb: 6 }}>
          A comprehensive platform designed for modern local communities
        </Typography>

        <Grid container spacing={4}>
          {features.map((feature, index) => (
            <Grid item xs={12} sm={6} md={4} key={index}>
              <Card
                sx={{
                  height: '100%',
                  transition: 'transform 0.3s, box-shadow 0.3s',
                  '&:hover': {
                    transform: 'translateY(-4px)',
                    boxShadow: 4,
                  },
                }}
              >
                <CardContent sx={{ textAlign: 'center', p: 3 }}>
                  <Box
                    sx={{
                      display: 'inline-flex',
                      p: 2,
                      borderRadius: '50%',
                      bgcolor: `${feature.color}.light`,
                      color: `${feature.color}.main`,
                      mb: 2,
                    }}
                  >
                    {feature.icon}
                  </Box>
                  <Typography variant="h6" fontWeight="bold" gutterBottom>
                    {feature.title}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {feature.description}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Container>

      {/* CTA Section */}
      {!user && (
        <Box
          sx={{
            bgcolor: 'primary.main',
            color: 'white',
            py: 8,
            px: 3,
            textAlign: 'center',
          }}
        >
          <Container maxWidth="md">
            <Typography variant="h4" fontWeight="bold" gutterBottom>
              Ready to Transform Your Community?
            </Typography>
            <Typography variant="h6" sx={{ mb: 4, opacity: 0.9 }}>
              Join thousands of communities already using our platform
            </Typography>
            <Button
              component="a"
              href="/api/auth/login"
              variant="contained"
              size="large"
              sx={{
                bgcolor: 'white',
                color: 'primary.main',
                '&:hover': { bgcolor: 'grey.100' },
              }}
            >
              Start Your Community Today
            </Button>
          </Container>
        </Box>
      )}

      {/* Footer */}
      <Box
        component="footer"
        sx={{
          bgcolor: 'grey.900',
          color: 'grey.400',
          py: 4,
          px: 3,
          mt: 'auto',
        }}
      >
        <Container maxWidth="lg">
          <Grid container spacing={4}>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Community Manager
              </Typography>
              <Typography variant="body2">
                Building stronger communities through technology
              </Typography>
            </Grid>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Features
              </Typography>
              <Stack spacing={1}>
                <Typography variant="body2">Community Management</Typography>
                <Typography variant="body2">Business Directory</Typography>
                <Typography variant="body2">Secure Messaging</Typography>
                <Typography variant="body2">Democratic Governance</Typography>
              </Stack>
            </Grid>
            <Grid item xs={12} md={4}>
              <Typography variant="h6" color="white" gutterBottom>
                Legal
              </Typography>
              <Stack spacing={1}>
                <Typography variant="body2">Privacy Policy</Typography>
                <Typography variant="body2">Terms of Service</Typography>
                <Typography variant="body2">Cookie Policy</Typography>
              </Stack>
            </Grid>
          </Grid>
          <Box sx={{ mt: 4, pt: 4, borderTop: 1, borderColor: 'grey.800' }}>
            <Typography variant="body2" align="center">
              © 2024 Community Manager. All rights reserved.
            </Typography>
          </Box>
        </Container>
      </Box>
    </Box>
  );
}