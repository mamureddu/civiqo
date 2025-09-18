'use client';

import { useUser } from '@auth0/nextjs-auth0/client';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';
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
} from '@mui/material';
import {
  Groups as GroupsIcon,
  Business as BusinessIcon,
  HowToVote as GovernanceIcon,
  Chat as ChatIcon,
  Security as SecurityIcon,
  Phone as PhoneIcon,
} from '@mui/icons-material';

export default function Home() {
  const { user, isLoading } = useUser();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && user) {
      router.push('/dashboard');
    }
  }, [user, isLoading, router]);

  if (isLoading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="100vh">
        <Typography>Loading...</Typography>
      </Box>
    );
  }

  const features = [
    {
      icon: <GroupsIcon sx={{ fontSize: 40 }} />,
      title: 'Community Management',
      description: 'Create and manage local communities with role-based access control and member management.',
    },
    {
      icon: <BusinessIcon sx={{ fontSize: 40 }} />,
      title: 'Business Directory',
      description: 'Discover local businesses with geographic mapping and product catalogs.',
    },
    {
      icon: <ChatIcon sx={{ fontSize: 40 }} />,
      title: 'Real-time Chat',
      description: 'End-to-end encrypted messaging with community-based chat rooms.',
    },
    {
      icon: <GovernanceIcon sx={{ fontSize: 40 }} />,
      title: 'Democratic Governance',
      description: 'Polls, voting systems, and community decision-making tools.',
    },
    {
      icon: <SecurityIcon sx={{ fontSize: 40 }} />,
      title: 'Secure & Private',
      description: 'Auth0 authentication with comprehensive security measures and data protection.',
    },
    {
      icon: <PhoneIcon sx={{ fontSize: 40 }} />,
      title: 'Progressive Web App',
      description: 'Mobile-responsive design with PWA features for a native app experience.',
    },
  ];

  return (
    <Box sx={{ minHeight: '100vh', background: 'linear-gradient(135deg, #2E7D32 0%, #1976D2 100%)' }}>
      <Container maxWidth="lg" sx={{ py: 8 }}>
        {/* Hero Section */}
        <Box sx={{ textAlign: 'center', mb: 8 }}>
          <Typography variant="h2" component="h1" color="white" fontWeight="bold" mb={3}>
            Community Manager
          </Typography>
          <Typography variant="h5" color="white" mb={4} sx={{ opacity: 0.9 }}>
            Local community management platform with real-time chat, business directory, and democratic governance tools
          </Typography>
          <Stack direction="row" spacing={1} justifyContent="center" mb={4}>
            <Chip label="Open Source" color="secondary" variant="filled" />
            <Chip label="Real-time" color="secondary" variant="filled" />
            <Chip label="Secure" color="secondary" variant="filled" />
            <Chip label="Mobile Ready" color="secondary" variant="filled" />
          </Stack>
          <Button
            variant="contained"
            size="large"
            color="secondary"
            href="/api/auth/login"
            sx={{
              px: 4,
              py: 1.5,
              fontSize: '1.1rem',
              textTransform: 'none',
              boxShadow: 3,
            }}
          >
            Get Started
          </Button>
        </Box>

        {/* Features Grid */}
        <Grid container spacing={4}>
          {features.map((feature, index) => (
            <Grid item xs={12} md={6} lg={4} key={index}>
              <Card
                sx={{
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                  transition: 'transform 0.2s ease-in-out',
                  '&:hover': {
                    transform: 'translateY(-4px)',
                    boxShadow: 4,
                  },
                }}
              >
                <CardContent sx={{ flexGrow: 1, textAlign: 'center', p: 3 }}>
                  <Box sx={{ color: 'primary.main', mb: 2 }}>
                    {feature.icon}
                  </Box>
                  <Typography variant="h6" component="h3" fontWeight="bold" mb={2}>
                    {feature.title}
                  </Typography>
                  <Typography variant="body2" color="text.secondary" lineHeight={1.6}>
                    {feature.description}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>

        {/* Call to Action */}
        <Box sx={{ textAlign: 'center', mt: 8 }}>
          <Typography variant="h4" color="white" fontWeight="bold" mb={2}>
            Ready to build stronger communities?
          </Typography>
          <Typography variant="body1" color="white" mb={4} sx={{ opacity: 0.9 }}>
            Join thousands of communities already using Community Manager
          </Typography>
          <Button
            variant="contained"
            size="large"
            color="secondary"
            href="/api/auth/login"
            sx={{
              px: 4,
              py: 1.5,
              fontSize: '1.1rem',
              textTransform: 'none',
              boxShadow: 3,
            }}
          >
            Start Building Your Community
          </Button>
        </Box>
      </Container>
    </Box>
  );
}
