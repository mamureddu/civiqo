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
  Badge,
} from '@mui/material';
import {
  Museum as MuseumIcon,
  Park as ParkIcon,
  LocalLibrary as LibraryIcon,
  Church as ChurchIcon,
  AccountBalance as MonumentIcon,
  Stadium as StadiumIcon,
  TheaterComedy as TheaterIcon,
  Restaurant as RestaurantIcon,
  Add as AddIcon,
  LocationOn as LocationIcon,
  Schedule as TimeIcon,
  Phone as PhoneIcon,
  Language as WebsiteIcon,
  Search as SearchIcon,
  Map as MapIcon,
  List as ListIcon,
  FilterList as FilterIcon,
  Star as StarIcon,
  Chat as ChatIcon,
  Share as ShareIcon,
  Favorite as FavoriteIcon,
  FavoriteBorder as FavoriteBorderIcon,
  Info as InfoIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import dynamic from 'next/dynamic';
import DashboardLayout from '@/components/layout/DashboardLayout';
import apiClient from '@/lib/api-client';

// Dynamically import POI Map to avoid SSR issues
const POIMap = dynamic(() => import('@/components/map/POIMap'), {
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

// POI Categories
const poiCategories = [
  { id: 'museum', label: 'Museums', icon: <MuseumIcon /> },
  { id: 'park', label: 'Parks & Recreation', icon: <ParkIcon /> },
  { id: 'library', label: 'Libraries', icon: <LibraryIcon /> },
  { id: 'religious', label: 'Religious Sites', icon: <ChurchIcon /> },
  { id: 'monument', label: 'Monuments & Historic', icon: <MonumentIcon /> },
  { id: 'sports', label: 'Sports & Recreation', icon: <StadiumIcon /> },
  { id: 'theater', label: 'Arts & Theater', icon: <TheaterIcon /> },
  { id: 'other', label: 'Other', icon: <InfoIcon /> },
];

// POI Interface
interface PointOfInterest {
  id: string;
  name: string;
  description: string;
  category: string;
  address: string;
  latitude: number;
  longitude: number;
  phone?: string;
  website?: string;
  opening_hours?: string;
  admission_fee?: string;
  image_url?: string;
  rating?: number;
  review_count: number;
  is_verified: boolean;
  created_at: string;
  community_id: string;
}

// Mock POI data
const mockPOIs: PointOfInterest[] = [
  {
    id: '1',
    name: 'Milan Cathedral (Duomo)',
    description: 'Stunning Gothic cathedral and symbol of Milan, featuring intricate spires and beautiful stained glass.',
    category: 'monument',
    address: 'Piazza del Duomo, 20122 Milan, Italy',
    latitude: 45.4641943,
    longitude: 9.1896346,
    website: 'https://www.duomomilano.it',
    opening_hours: 'Mon-Sun: 8:00-19:00',
    admission_fee: '€3-15 (varies by area)',
    image_url: 'https://images.unsplash.com/photo-1513475382585-d06e58bcb0e0?w=400',
    rating: 4.6,
    review_count: 12847,
    is_verified: true,
    created_at: '2024-01-01T00:00:00Z',
    community_id: '1',
  },
  {
    id: '2',
    name: 'Sempione Park',
    description: 'Large public park perfect for walking, jogging, and outdoor activities. Features beautiful landscapes and historical monuments.',
    category: 'park',
    address: 'Piazza Sempione, 20154 Milan, Italy',
    latitude: 45.4715,
    longitude: 9.1738,
    opening_hours: 'Daily: 6:30-20:30',
    admission_fee: 'Free',
    image_url: 'https://images.unsplash.com/photo-1594824717313-684b769b8050?w=400',
    rating: 4.4,
    review_count: 3021,
    is_verified: true,
    created_at: '2024-01-01T00:00:00Z',
    community_id: '1',
  },
  {
    id: '3',
    name: 'Biblioteca Ambrosiana',
    description: 'Historic library and art gallery founded in 1607, housing precious manuscripts and Leonardo da Vinci\'s works.',
    category: 'library',
    address: 'Piazza Pio XI, 2, 20123 Milan, Italy',
    latitude: 45.4632,
    longitude: 9.1887,
    phone: '+39 02 806921',
    website: 'https://www.ambrosiana.it',
    opening_hours: 'Tue-Sun: 10:00-18:00',
    admission_fee: '€15',
    image_url: 'https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=400',
    rating: 4.3,
    review_count: 891,
    is_verified: true,
    created_at: '2024-01-01T00:00:00Z',
    community_id: '1',
  },
];

export default function POIPage() {
  const { data: session } = useSession();
  const user = session?.user;

  const [pois, setPois] = useState<PointOfInterest[]>(mockPOIs);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('all');
  const [viewMode, setViewMode] = useState<'list' | 'map'>('list');
  const [selectedPOI, setSelectedPOI] = useState<PointOfInterest | null>(null);
  const [favorites, setFavorites] = useState<Set<string>>(new Set());

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

  const toggleFavorite = (poiId: string) => {
    setFavorites(prev => {
      const newFavorites = new Set(prev);
      if (newFavorites.has(poiId)) {
        newFavorites.delete(poiId);
      } else {
        newFavorites.add(poiId);
      }
      return newFavorites;
    });
  };

  const filteredPOIs = pois.filter(poi => {
    const matchesSearch = poi.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         poi.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesCategory = categoryFilter === 'all' || poi.category === categoryFilter;
    return matchesSearch && matchesCategory;
  });

  const getCategoryIcon = (category: string) => {
    const categoryData = poiCategories.find(cat => cat.id === category);
    return categoryData?.icon || <InfoIcon />;
  };

  const getCategoryLabel = (category: string) => {
    const categoryData = poiCategories.find(cat => cat.id === category);
    return categoryData?.label || 'Other';
  };

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'museum': return '#8e24aa';
      case 'park': return '#388e3c';
      case 'library': return '#1976d2';
      case 'religious': return '#f57c00';
      case 'monument': return '#5d4037';
      case 'sports': return '#d32f2f';
      case 'theater': return '#7b1fa2';
      default: return '#616161';
    }
  };

  const POICard = ({ poi }: { poi: PointOfInterest }) => (
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
      {poi.image_url && (
        <CardMedia
          component="img"
          height="200"
          image={poi.image_url}
          alt={poi.name}
          sx={{ objectFit: 'cover' }}
        />
      )}

      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" alignItems="flex-start" justifyContent="space-between" mb={1}>
          <Typography variant="h6" fontWeight="bold" sx={{ flex: 1, mr: 1 }}>
            {poi.name}
          </Typography>
          <Box display="flex" alignItems="center" gap={0.5}>
            <Chip
              icon={getCategoryIcon(poi.category)}
              label={getCategoryLabel(poi.category)}
              size="small"
              sx={{
                bgcolor: getCategoryColor(poi.category),
                color: 'white',
                '& .MuiChip-icon': { color: 'white' }
              }}
            />
            {poi.is_verified && (
              <Chip
                label="Verified"
                size="small"
                color="success"
                variant="outlined"
              />
            )}
          </Box>
        </Box>

        {poi.rating && (
          <Box display="flex" alignItems="center" mb={1}>
            <Rating value={poi.rating} readOnly size="small" precision={0.1} />
            <Typography variant="body2" color="text.secondary" ml={1}>
              {poi.rating} ({poi.review_count.toLocaleString()} reviews)
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
          {poi.description}
        </Typography>

        <Stack spacing={1}>
          <Stack direction="row" spacing={1} alignItems="center">
            <LocationIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
            <Typography variant="caption" color="text.secondary">
              {poi.address}
            </Typography>
          </Stack>

          {poi.opening_hours && (
            <Stack direction="row" spacing={1} alignItems="center">
              <TimeIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
              <Typography variant="caption" color="text.secondary">
                {poi.opening_hours}
              </Typography>
            </Stack>
          )}

          {poi.admission_fee && (
            <Stack direction="row" spacing={1} alignItems="center">
              <StarIcon sx={{ fontSize: 16, color: 'warning.main' }} />
              <Typography variant="caption" color="text.secondary">
                {poi.admission_fee}
              </Typography>
            </Stack>
          )}
        </Stack>
      </CardContent>

      <CardActions sx={{ p: 2, pt: 0 }}>
        <Button
          component={Link}
          href={`/poi/${poi.id}`}
          variant="contained"
          size="small"
          sx={{ flex: 2 }}
        >
          View Details
        </Button>
        <IconButton
          onClick={() => toggleFavorite(poi.id)}
          color={favorites.has(poi.id) ? 'error' : 'default'}
          size="small"
        >
          {favorites.has(poi.id) ? <FavoriteIcon /> : <FavoriteBorderIcon />}
        </IconButton>
        <IconButton size="small">
          <ShareIcon />
        </IconButton>
      </CardActions>
    </Card>
  );

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box mb={4}>
          <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
            Points of Interest
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Discover museums, parks, libraries, and cultural sites in your community
          </Typography>
        </Box>

        {/* Search and Filters */}
        <Paper sx={{ p: 3, mb: 4 }}>
          <Grid container spacing={3} alignItems="center">
            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                placeholder="Search points of interest..."
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
                  {poiCategories.map((category) => (
                    <MenuItem key={category.id} value={category.id}>
                      <Box display="flex" alignItems="center" gap={1}>
                        {category.icon}
                        {category.label}
                      </Box>
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

        {/* Category Quick Filters */}
        <Box mb={4}>
          <Stack direction="row" spacing={1} flexWrap="wrap" gap={1}>
            <Chip
              label="All"
              onClick={() => setCategoryFilter('all')}
              color={categoryFilter === 'all' ? 'primary' : 'default'}
              variant={categoryFilter === 'all' ? 'filled' : 'outlined'}
            />
            {poiCategories.map((category) => {
              const count = pois.filter(poi => poi.category === category.id).length;
              return (
                <Badge key={category.id} badgeContent={count} color="primary">
                  <Chip
                    icon={category.icon}
                    label={category.label}
                    onClick={() => setCategoryFilter(category.id)}
                    color={categoryFilter === category.id ? 'primary' : 'default'}
                    variant={categoryFilter === category.id ? 'filled' : 'outlined'}
                  />
                </Badge>
              );
            })}
          </Stack>
        </Box>

        {/* Content */}
        <>
          {/* Results Count */}
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
            <Typography variant="body2" color="text.secondary">
              {filteredPOIs.length} point{filteredPOIs.length !== 1 ? 's' : ''} of interest found
            </Typography>

            <Button
              component={Link}
              href="/poi/create"
              variant="contained"
              startIcon={<AddIcon />}
              sx={{ display: { xs: 'none', md: 'flex' } }}
            >
              Add POI
            </Button>
          </Box>

          {viewMode === 'list' ? (
            /* List View */
            filteredPOIs.length > 0 ? (
              <Grid container spacing={3}>
                {filteredPOIs.map((poi) => (
                  <Grid item xs={12} sm={6} lg={4} key={poi.id}>
                    <POICard poi={poi} />
                  </Grid>
                ))}
              </Grid>
            ) : (
              <Box textAlign="center" py={8}>
                <MuseumIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
                <Typography variant="h6" color="text.secondary" gutterBottom>
                  No points of interest found
                </Typography>
                <Typography variant="body2" color="text.secondary" mb={3}>
                  {searchTerm || categoryFilter !== 'all'
                    ? 'Try adjusting your search criteria'
                    : 'Be the first to add a point of interest to this community!'
                  }
                </Typography>
                <Button
                  component={Link}
                  href="/poi/create"
                  variant="contained"
                  startIcon={<AddIcon />}
                >
                  Add First POI
                </Button>
              </Box>
            )
          ) : (
            /* Map View */
            <Box sx={{ height: '600px', mb: 4 }}>
              <POIMap
                pois={filteredPOIs}
                selectedPOI={selectedPOI}
                onPOISelect={setSelectedPOI}
              />
            </Box>
          )}
        </>

        {/* Floating Action Button */}
        <Fab
          color="primary"
          aria-label="add poi"
          sx={{
            position: 'fixed',
            bottom: 24,
            right: 24,
            display: { xs: 'flex', md: 'none' },
          }}
          component={Link}
          href="/poi/create"
        >
          <AddIcon />
        </Fab>
      </Container>
    </DashboardLayout>
  );
}