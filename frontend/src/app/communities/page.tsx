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
  TextField,
  InputAdornment,
  MenuItem,
  FormControl,
  InputLabel,
  Select,
  CircularProgress,
  Alert,
  Avatar,
  IconButton,
  Fab,
} from '@mui/material';
import {
  Search as SearchIcon,
  Add as AddIcon,
  Groups as GroupsIcon,
  LocationOn as LocationIcon,
  Public as PublicIcon,
  Lock as LockIcon,
  Person as PersonIcon,
} from '@mui/icons-material';
import DashboardLayout from '@/components/layout/DashboardLayout';
import { useRouter } from 'next/navigation';
import apiClient from '@/lib/api-client';
import type { Community, CommunityType, ApiResponse } from '@/types/api';

export default function CommunitiesPage() {
  const router = useRouter();
  const [communities, setCommunities] = useState<Community[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [filterType, setFilterType] = useState<CommunityType | 'all'>('all');
  const [filterVisibility, setFilterVisibility] = useState<'all' | 'public' | 'private'>('all');

  useEffect(() => {
    fetchCommunities();
  }, []);

  const fetchCommunities = async () => {
    try {
      setLoading(true);
      setError(null);
      const response: ApiResponse<Community[]> = await apiClient.getCommunities();

      if (response.success && response.data) {
        setCommunities(response.data);
      } else {
        setError(response.error?.message || 'Failed to fetch communities');
      }
    } catch (error) {
      console.error('Failed to fetch communities:', error);
      setError('Failed to connect to server');
    } finally {
      setLoading(false);
    }
  };

  const filteredCommunities = communities.filter(community => {
    const matchesSearch = community.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         (community.description?.toLowerCase().includes(searchQuery.toLowerCase()) ?? false);
    const matchesType = filterType === 'all' || community.community_type === filterType;
    const matchesVisibility = filterVisibility === 'all' ||
                             (filterVisibility === 'public' && community.is_public) ||
                             (filterVisibility === 'private' && !community.is_public);

    return matchesSearch && matchesType && matchesVisibility;
  });

  const handleJoinCommunity = async (communityId: string) => {
    try {
      const response = await apiClient.joinCommunity(communityId);
      if (response.success) {
        // Refresh communities list
        fetchCommunities();
      } else {
        setError(response.error?.message || 'Failed to join community');
      }
    } catch (error) {
      console.error('Failed to join community:', error);
      setError('Failed to join community');
    }
  };

  const CommunityCard = ({ community }: { community: Community }) => (
    <Card
      sx={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        transition: 'transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out',
        '&:hover': {
          transform: 'translateY(-2px)',
          boxShadow: 4,
        },
        cursor: 'pointer',
      }}
      onClick={() => router.push(`/communities/${community.id}`)}
    >
      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
          <Avatar sx={{ bgcolor: 'primary.main' }}>
            <GroupsIcon />
          </Avatar>
          <Box display="flex" gap={1}>
            <Chip
              icon={community.is_public ? <PublicIcon /> : <LockIcon />}
              label={community.is_public ? 'Public' : 'Private'}
              size="small"
              color={community.is_public ? 'success' : 'default'}
            />
          </Box>
        </Box>

        <Typography variant="h6" component="h3" fontWeight="bold" gutterBottom>
          {community.name}
        </Typography>

        {community.description && (
          <Typography variant="body2" color="text.secondary" paragraph>
            {community.description}
          </Typography>
        )}

        <Box display="flex" flex-wrap="wrap" gap={1} mb={2}>
          <Chip
            label={community.community_type}
            size="small"
            variant="outlined"
          />
          {community.location && (
            <Chip
              icon={<LocationIcon />}
              label={community.location}
              size="small"
              variant="outlined"
            />
          )}
        </Box>

        <Box display="flex" alignItems="center" justifyContent="space-between" mt="auto">
          <Box display="flex" alignItems="center" gap={1}>
            <PersonIcon fontSize="small" color="action" />
            <Typography variant="body2" color="text.secondary">
              {community.member_count} member{community.member_count !== 1 ? 's' : ''}
            </Typography>
          </Box>

          <Button
            variant="outlined"
            size="small"
            onClick={(e) => {
              e.stopPropagation();
              handleJoinCommunity(community.id);
            }}
          >
            Join
          </Button>
        </Box>
      </CardContent>
    </Card>
  );

  return (
    <DashboardLayout>
      <Box>
        {/* Header */}
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={4}>
          <Box>
            <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
              Communities
            </Typography>
            <Typography variant="body1" color="text.secondary">
              Discover and join local communities in your area
            </Typography>
          </Box>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => router.push('/communities/create')}
            size="large"
          >
            Create Community
          </Button>
        </Box>

        {/* Filters */}
        <Box mb={4}>
          <Grid container spacing={2} alignItems="center">
            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                placeholder="Search communities..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <SearchIcon />
                    </InputAdornment>
                  ),
                }}
              />
            </Grid>
            <Grid item xs={12} sm={6} md={4}>
              <FormControl fullWidth>
                <InputLabel>Type</InputLabel>
                <Select
                  value={filterType}
                  label="Type"
                  onChange={(e) => setFilterType(e.target.value as CommunityType | 'all')}
                >
                  <MenuItem value="all">All Types</MenuItem>
                  <MenuItem value="geographic">Geographic</MenuItem>
                  <MenuItem value="interest">Interest</MenuItem>
                  <MenuItem value="organization">Organization</MenuItem>
                  <MenuItem value="event">Event</MenuItem>
                </Select>
              </FormControl>
            </Grid>
            <Grid item xs={12} sm={6} md={4}>
              <FormControl fullWidth>
                <InputLabel>Visibility</InputLabel>
                <Select
                  value={filterVisibility}
                  label="Visibility"
                  onChange={(e) => setFilterVisibility(e.target.value as 'all' | 'public' | 'private')}
                >
                  <MenuItem value="all">All</MenuItem>
                  <MenuItem value="public">Public</MenuItem>
                  <MenuItem value="private">Private</MenuItem>
                </Select>
              </FormControl>
            </Grid>
          </Grid>
        </Box>

        {/* Error Display */}
        {error && (
          <Alert
            severity="error"
            sx={{ mb: 3 }}
            onClose={() => setError(null)}
          >
            {error}
          </Alert>
        )}

        {/* Loading State */}
        {loading ? (
          <Box display="flex" justifyContent="center" py={8}>
            <CircularProgress />
          </Box>
        ) : (
          <>
            {/* Results Count */}
            <Typography variant="body2" color="text.secondary" mb={2}>
              {filteredCommunities.length} communit{filteredCommunities.length !== 1 ? 'ies' : 'y'} found
            </Typography>

            {/* Communities Grid */}
            {filteredCommunities.length > 0 ? (
              <Grid container spacing={3}>
                {filteredCommunities.map((community) => (
                  <Grid item xs={12} sm={6} lg={4} key={community.id}>
                    <CommunityCard community={community} />
                  </Grid>
                ))}
              </Grid>
            ) : (
              <Box textAlign="center" py={8}>
                <GroupsIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
                <Typography variant="h6" color="text.secondary" gutterBottom>
                  No communities found
                </Typography>
                <Typography variant="body2" color="text.secondary" mb={3}>
                  {searchQuery || filterType !== 'all' || filterVisibility !== 'all'
                    ? 'Try adjusting your search criteria'
                    : 'Be the first to create a community in your area!'
                  }
                </Typography>
                <Button
                  variant="contained"
                  startIcon={<AddIcon />}
                  onClick={() => router.push('/communities/create')}
                >
                  Create First Community
                </Button>
              </Box>
            )}
          </>
        )}

        {/* Floating Action Button for mobile */}
        <Fab
          color="primary"
          aria-label="create community"
          sx={{
            position: 'fixed',
            bottom: 16,
            right: 16,
            display: { xs: 'flex', md: 'none' },
          }}
          onClick={() => router.push('/communities/create')}
        >
          <AddIcon />
        </Fab>
      </Box>
    </DashboardLayout>
  );
}