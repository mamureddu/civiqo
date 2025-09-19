'use client';

import { useState, useEffect } from 'react';
import { useParams } from 'next/navigation';
import { useSession } from 'next-auth/react';
import {
  Box,
  Container,
  Grid,
  Card,
  CardContent,
  CardMedia,
  Typography,
  Button,
  Chip,
  Stack,
  Avatar,
  Divider,
  IconButton,
  TextField,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Fab,
  Paper,
  Rating,
  CircularProgress,
  Alert,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
} from '@mui/material';
import {
  LocationOn as LocationIcon,
  Phone as PhoneIcon,
  Language as WebsiteIcon,
  Schedule as ScheduleIcon,
  Chat as ChatIcon,
  Add as AddIcon,
  MoreVert as MoreIcon,
  Share as ShareIcon,
  Favorite as FavoriteIcon,
  FavoriteBorder as FavoriteBorderIcon,
  Image as ImageIcon,
  Event as EventIcon,
  LocalOffer as OfferIcon,
  Announcement as AnnouncementIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Diamond as DiamondIcon,
} from '@mui/icons-material';
import { format } from 'date-fns';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import ChatInitiator from '@/components/chat/ChatInitiator';
import apiClient from '@/lib/api-client';
import type { Business, ApiResponse } from '@/types/api';

// Bacheca Post Types
interface BachecaPost {
  id: string;
  type: 'announcement' | 'offer' | 'event' | 'image';
  title: string;
  content: string;
  image_url?: string;
  event_date?: string;
  offer_valid_until?: string;
  created_at: string;
  updated_at: string;
  business_id: string;
}

// Mock bacheca posts for now (will be replaced with real API)
const mockBachecaPosts: BachecaPost[] = [
  {
    id: '1',
    type: 'offer',
    title: '20% Off All Lunch Specials',
    content: 'Join us this week for amazing lunch deals! Fresh ingredients, great taste, unbeatable prices.',
    image_url: 'https://images.unsplash.com/photo-1565299624946-b28f40a0ca4b?w=400',
    offer_valid_until: '2024-01-15',
    created_at: '2024-01-08T10:00:00Z',
    updated_at: '2024-01-08T10:00:00Z',
    business_id: '1',
  },
  {
    id: '2',
    type: 'event',
    title: 'Live Jazz Night',
    content: 'Every Friday night featuring local musicians. Great atmosphere, delicious cocktails!',
    event_date: '2024-01-12T20:00:00Z',
    created_at: '2024-01-05T14:30:00Z',
    updated_at: '2024-01-05T14:30:00Z',
    business_id: '1',
  },
  {
    id: '3',
    type: 'announcement',
    title: 'New Extended Hours',
    content: 'We\'re now open until 11 PM on weekends to serve you better!',
    created_at: '2024-01-03T09:15:00Z',
    updated_at: '2024-01-03T09:15:00Z',
    business_id: '1',
  },
];

export default function BusinessDetailPage() {
  const params = useParams();
  const businessId = params.id as string;
  const { data: session } = useSession();
  const user = session?.user;

  const [business, setBusiness] = useState<Business | null>(null);
  const [bachecaPosts, setBachecaPosts] = useState<BachecaPost[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isFavorite, setIsFavorite] = useState(false);
  const [chatDialogOpen, setChatDialogOpen] = useState(false);
  const [selectedPost, setSelectedPost] = useState<BachecaPost | null>(null);
  const [postMenuAnchor, setPostMenuAnchor] = useState<null | HTMLElement>(null);
  const [selectedPostId, setSelectedPostId] = useState<string | null>(null);

  // Fetch business data
  useEffect(() => {
    const fetchBusiness = async () => {
      try {
        setLoading(true);
        setError(null);

        const response: ApiResponse<Business> = await apiClient.getBusiness(businessId);

        if (response.success && response.data) {
          setBusiness(response.data);
          // For now, use mock posts. Later connect to real API
          setBachecaPosts(mockBachecaPosts);
        } else {
          setError('Business not found');
        }
      } catch (err) {
        console.error('Failed to fetch business:', err);
        setError('Failed to load business details. Please try again.');
      } finally {
        setLoading(false);
      }
    };

    if (businessId) {
      fetchBusiness();
    }
  }, [businessId]);

  const handleStartChat = (post?: BachecaPost) => {
    setSelectedPost(post || null);
    setChatDialogOpen(true);
  };

  const handleChatStarted = (chatId: string) => {
    console.log('Chat started with ID:', chatId);
    // TODO: Navigate to chat interface or show success message
  };

  const handleChatClose = () => {
    setChatDialogOpen(false);
    setSelectedPost(null);
  };

  const handleToggleFavorite = () => {
    setIsFavorite(!isFavorite);
  };

  const handlePostMenuClick = (event: React.MouseEvent<HTMLElement>, postId: string) => {
    setPostMenuAnchor(event.currentTarget);
    setSelectedPostId(postId);
  };

  const handlePostMenuClose = () => {
    setPostMenuAnchor(null);
    setSelectedPostId(null);
  };

  const getPostIcon = (type: BachecaPost['type']) => {
    switch (type) {
      case 'announcement': return <AnnouncementIcon />;
      case 'offer': return <OfferIcon />;
      case 'event': return <EventIcon />;
      case 'image': return <ImageIcon />;
      default: return <AnnouncementIcon />;
    }
  };

  const getPostColor = (type: BachecaPost['type']) => {
    switch (type) {
      case 'announcement': return 'info';
      case 'offer': return 'success';
      case 'event': return 'warning';
      case 'image': return 'secondary';
      default: return 'default';
    }
  };

  const BachecaPostCard = ({ post }: { post: BachecaPost }) => (
    <Card sx={{ mb: 3 }}>
      {post.image_url && (
        <CardMedia
          component="img"
          height="200"
          image={post.image_url}
          alt={post.title}
          sx={{ objectFit: 'cover' }}
        />
      )}

      <CardContent>
        <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={2}>
          <Box display="flex" alignItems="center" gap={1}>
            <Chip
              icon={getPostIcon(post.type)}
              label={post.type.charAt(0).toUpperCase() + post.type.slice(1)}
              color={getPostColor(post.type) as any}
              variant="outlined"
              size="small"
            />
            <Typography variant="caption" color="text.secondary">
              {format(new Date(post.created_at), 'MMM dd, yyyy')}
            </Typography>
          </Box>

          <IconButton
            size="small"
            onClick={(e) => handlePostMenuClick(e, post.id)}
          >
            <MoreIcon />
          </IconButton>
        </Box>

        <Typography variant="h6" fontWeight="bold" gutterBottom>
          {post.title}
        </Typography>

        <Typography variant="body2" color="text.secondary" paragraph>
          {post.content}
        </Typography>

        {/* Special fields based on post type */}
        {post.type === 'event' && post.event_date && (
          <Box display="flex" alignItems="center" gap={1} mb={1}>
            <EventIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
            <Typography variant="caption" color="text.secondary">
              {format(new Date(post.event_date), 'PPP p')}
            </Typography>
          </Box>
        )}

        {post.type === 'offer' && post.offer_valid_until && (
          <Box display="flex" alignItems="center" gap={1} mb={1}>
            <OfferIcon sx={{ fontSize: 16, color: 'success.main' }} />
            <Typography variant="caption" color="success.main">
              Valid until {format(new Date(post.offer_valid_until), 'PPP')}
            </Typography>
          </Box>
        )}

        <Divider sx={{ my: 2 }} />

        <Box display="flex" justifyContent="space-between" alignItems="center">
          <Button
            variant="contained"
            startIcon={<ChatIcon />}
            onClick={() => handleStartChat(post)}
            size="small"
          >
            Chat About This
          </Button>

          <Box>
            <IconButton onClick={handleToggleFavorite} size="small">
              {isFavorite ? (
                <FavoriteIcon color="error" />
              ) : (
                <FavoriteBorderIcon />
              )}
            </IconButton>
            <IconButton size="small">
              <ShareIcon />
            </IconButton>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );

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

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Business Header */}
        <Paper elevation={2} sx={{ p: 4, mb: 4 }}>
          <Grid container spacing={4}>
            <Grid xs={12} md={8}>
              <Box display="flex" alignItems="flex-start" gap={3}>
                {business.image_url && (
                  <Avatar
                    src={business.image_url}
                    alt={business.name}
                    sx={{ width: 80, height: 80 }}
                    variant="rounded"
                  />
                )}

                <Box flex={1}>
                  <Box display="flex" alignItems="center" gap={2} mb={2}>
                    <Typography variant="h4" fontWeight="bold">
                      {business.name}
                    </Typography>
                    <Chip
                      label={business.category}
                      color="primary"
                      variant="outlined"
                    />
                  </Box>

                  {business.rating && (
                    <Box display="flex" alignItems="center" mb={2}>
                      <Rating value={business.rating} readOnly />
                      <Typography variant="body2" color="text.secondary" ml={1}>
                        ({business.rating} stars)
                      </Typography>
                    </Box>
                  )}

                  <Typography variant="body1" color="text.secondary" paragraph>
                    {business.description}
                  </Typography>

                  <Stack spacing={1}>
                    {business.address && (
                      <Box display="flex" alignItems="center" gap={1}>
                        <LocationIcon sx={{ fontSize: 18, color: 'text.secondary' }} />
                        <Typography variant="body2">{business.address}</Typography>
                      </Box>
                    )}

                    {business.phone && (
                      <Box display="flex" alignItems="center" gap={1}>
                        <PhoneIcon sx={{ fontSize: 18, color: 'text.secondary' }} />
                        <Typography variant="body2">{business.phone}</Typography>
                      </Box>
                    )}

                    {business.website && (
                      <Box display="flex" alignItems="center" gap={1}>
                        <WebsiteIcon sx={{ fontSize: 18, color: 'text.secondary' }} />
                        <Typography
                          variant="body2"
                          component="a"
                          href={business.website}
                          target="_blank"
                          rel="noopener noreferrer"
                          sx={{ textDecoration: 'none', color: 'primary.main' }}
                        >
                          {business.website}
                        </Typography>
                      </Box>
                    )}
                  </Stack>
                </Box>
              </Box>
            </Grid>

            <Grid xs={12} md={4}>
              <Stack spacing={2}>
                <Button
                  variant="contained"
                  size="large"
                  startIcon={<ChatIcon />}
                  onClick={handleStartChat}
                  fullWidth
                >
                  Contact Business
                </Button>

                <Box display="flex" gap={1}>
                  <Button
                    variant="outlined"
                    startIcon={isFavorite ? <FavoriteIcon /> : <FavoriteBorderIcon />}
                    onClick={handleToggleFavorite}
                    flex={1}
                  >
                    {isFavorite ? 'Favorited' : 'Favorite'}
                  </Button>
                  <Button
                    variant="outlined"
                    startIcon={<ShareIcon />}
                    flex={1}
                  >
                    Share
                  </Button>
                </Box>
              </Stack>
            </Grid>
          </Grid>
        </Paper>

        {/* Bacheca Section */}
        <Box mb={4}>
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
            <Typography variant="h5" fontWeight="bold">
              Bacheca (Business Board)
            </Typography>
            <Typography variant="body2" color="text.secondary">
              {bachecaPosts.length} post{bachecaPosts.length !== 1 ? 's' : ''}
            </Typography>
          </Box>

          {bachecaPosts.length > 0 ? (
            <Grid container spacing={3}>
              <Grid xs={12} md={8}>
                {bachecaPosts.map((post) => (
                  <BachecaPostCard key={post.id} post={post} />
                ))}
              </Grid>

              <Grid xs={12} md={4}>
                <Paper sx={{ p: 3, position: 'sticky', top: 20 }}>
                  <Typography variant="h6" fontWeight="bold" gutterBottom>
                    About This Business
                  </Typography>
                  <Typography variant="body2" color="text.secondary" paragraph>
                    Connect directly with {business.name} through our secure chat system.
                    All conversations are end-to-end encrypted for your privacy.
                  </Typography>

                  <Stack spacing={2}>
                    <Button
                      variant="outlined"
                      startIcon={<ChatIcon />}
                      onClick={handleStartChat}
                      fullWidth
                    >
                      Start Conversation
                    </Button>

                    <Divider />

                    <Box>
                      <Typography variant="subtitle2" fontWeight="bold" gutterBottom>
                        Business Subscription
                      </Typography>
                      <Stack direction="row" spacing={1} alignItems="center" mb={1}>
                        <Chip
                          label="Professional"
                          color="success"
                          size="small"
                          icon={<DiamondIcon />}
                        />
                        <Typography variant="caption" color="text.secondary">
                          Active
                        </Typography>
                      </Stack>
                      <Typography variant="caption" color="text.secondary" display="block" mb={2}>
                        Premium bacheca features enabled
                      </Typography>
                      <Button
                        component={Link}
                        href={`/businesses/${businessId}/subscription`}
                        variant="outlined"
                        size="small"
                        fullWidth
                      >
                        Manage Subscription
                      </Button>
                    </Box>
                  </Stack>
                </Paper>
              </Grid>
            </Grid>
          ) : (
            <Paper sx={{ p: 6, textAlign: 'center' }}>
              <AnnouncementIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
              <Typography variant="h6" color="text.secondary" gutterBottom>
                No posts yet
              </Typography>
              <Typography variant="body2" color="text.secondary">
                This business hasn't posted anything on their bacheca yet.
              </Typography>
            </Paper>
          )}
        </Box>

        {/* Chat Initiator */}
        {business && (
          <ChatInitiator
            open={chatDialogOpen}
            onClose={handleChatClose}
            recipient={{
              id: business.id,
              name: business.name,
              type: 'business',
              category: business.category,
              avatar: business.image_url
            }}
            context={selectedPost ? {
              type: 'bacheca_post',
              title: selectedPost.title,
              postId: selectedPost.id
            } : {
              type: 'general'
            }}
            onChatStarted={handleChatStarted}
          />
        )}

        {/* Post Actions Menu */}
        <Menu
          anchorEl={postMenuAnchor}
          open={Boolean(postMenuAnchor)}
          onClose={handlePostMenuClose}
        >
          <MenuItem onClick={handlePostMenuClose}>
            <ListItemIcon>
              <ShareIcon fontSize="small" />
            </ListItemIcon>
            <ListItemText>Share Post</ListItemText>
          </MenuItem>
          {user && (
            <MenuItem
              onClick={() => {
                const post = bachecaPosts.find(p => p.id === selectedPostId);
                if (post) handleStartChat(post);
                handlePostMenuClose();
              }}
            >
              <ListItemIcon>
                <ChatIcon fontSize="small" />
              </ListItemIcon>
              <ListItemText>Contact About This</ListItemText>
            </MenuItem>
          )}
        </Menu>

        {/* Floating Action Button for Business Owners */}
        {user && (
          <Fab
            color="primary"
            aria-label="add post"
            sx={{
              position: 'fixed',
              bottom: 24,
              right: 24,
            }}
            component={Link}
            href={`/businesses/${businessId}/create-post`}
          >
            <AddIcon />
          </Fab>
        )}
      </Container>
    </DashboardLayout>
  );
}