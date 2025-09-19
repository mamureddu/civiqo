'use client';

import { useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useSession } from 'next-auth/react';
import {
  Box,
  Container,
  Card,
  CardContent,
  Typography,
  TextField,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Grid,
  Chip,
  Stack,
  Alert,
  IconButton,
  Paper,
  Divider,
  Switch,
  FormControlLabel,
} from '@mui/material';
import {
  ArrowBack as BackIcon,
  Image as ImageIcon,
  Event as EventIcon,
  LocalOffer as OfferIcon,
  Announcement as AnnouncementIcon,
  Upload as UploadIcon,
  Preview as PreviewIcon,
} from '@mui/icons-material';
import { DateTimePicker } from '@mui/x-date-pickers/DateTimePicker';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import { format } from 'date-fns';

interface BachecaPostForm {
  type: 'announcement' | 'offer' | 'event' | 'image';
  title: string;
  content: string;
  image_url?: string;
  event_date?: Date | null;
  offer_valid_until?: Date | null;
  is_pinned: boolean;
  notify_followers: boolean;
}

export default function CreateBachecaPostPage() {
  const params = useParams();
  const router = useRouter();
  const businessId = params.id as string;
  const { data: session } = useSession();

  const [formData, setFormData] = useState<BachecaPostForm>({
    type: 'announcement',
    title: '',
    content: '',
    image_url: '',
    event_date: null,
    offer_valid_until: null,
    is_pinned: false,
    notify_followers: true,
  });

  const [isSubmitting, setIsSubmitting] = useState(false);
  const [previewMode, setPreviewMode] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleInputChange = (field: keyof BachecaPostForm, value: any) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
    if (error) setError(null);
  };

  const handleImageUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      // TODO: Implement actual image upload to S3
      // For now, use a placeholder
      const imageUrl = URL.createObjectURL(file);
      handleInputChange('image_url', imageUrl);
    }
  };

  const handleSubmit = async () => {
    try {
      setIsSubmitting(true);
      setError(null);

      // Validate required fields
      if (!formData.title.trim() || !formData.content.trim()) {
        setError('Title and content are required');
        return;
      }

      if (formData.type === 'event' && !formData.event_date) {
        setError('Event date is required for event posts');
        return;
      }

      if (formData.type === 'offer' && !formData.offer_valid_until) {
        setError('Valid until date is required for offer posts');
        return;
      }

      // TODO: Implement API call to create bacheca post
      console.log('Creating bacheca post:', formData);

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Redirect back to business page
      router.push(`/businesses/${businessId}`);
    } catch (err) {
      console.error('Failed to create post:', err);
      setError('Failed to create post. Please try again.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const getPostTypeIcon = (type: string) => {
    switch (type) {
      case 'announcement': return <AnnouncementIcon />;
      case 'offer': return <OfferIcon />;
      case 'event': return <EventIcon />;
      case 'image': return <ImageIcon />;
      default: return <AnnouncementIcon />;
    }
  };

  const getPostTypeColor = (type: string) => {
    switch (type) {
      case 'announcement': return 'info';
      case 'offer': return 'success';
      case 'event': return 'warning';
      case 'image': return 'secondary';
      default: return 'default';
    }
  };

  const PostPreview = () => (
    <Card sx={{ mt: 3 }}>
      <CardContent>
        <Typography variant="h6" color="text.secondary" gutterBottom>
          Preview
        </Typography>

        {formData.image_url && (
          <Box
            component="img"
            src={formData.image_url}
            alt="Post image"
            sx={{
              width: '100%',
              height: 200,
              objectFit: 'cover',
              borderRadius: 1,
              mb: 2
            }}
          />
        )}

        <Box display="flex" alignItems="center" gap={1} mb={2}>
          <Chip
            icon={getPostTypeIcon(formData.type)}
            label={formData.type.charAt(0).toUpperCase() + formData.type.slice(1)}
            color={getPostTypeColor(formData.type) as any}
            variant="outlined"
            size="small"
          />
          <Typography variant="caption" color="text.secondary">
            {format(new Date(), 'MMM dd, yyyy')}
          </Typography>
          {formData.is_pinned && (
            <Chip label="Pinned" size="small" color="primary" />
          )}
        </Box>

        <Typography variant="h6" fontWeight="bold" gutterBottom>
          {formData.title || 'Post title will appear here'}
        </Typography>

        <Typography variant="body2" color="text.secondary" paragraph>
          {formData.content || 'Post content will appear here'}
        </Typography>

        {formData.type === 'event' && formData.event_date && (
          <Box display="flex" alignItems="center" gap={1} mb={1}>
            <EventIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
            <Typography variant="caption" color="text.secondary">
              {format(formData.event_date, 'PPP p')}
            </Typography>
          </Box>
        )}

        {formData.type === 'offer' && formData.offer_valid_until && (
          <Box display="flex" alignItems="center" gap={1} mb={1}>
            <OfferIcon sx={{ fontSize: 16, color: 'success.main' }} />
            <Typography variant="caption" color="success.main">
              Valid until {format(formData.offer_valid_until, 'PPP')}
            </Typography>
          </Box>
        )}
      </CardContent>
    </Card>
  );

  return (
    <DashboardLayout>
      <Container maxWidth="md">
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
              Create Bacheca Post
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Share updates, offers, and events with your community
            </Typography>
          </Box>
        </Box>

        {error && (
          <Alert severity="error" sx={{ mb: 3 }}>
            {error}
          </Alert>
        )}

        <Paper sx={{ p: 4 }}>
          <Grid container spacing={3}>
            {/* Post Type Selection */}
            <Grid item xs={12}>
              <FormControl fullWidth>
                <InputLabel>Post Type</InputLabel>
                <Select
                  value={formData.type}
                  label="Post Type"
                  onChange={(e) => handleInputChange('type', e.target.value)}
                >
                  <MenuItem value="announcement">
                    <Box display="flex" alignItems="center" gap={1}>
                      <AnnouncementIcon fontSize="small" />
                      Announcement
                    </Box>
                  </MenuItem>
                  <MenuItem value="offer">
                    <Box display="flex" alignItems="center" gap={1}>
                      <OfferIcon fontSize="small" />
                      Special Offer
                    </Box>
                  </MenuItem>
                  <MenuItem value="event">
                    <Box display="flex" alignItems="center" gap={1}>
                      <EventIcon fontSize="small" />
                      Event
                    </Box>
                  </MenuItem>
                  <MenuItem value="image">
                    <Box display="flex" alignItems="center" gap={1}>
                      <ImageIcon fontSize="small" />
                      Image Post
                    </Box>
                  </MenuItem>
                </Select>
              </FormControl>
            </Grid>

            {/* Title */}
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Post Title"
                value={formData.title}
                onChange={(e) => handleInputChange('title', e.target.value)}
                placeholder="Enter an engaging title for your post"
                required
              />
            </Grid>

            {/* Content */}
            <Grid item xs={12}>
              <TextField
                fullWidth
                multiline
                rows={4}
                label="Post Content"
                value={formData.content}
                onChange={(e) => handleInputChange('content', e.target.value)}
                placeholder="Write your post content here..."
                required
              />
            </Grid>

            {/* Image Upload */}
            <Grid item xs={12}>
              <Box>
                <Typography variant="subtitle2" gutterBottom>
                  Image (Optional)
                </Typography>
                <Button
                  variant="outlined"
                  component="label"
                  startIcon={<UploadIcon />}
                  sx={{ mb: 2 }}
                >
                  Upload Image
                  <input
                    type="file"
                    hidden
                    accept="image/*"
                    onChange={handleImageUpload}
                  />
                </Button>
                {formData.image_url && (
                  <Box
                    component="img"
                    src={formData.image_url}
                    alt="Uploaded"
                    sx={{
                      width: 200,
                      height: 120,
                      objectFit: 'cover',
                      borderRadius: 1,
                      border: 1,
                      borderColor: 'divider'
                    }}
                  />
                )}
              </Box>
            </Grid>

            {/* Event Date (for events) */}
            {formData.type === 'event' && (
              <Grid item xs={12} md={6}>
                <LocalizationProvider dateAdapter={AdapterDateFns}>
                  <DateTimePicker
                    label="Event Date & Time"
                    value={formData.event_date}
                    onChange={(date) => handleInputChange('event_date', date)}
                    slotProps={{
                      textField: {
                        fullWidth: true,
                        required: true
                      }
                    }}
                  />
                </LocalizationProvider>
              </Grid>
            )}

            {/* Offer Valid Until (for offers) */}
            {formData.type === 'offer' && (
              <Grid item xs={12} md={6}>
                <LocalizationProvider dateAdapter={AdapterDateFns}>
                  <DateTimePicker
                    label="Valid Until"
                    value={formData.offer_valid_until}
                    onChange={(date) => handleInputChange('offer_valid_until', date)}
                    slotProps={{
                      textField: {
                        fullWidth: true,
                        required: true
                      }
                    }}
                  />
                </LocalizationProvider>
              </Grid>
            )}

            {/* Options */}
            <Grid item xs={12}>
              <Divider sx={{ my: 2 }} />
              <Typography variant="subtitle2" gutterBottom>
                Post Options
              </Typography>
              <Stack spacing={1}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={formData.is_pinned}
                      onChange={(e) => handleInputChange('is_pinned', e.target.checked)}
                    />
                  }
                  label="Pin this post to the top of your bacheca"
                />
                <FormControlLabel
                  control={
                    <Switch
                      checked={formData.notify_followers}
                      onChange={(e) => handleInputChange('notify_followers', e.target.checked)}
                    />
                  }
                  label="Notify followers about this post"
                />
              </Stack>
            </Grid>

            {/* Actions */}
            <Grid item xs={12}>
              <Divider sx={{ my: 2 }} />
              <Stack direction="row" spacing={2} justifyContent="space-between">
                <Button
                  variant="outlined"
                  startIcon={<PreviewIcon />}
                  onClick={() => setPreviewMode(!previewMode)}
                >
                  {previewMode ? 'Hide Preview' : 'Show Preview'}
                </Button>

                <Box>
                  <Button
                    component={Link}
                    href={`/businesses/${businessId}`}
                    sx={{ mr: 2 }}
                  >
                    Cancel
                  </Button>
                  <Button
                    variant="contained"
                    onClick={handleSubmit}
                    disabled={isSubmitting}
                  >
                    {isSubmitting ? 'Publishing...' : 'Publish Post'}
                  </Button>
                </Box>
              </Stack>
            </Grid>
          </Grid>

          {/* Preview */}
          {previewMode && <PostPreview />}
        </Paper>
      </Container>
    </DashboardLayout>
  );
}