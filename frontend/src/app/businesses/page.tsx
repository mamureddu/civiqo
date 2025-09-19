'use client';

import { useState, useEffect } from 'react';
import { useUser } from '@auth0/nextjs-auth0';
import {
  Box,
  Container,
  Grid,
  Card,
  CardContent,
  CardMedia,
  CardActions,
  Typography,
  Button,
  Chip,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  CircularProgress,
  Alert,
  Fab,
  Stack,
  Rating,
  Avatar,
  Divider,
  IconButton,
  Paper,
  ToggleButton,
  ToggleButtonGroup,
} from '@mui/material';
import {
  Business as BusinessIcon,
  Add as AddIcon,
  LocationOn as LocationIcon,
  Phone as PhoneIcon,
  Language as WebsiteIcon,
  Star as StarIcon,
  Search as SearchIcon,
  Map as MapIcon,
  List as ListIcon,
  FilterList as FilterIcon,
  OpenInNew as OpenIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import dynamic from 'next/dynamic';
import DashboardLayout from '@/components/layout/DashboardLayout';
import apiClient from '@/lib/api-client';
import type { Business, ApiResponse } from '@/types/api';

// Dynamically import the Map component to avoid SSR issues
const BusinessMap = dynamic(() => import('@/components/map/BusinessMap'), {
  ssr: false,
  loading: () => (
    <Box
      sx={{
        height: '400px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        bgcolor: 'grey.100',
      }}
    >
      <CircularProgress />
    </Box>
  ),
});

const businessCategories = [
  'Restaurant',
  'Retail',
  'Service',
  'Healthcare',
  'Technology',
  'Entertainment',
  'Education',
  'Real Estate',
  'Automotive',
  'Other',
];

export default function BusinessesPage() {
  const { user } = useUser();
  const [businesses, setBusinesses] = useState<Business[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('all');
  const [viewMode, setViewMode] = useState<'list' | 'map'>('list');
  const [selectedBusiness, setSelectedBusiness] = useState<Business | null>(null);

  const fetchBusinesses = async () => {
    try {
      setLoading(true);
      setError(null);

      const params = {
        search: searchTerm || undefined,
        category: categoryFilter !== 'all' ? categoryFilter : undefined,
        limit: 50,
      };

      const response: ApiResponse<Business[]> = await apiClient.getBusinesses(params);

      if (response.success && response.data) {
        setBusinesses(response.data);
      } else {
        setError('Failed to load businesses');
      }
    } catch (err) {
      console.error('Failed to fetch businesses:', err);
      setError('Failed to load businesses. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBusinesses();
  }, [searchTerm, categoryFilter]);

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(event.target.value);
  };

  const handleCategoryChange = (event: any) => {
    setCategoryFilter(event.target.value);
  };

  const handleViewModeChange = (event: React.MouseEvent<HTMLElement>, newViewMode: 'list' | 'map') => {
    if (newViewMode !== null) {
      setViewMode(newViewMode);
    }
  };

  const filteredBusinesses = businesses.filter(business => {
    const matchesSearch = business.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         (business.description?.toLowerCase().includes(searchTerm.toLowerCase()) ?? false);
    const matchesCategory = categoryFilter === 'all' || business.category === categoryFilter;
    return matchesSearch && matchesCategory;
  });

  const BusinessCard = ({ business }: { business: Business }) => (
    <Card
      sx={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        transition: 'transform 0.2s, box-shadow 0.2s',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow: 4,
        },
      }}
    >
      {business.image_url && (
        <CardMedia
          component="img"
          height="200"
          image={business.image_url}
          alt={business.name}
          sx={{ objectFit: 'cover' }}
        />
      )}

      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" alignItems="center" justifyContent="space-between" mb={1}>
          <Typography variant="h6" fontWeight="bold" noWrap>
            {business.name}
          </Typography>
          <Chip
            label={business.category}
            size="small"
            color="primary"
            variant="outlined"
          />
        </Box>

        {business.rating && (
          <Box display="flex" alignItems="center" mb={1}>
            <Rating value={business.rating} readOnly size="small" />
            <Typography variant="body2" color="text.secondary" ml={1}>
              ({business.rating})
            </Typography>
          </Box>
        )}

        <Typography
          variant="body2"
          color="text.secondary"
          sx={{
            mb: 2,
            display: '-webkit-box',
            overflow: 'hidden',
            WebkitBoxOrient: 'vertical',
            WebkitLineClamp: 3,
          }}
        >
          {business.description}
        </Typography>

        <Stack spacing={1}>
          {business.address && (
            <Stack direction="row" spacing={1} alignItems="center">
              <LocationIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
              <Typography variant="caption" color="text.secondary">
                {business.address}
              </Typography>
            </Stack>
          )}

          {business.phone && (
            <Stack direction="row" spacing={1} alignItems="center">
              <PhoneIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
              <Typography variant="caption" color="text.secondary">
                {business.phone}
              </Typography>
            </Stack>
          )}

          {business.website && (
            <Stack direction="row" spacing={1} alignItems="center">
              <WebsiteIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
              <Typography variant="caption" color="text.secondary" noWrap>
                {business.website}
              </Typography>
            </Stack>
          )}
        </Stack>
      </CardContent>

      <CardActions sx={{ p: 2, pt: 0 }}>
        <Button
          component={Link}
          href={`/businesses/${business.id}`}
          variant="outlined"
          size="small"
          fullWidth
        >
          View Details
        </Button>
        {business.website && (
          <IconButton
            component="a"
            href={business.website}
            target="_blank"
            rel="noopener noreferrer"
            size="small"
          >
            <OpenIcon />
          </IconButton>
        )}
      </CardActions>
    </Card>
  );

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box mb={4}>
          <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
            Local Businesses
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Discover and support local businesses in your community
          </Typography>
        </Box>

        {/* Search and Filters */}
        <Paper sx={{ p: 3, mb: 4 }}>
          <Grid container spacing={3} alignItems="center">
            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                placeholder="Search businesses..."
                value={searchTerm}
                onChange={handleSearchChange}
                InputProps={{
                  startAdornment: <SearchIcon sx={{ mr: 1, color: 'text.secondary' }} />,
                }}
              />
            </Grid>

            <Grid item xs={12} md={3}>
              <FormControl fullWidth>
                <InputLabel>Category</InputLabel>
                <Select
                  value={categoryFilter}
                  label="Category"
                  onChange={handleCategoryChange}
                >
                  <MenuItem value="all">All Categories</MenuItem>
                  {businessCategories.map((category) => (
                    <MenuItem key={category} value={category}>
                      {category}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={3}>
              <ToggleButtonGroup
                value={viewMode}
                exclusive
                onChange={handleViewModeChange}
                aria-label="view mode"
                fullWidth
              >
                <ToggleButton value="list" aria-label="list view">
                  <ListIcon sx={{ mr: 1 }} />
                  List
                </ToggleButton>
                <ToggleButton value="map" aria-label="map view">
                  <MapIcon sx={{ mr: 1 }} />
                  Map
                </ToggleButton>
              </ToggleButtonGroup>
            </Grid>

            <Grid item xs={12} md={2}>
              <Button
                fullWidth
                variant="outlined"
                onClick={() => {
                  setSearchTerm('');
                  setCategoryFilter('all');
                }}
              >
                Clear
              </Button>
            </Grid>
          </Grid>
        </Paper>

        {/* Loading State */}
        {loading && (
          <Box display="flex" justifyContent="center" py={4}>
            <CircularProgress />
          </Box>
        )}

        {/* Error State */}
        {error && (
          <Alert severity="error" sx={{ mb: 4 }}>
            {error}
          </Alert>
        )}

        {/* Content */}
        {!loading && !error && (
          <>
            {/* Results Count */}
            <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
              <Typography variant="body2" color="text.secondary">
                {filteredBusinesses.length} business{filteredBusinesses.length !== 1 ? 'es' : ''} found
              </Typography>

              <Button
                component={Link}
                href="/businesses/create"
                variant="contained"
                startIcon={<AddIcon />}
                sx={{ display: { xs: 'none', md: 'flex' } }}
              >
                Add Business
              </Button>
            </Box>

            {viewMode === 'list' ? (
              /* List View */
              filteredBusinesses.length > 0 ? (
                <Grid container spacing={3}>
                  {filteredBusinesses.map((business) => (
                    <Grid item xs={12} sm={6} lg={4} key={business.id}>
                      <BusinessCard business={business} />
                    </Grid>
                  ))}
                </Grid>
              ) : (
                <Box textAlign="center" py={8}>
                  <BusinessIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
                  <Typography variant="h6" color="text.secondary" gutterBottom>
                    No businesses found
                  </Typography>
                  <Typography variant="body2" color="text.secondary" mb={3}>
                    {searchTerm || categoryFilter !== 'all'
                      ? 'Try adjusting your search criteria'
                      : 'Be the first to add a business to this community!'
                    }
                  </Typography>
                  <Button
                    component={Link}
                    href="/businesses/create"
                    variant="contained"
                    startIcon={<AddIcon />}
                  >
                    Add First Business
                  </Button>
                </Box>
              )
            ) : (
              /* Map View */
              <Box sx={{ height: '600px', mb: 4 }}>
                <BusinessMap
                  businesses={filteredBusinesses}
                  selectedBusiness={selectedBusiness}
                  onBusinessSelect={setSelectedBusiness}
                />
              </Box>
            )}
          </>
        )}

        {/* Floating Action Button */}
        <Fab
          color="primary"
          aria-label="add business"
          sx={{
            position: 'fixed',
            bottom: 24,
            right: 24,
            display: { xs: 'flex', md: 'none' },
          }}
          component={Link}
          href="/businesses/create"
        >
          <AddIcon />
        </Fab>
      </Container>
    </DashboardLayout>
  );
}