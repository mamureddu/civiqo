'use client';

import { useState, useEffect } from 'react';
import { useSession } from 'next-auth/react';
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
import ChatInitiator from '@/components/chat/ChatInitiator';
import apiClient from '@/lib/api-client';
import { useDebounce } from '@/hooks/useDebounce';
import { getBusinessImageAlt } from '@/utils/altTextHelpers';
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
  const { data: session } = useSession();
  const user = session?.user;
  const [businesses, setBusinesses] = useState<Business[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('all');
  const [viewMode, setViewMode] = useState<'list' | 'map'>('list');
  const [selectedBusiness, setSelectedBusiness] = useState<Business | null>(null);
  const [chatDialogOpen, setChatDialogOpen] = useState(false);
  const [chatBusiness, setChatBusiness] = useState<Business | null>(null);
  const [searchError, setSearchError] = useState('');

  // Debounce search term to avoid excessive filtering
  const debouncedSearchTerm = useDebounce(searchTerm, 300);

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
    const value = event.target.value;
    setSearchTerm(value);

    // Validation feedback
    if (value.length > 0 && value.length < 2) {
      setSearchError('Search term must be at least 2 characters');
    } else if (value.length > 100) {
      setSearchError('Search term is too long');
    } else {
      setSearchError('');
    }
  };

  const handleCategoryChange = (event: any) => {
    setCategoryFilter(event.target.value);
  };

  const handleViewModeChange = (event: React.MouseEvent<HTMLElement>, newViewMode: 'list' | 'map') => {
    if (newViewMode !== null) {
      setViewMode(newViewMode);
    }
  };

  const handleStartChat = (business: Business) => {
    setChatBusiness(business);
    setChatDialogOpen(true);
  };

  const handleChatStarted = (chatId: string) => {
    console.log('Chat started with ID:', chatId);
    // TODO: Navigate to chat interface or show success message
  };

  const handleChatClose = () => {
    setChatDialogOpen(false);
    setChatBusiness(null);
  };

  const filteredBusinesses = businesses.filter(business => {
    // Use debounced search term for better performance
    const matchesSearch = debouncedSearchTerm === '' ||
                         business.name.toLowerCase().includes(debouncedSearchTerm.toLowerCase()) ||
                         (business.description?.toLowerCase().includes(debouncedSearchTerm.toLowerCase()) ?? false);
    const matchesCategory = categoryFilter === 'all' || business.category === categoryFilter;
    return matchesSearch && matchesCategory;
  });

  const BusinessCard = ({ business }: { business: Business }) => (
    <Card
      className="glass hover-lift animate-scale"
      sx={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        background: 'var(--glass-bg)',
        backdropFilter: 'var(--glass-blur)',
        border: '1px solid var(--glass-border)',
        borderRadius: 3,
        overflow: 'hidden',
        position: 'relative',
        transition: 'all var(--transition-medium)',
        '&:hover': {
          transform: 'translateY(-8px) scale(1.02)',
          boxShadow: 'var(--shadow-heavy)',
          '& .business-image': {
            transform: 'scale(1.05)',
          },
          '& .gradient-overlay': {
            opacity: 0.8,
          }
        },
        '&::before': {
          content: '""',
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          height: '4px',
          background: 'linear-gradient(90deg, var(--mediterranean-500), var(--terracotta-500), var(--oliva-500))',
          zIndex: 1
        }
      }}
    >
      {business.image_url && (
        <Box sx={{ position: 'relative', overflow: 'hidden', height: 200 }}>
          <CardMedia
            component="img"
            height="200"
            image={business.image_url}
            alt={getBusinessImageAlt(business.name)}
            className="business-image"
            sx={{
              objectFit: 'cover',
              transition: 'transform var(--transition-medium)',
              position: 'relative',
              zIndex: 1
            }}
          />
          <Box
            className="gradient-overlay"
            sx={{
              position: 'absolute',
              top: 0,
              left: 0,
              right: 0,
              bottom: 0,
              background: 'linear-gradient(135deg, rgba(0, 102, 204, 0.1), rgba(230, 126, 34, 0.1))',
              opacity: 0.4,
              transition: 'opacity var(--transition-medium)',
              zIndex: 2
            }}
          />
        </Box>
      )}

      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" alignItems="center" justifyContent="space-between" mb={1}>
          <Typography
            variant="h6"
            className="font-display"
            sx={{
              fontWeight: 600,
              background: 'linear-gradient(135deg, var(--mediterranean-700), var(--mediterranean-500))',
              backgroundClip: 'text',
              WebkitBackgroundClip: 'text',
              color: 'transparent',
              fontSize: '1.1rem'
            }}
            noWrap
          >
            {business.name}
          </Typography>
          <Chip
            label={business.category}
            size="small"
            sx={{
              background: 'linear-gradient(135deg, var(--terracotta-500), var(--terracotta-600))',
              color: 'white',
              fontWeight: 500,
              border: 'none',
              boxShadow: '0 2px 8px rgba(230, 126, 34, 0.3)',
              '&:hover': {
                background: 'linear-gradient(135deg, var(--terracotta-600), var(--terracotta-700))',
                transform: 'scale(1.05)'
              }
            }}
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

        {/* Bacheca Preview with Subscription Status */}
        <Box
          className="glass-subtle"
          sx={{
            borderRadius: 2,
            p: 1.5,
            mb: 2,
            border: '1px solid var(--glass-border)',
            background: 'linear-gradient(135deg, rgba(0, 102, 204, 0.05), rgba(107, 142, 35, 0.05))',
            position: 'relative',
            overflow: 'hidden',
            '&::before': {
              content: '""',
              position: 'absolute',
              top: 0,
              left: 0,
              width: '4px',
              height: '100%',
              background: 'linear-gradient(180deg, var(--mediterranean-500), var(--oliva-500))',
            }
          }}
        >
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={0.5}>
            <Typography
              variant="caption"
              className="font-body"
              sx={{
                color: 'var(--mediterranean-600)',
                fontWeight: 600,
                display: 'flex',
                alignItems: 'center',
                gap: 0.5
              }}
            >
              📌 Latest from Bacheca
            </Typography>
            <Chip
              label="Pro"
              size="small"
              sx={{
                fontSize: '0.7rem',
                height: '20px',
                background: 'linear-gradient(135deg, var(--oliva-500), var(--oliva-600))',
                color: 'white',
                fontWeight: 600,
                boxShadow: '0 2px 4px rgba(107, 142, 35, 0.3)'
              }}
            />
          </Box>
          <Typography
            variant="caption"
            color="text.secondary"
            className="font-body"
            sx={{ fontStyle: 'italic', lineHeight: 1.4 }}
          >
            "20% off lunch specials this week!" • 2 days ago
          </Typography>
        </Box>

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

      <CardActions sx={{ p: 2, pt: 0, gap: 1 }}>
        <Button
          component={Link}
          href={`/businesses/${business.id}`}
          variant="contained"
          size="small"
          className="gradient-mediterranean hover-lift"
          sx={{
            flex: 2,
            background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
            color: 'white',
            fontWeight: 600,
            borderRadius: 2,
            textTransform: 'none',
            boxShadow: '0 4px 12px rgba(0, 102, 204, 0.3)',
            transition: 'all var(--transition-fast)',
            '&:hover': {
              background: 'linear-gradient(135deg, var(--mediterranean-600), var(--mediterranean-700))',
              boxShadow: '0 6px 20px rgba(0, 102, 204, 0.4)',
              transform: 'translateY(-2px)'
            }
          }}
        >
          View Bacheca
        </Button>
        <Button
          variant="outlined"
          size="small"
          startIcon={<ChatIcon />}
          className="hover-lift"
          sx={{
            flex: 1,
            borderColor: 'var(--terracotta-500)',
            color: 'var(--terracotta-600)',
            fontWeight: 500,
            borderRadius: 2,
            textTransform: 'none',
            borderWidth: '2px',
            transition: 'all var(--transition-fast)',
            '&:hover': {
              background: 'linear-gradient(135deg, var(--terracotta-500), var(--terracotta-600))',
              borderColor: 'var(--terracotta-600)',
              color: 'white',
              transform: 'translateY(-2px)',
              boxShadow: '0 4px 12px rgba(230, 126, 34, 0.3)'
            }
          }}
          onClick={() => handleStartChat(business)}
        >
          Chat
        </Button>
      </CardActions>
    </Card>
  );

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box mb={4} className="animate-slide-up">
          <Typography
            variant="h3"
            component="h1"
            className="font-display"
            sx={{
              fontWeight: 700,
              background: 'linear-gradient(135deg, var(--mediterranean-600), var(--terracotta-500))',
              backgroundClip: 'text',
              WebkitBackgroundClip: 'text',
              color: 'transparent',
              mb: 2,
              textShadow: '0 4px 8px rgba(0, 102, 204, 0.1)'
            }}
            gutterBottom
          >
            Local Business Network
          </Typography>
          <Typography
            variant="h6"
            className="font-body"
            sx={{
              color: 'var(--text-secondary)',
              fontWeight: 400,
              lineHeight: 1.6,
              maxWidth: '600px'
            }}
          >
            Connect with local businesses through their bacheca (bulletin boards) and start secure conversations
          </Typography>
        </Box>

        {/* Search and Filters */}
        <Paper
          className="glass hover-lift"
          sx={{
            p: 3,
            mb: 4,
            background: 'var(--glass-bg)',
            backdropFilter: 'var(--glass-blur)',
            border: '1px solid var(--glass-border)',
            borderRadius: 3,
            boxShadow: 'var(--shadow-medium)',
          }}
        >
          <Grid container spacing={3} alignItems="center">
            <Grid xs={12} md={4}>
              <TextField
                fullWidth
                placeholder="Search businesses..."
                value={searchTerm}
                onChange={handleSearchChange}
                error={!!searchError}
                helperText={searchError}
                InputProps={{
                  startAdornment: <SearchIcon sx={{ mr: 1, color: 'text.secondary' }} />,
                }}
              />
            </Grid>

            <Grid xs={12} md={3}>
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

            <Grid xs={12} md={3}>
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

            <Grid xs={12} md={2}>
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
                  {filteredBusinesses.map((business, index) => (
                    <Grid xs={12} sm={6} lg={4} key={business.id}>
                      <Box
                        sx={{
                          animationDelay: `${index * 0.1}s`,
                        }}
                        className="animate-slide-up"
                      >
                        <BusinessCard business={business} />
                      </Box>
                    </Grid>
                  ))}
                </Grid>
              ) : (
                <Box textAlign="center" py={8} className="animate-fade-in">
                  <Box
                    sx={{
                      width: 120,
                      height: 120,
                      borderRadius: '50%',
                      background: 'linear-gradient(135deg, var(--mediterranean-100), var(--terracotta-100))',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      margin: '0 auto 24px',
                      animation: 'float 3s ease-in-out infinite'
                    }}
                  >
                    <BusinessIcon sx={{ fontSize: 48, color: 'var(--mediterranean-500)' }} />
                  </Box>
                  <Typography
                    variant="h5"
                    className="font-display"
                    sx={{
                      color: 'var(--foreground)',
                      fontWeight: 600,
                      mb: 2
                    }}
                    gutterBottom
                  >
                    No businesses found
                  </Typography>
                  <Typography
                    variant="body1"
                    className="font-body"
                    sx={{
                      color: (theme) => theme.palette.text.secondary,
                      mb: 4,
                      maxWidth: '400px',
                      margin: '0 auto 32px'
                    }}
                  >
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
                    className="gradient-mediterranean hover-lift"
                    sx={{
                      background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
                      color: 'white',
                      fontWeight: 600,
                      px: 4,
                      py: 1.5,
                      borderRadius: 3,
                      textTransform: 'none',
                      fontSize: '1.1rem',
                      boxShadow: '0 4px 20px rgba(0, 102, 204, 0.3)',
                      '&:hover': {
                        background: 'linear-gradient(135deg, var(--mediterranean-600), var(--mediterranean-700))',
                        boxShadow: '0 6px 24px rgba(0, 102, 204, 0.4)',
                        transform: 'translateY(-3px)'
                      }
                    }}
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

        {/* Chat Initiator */}
        {chatBusiness && (
          <ChatInitiator
            open={chatDialogOpen}
            onClose={handleChatClose}
            recipient={{
              id: chatBusiness.id,
              name: chatBusiness.name,
              type: 'business',
              category: chatBusiness.category,
              avatar: chatBusiness.image_url
            }}
            context={{
              type: 'general'
            }}
            onChatStarted={handleChatStarted}
          />
        )}
      </Container>
    </DashboardLayout>
  );
}