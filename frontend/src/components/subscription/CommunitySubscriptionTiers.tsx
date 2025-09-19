'use client';

import { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  CardActions,
  Typography,
  Button,
  Chip,
  Stack,
  Grid,
  Divider,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Alert,
  CircularProgress,
  Switch,
  FormControlLabel,
} from '@mui/material';
import {
  Check as CheckIcon,
  Star as StarIcon,
  Diamond as DiamondIcon,
  Stars as CrownIcon,
  Lock as LockIcon,
  Chat as ChatIcon,
  Business as BusinessIcon,
  Event as EventIcon,
  Poll as PollIcon,
  Priority as PriorityIcon,
  Support as SupportIcon,
  Badge as BadgeIcon,
  Verified as VerifiedIcon,
  Payment as PaymentIcon,
} from '@mui/icons-material';
import { useSession } from 'next-auth/react';

interface SubscriptionTier {
  id: string;
  name: string;
  price: number;
  interval: 'month' | 'year';
  description: string;
  features: string[];
  isPopular?: boolean;
  isCurrentPlan?: boolean;
  maxMembers?: number;
  icon: React.ReactNode;
  color: string;
}

interface CommunitySubscriptionTiersProps {
  communityId: string;
  communityName: string;
  currentTier?: string;
  availableTiers: SubscriptionTier[];
  onSubscribe?: (tierId: string) => void;
}

const defaultTiers: SubscriptionTier[] = [
  {
    id: 'free',
    name: 'Community Member',
    price: 0,
    interval: 'month',
    description: 'Basic community access with essential features',
    features: [
      'Join community discussions',
      'View business directory',
      'Access public events',
      'Basic chat functionality',
      'View points of interest'
    ],
    icon: <StarIcon />,
    color: '#616161',
    maxMembers: undefined
  },
  {
    id: 'supporter',
    name: 'Community Supporter',
    price: 9.99,
    interval: 'month',
    description: 'Enhanced community experience with additional perks',
    features: [
      'All free features',
      'Priority customer support',
      'Exclusive community events',
      'Enhanced chat features',
      'Community badge',
      'Early access to new features',
      'Member directory access'
    ],
    isPopular: true,
    icon: <DiamondIcon />,
    color: '#1976d2',
  },
  {
    id: 'premium',
    name: 'Community VIP',
    price: 19.99,
    interval: 'month',
    description: 'Premium access with exclusive benefits and priority',
    features: [
      'All supporter features',
      'VIP community badge',
      'Private VIP chat channels',
      'Priority in community polls',
      'Exclusive business discounts',
      'Personal community concierge',
      'Advanced networking features',
      'Monthly community meetups'
    ],
    icon: <CrownIcon />,
    color: '#9c27b0',
  }
];

export default function CommunitySubscriptionTiers({
  communityId,
  communityName,
  currentTier = 'free',
  availableTiers = defaultTiers,
  onSubscribe
}: CommunitySubscriptionTiersProps) {
  const { data: session } = useSession();
  const user = session?.user;

  const [selectedTier, setSelectedTier] = useState<SubscriptionTier | null>(null);
  const [subscriptionDialogOpen, setSubscriptionDialogOpen] = useState(false);
  const [paymentMethod, setPaymentMethod] = useState<'card' | 'paypal'>('card');
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [annualBilling, setAnnualBilling] = useState(false);

  const handleSubscribe = async (tier: SubscriptionTier) => {
    if (!user) {
      // Redirect to login
      return;
    }

    if (tier.id === 'free') {
      // Handle free tier subscription immediately
      onSubscribe?.(tier.id);
      return;
    }

    setSelectedTier(tier);
    setSubscriptionDialogOpen(true);
  };

  const handlePayment = async () => {
    if (!selectedTier) return;

    try {
      setIsProcessing(true);
      setError(null);

      // Simulate payment processing
      console.log('Processing payment for tier:', selectedTier.id);
      console.log('Payment method:', paymentMethod);
      console.log('Annual billing:', annualBilling);

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 3000));

      // Success
      onSubscribe?.(selectedTier.id);
      setSubscriptionDialogOpen(false);
      setSelectedTier(null);

    } catch (err) {
      console.error('Payment failed:', err);
      setError('Payment failed. Please try again.');
    } finally {
      setIsProcessing(false);
    }
  };

  const getDiscountedPrice = (tier: SubscriptionTier) => {
    if (annualBilling && tier.price > 0) {
      return (tier.price * 12 * 0.8); // 20% discount for annual
    }
    return tier.price;
  };

  const SubscriptionCard = ({ tier }: { tier: SubscriptionTier }) => {
    const isCurrentTier = currentTier === tier.id;
    const displayPrice = getDiscountedPrice(tier);

    return (
      <Card
        sx={{
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          position: 'relative',
          border: tier.isPopular ? 2 : 1,
          borderColor: tier.isPopular ? 'primary.main' : 'divider',
          transform: tier.isPopular ? 'scale(1.05)' : 'none',
          '&:hover': {
            transform: tier.isPopular ? 'scale(1.05)' : 'scale(1.02)',
            boxShadow: 4,
          },
          transition: 'all 0.2s ease-in-out',
        }}
      >
        {tier.isPopular && (
          <Chip
            label="Most Popular"
            color="primary"
            sx={{
              position: 'absolute',
              top: -10,
              left: '50%',
              transform: 'translateX(-50%)',
              zIndex: 1,
            }}
          />
        )}

        <CardContent sx={{ flexGrow: 1, pt: tier.isPopular ? 4 : 2 }}>
          <Box display="flex" alignItems="center" gap={1} mb={2}>
            <Box sx={{ color: tier.color }}>
              {tier.icon}
            </Box>
            <Typography variant="h6" fontWeight="bold">
              {tier.name}
            </Typography>
            {isCurrentTier && (
              <Chip
                label="Current"
                size="small"
                color="success"
                variant="outlined"
              />
            )}
          </Box>

          <Box mb={2}>
            <Box display="flex" alignItems="baseline" gap={0.5}>
              <Typography variant="h4" fontWeight="bold" color={tier.color}>
                €{annualBilling && tier.price > 0 ? (displayPrice / 12).toFixed(2) : displayPrice}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                /{tier.interval}
              </Typography>
            </Box>
            {annualBilling && tier.price > 0 && (
              <Typography variant="caption" color="success.main">
                Save 20% with annual billing
              </Typography>
            )}
          </Box>

          <Typography variant="body2" color="text.secondary" paragraph>
            {tier.description}
          </Typography>

          <Divider sx={{ my: 2 }} />

          <List dense>
            {tier.features.map((feature, index) => (
              <ListItem key={index} sx={{ px: 0 }}>
                <ListItemIcon sx={{ minWidth: 32 }}>
                  <CheckIcon color="success" fontSize="small" />
                </ListItemIcon>
                <ListItemText
                  primary={feature}
                  primaryTypographyProps={{ variant: 'body2' }}
                />
              </ListItem>
            ))}
          </List>

          {tier.maxMembers && (
            <Alert severity="info" sx={{ mt: 2 }}>
              Limited to {tier.maxMembers.toLocaleString()} members
            </Alert>
          )}
        </CardContent>

        <CardActions sx={{ p: 2, pt: 0 }}>
          <Button
            fullWidth
            variant={tier.isPopular ? 'contained' : 'outlined'}
            size="large"
            onClick={() => handleSubscribe(tier)}
            disabled={isCurrentTier}
            sx={{
              bgcolor: tier.isPopular ? tier.color : 'transparent',
              borderColor: tier.color,
              color: tier.isPopular ? 'white' : tier.color,
              '&:hover': {
                bgcolor: tier.isPopular ? tier.color : 'rgba(0,0,0,0.04)',
              }
            }}
          >
            {isCurrentTier ? 'Current Plan' : tier.price === 0 ? 'Join Community' : 'Subscribe'}
          </Button>
        </CardActions>
      </Card>
    );
  };

  return (
    <Box>
      {/* Header */}
      <Box textAlign="center" mb={4}>
        <Typography variant="h4" fontWeight="bold" gutterBottom>
          {communityName} Membership Tiers
        </Typography>
        <Typography variant="body1" color="text.secondary" paragraph>
          Choose the membership level that best fits your community engagement needs
        </Typography>

        {/* Billing Toggle */}
        <Box display="flex" justifyContent="center" mt={3}>
          <FormControlLabel
            control={
              <Switch
                checked={annualBilling}
                onChange={(e) => setAnnualBilling(e.target.checked)}
              />
            }
            label={
              <Box>
                <Typography variant="body2">
                  Annual Billing
                </Typography>
                <Typography variant="caption" color="success.main">
                  Save 20%
                </Typography>
              </Box>
            }
          />
        </Box>
      </Box>

      {/* Subscription Tiers */}
      <Grid container spacing={3} justifyContent="center">
        {availableTiers.map((tier) => (
          <Grid item xs={12} md={4} key={tier.id}>
            <SubscriptionCard tier={tier} />
          </Grid>
        ))}
      </Grid>

      {/* Payment Dialog */}
      <Dialog
        open={subscriptionDialogOpen}
        onClose={() => !isProcessing && setSubscriptionDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>
          <Box display="flex" alignItems="center" gap={2}>
            <PaymentIcon color="primary" />
            <Box>
              <Typography variant="h6" fontWeight="bold">
                Subscribe to {selectedTier?.name}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                {communityName}
              </Typography>
            </Box>
          </Box>
        </DialogTitle>

        <DialogContent>
          {selectedTier && (
            <>
              {/* Order Summary */}
              <Box
                sx={{
                  bgcolor: 'action.hover',
                  borderRadius: 2,
                  p: 2,
                  mb: 3,
                }}
              >
                <Typography variant="subtitle1" fontWeight="bold" gutterBottom>
                  Order Summary
                </Typography>
                <Box display="flex" justifyContent="space-between" mb={1}>
                  <Typography variant="body2">
                    {selectedTier.name} ({annualBilling ? 'Annual' : 'Monthly'})
                  </Typography>
                  <Typography variant="body2" fontWeight="bold">
                    €{getDiscountedPrice(selectedTier)}
                  </Typography>
                </Box>
                {annualBilling && selectedTier.price > 0 && (
                  <Box display="flex" justifyContent="space-between" mb={1}>
                    <Typography variant="caption" color="success.main">
                      Annual discount (20%)
                    </Typography>
                    <Typography variant="caption" color="success.main">
                      -€{(selectedTier.price * 12 * 0.2).toFixed(2)}
                    </Typography>
                  </Box>
                )}
              </Box>

              {/* Payment Method Selection */}
              <Typography variant="subtitle2" gutterBottom>
                Payment Method
              </Typography>
              <Grid container spacing={2} mb={3}>
                <Grid item xs={6}>
                  <Button
                    fullWidth
                    variant={paymentMethod === 'card' ? 'contained' : 'outlined'}
                    onClick={() => setPaymentMethod('card')}
                  >
                    Credit Card
                  </Button>
                </Grid>
                <Grid item xs={6}>
                  <Button
                    fullWidth
                    variant={paymentMethod === 'paypal' ? 'contained' : 'outlined'}
                    onClick={() => setPaymentMethod('paypal')}
                  >
                    PayPal
                  </Button>
                </Grid>
              </Grid>

              {/* Payment Form */}
              {paymentMethod === 'card' && (
                <Stack spacing={2} mb={3}>
                  <TextField
                    fullWidth
                    label="Card Number"
                    placeholder="1234 5678 9012 3456"
                    disabled={isProcessing}
                  />
                  <Grid container spacing={2}>
                    <Grid item xs={6}>
                      <TextField
                        fullWidth
                        label="Expiry Date"
                        placeholder="MM/YY"
                        disabled={isProcessing}
                      />
                    </Grid>
                    <Grid item xs={6}>
                      <TextField
                        fullWidth
                        label="CVV"
                        placeholder="123"
                        disabled={isProcessing}
                      />
                    </Grid>
                  </Grid>
                  <TextField
                    fullWidth
                    label="Cardholder Name"
                    placeholder="John Doe"
                    disabled={isProcessing}
                  />
                </Stack>
              )}

              {/* Error Display */}
              {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                  {error}
                </Alert>
              )}

              {/* Terms */}
              <Alert severity="info">
                <Typography variant="body2">
                  By subscribing, you agree to our Terms of Service and Privacy Policy.
                  You can cancel your subscription at any time.
                </Typography>
              </Alert>
            </>
          )}
        </DialogContent>

        <DialogActions sx={{ p: 3 }}>
          <Button
            onClick={() => setSubscriptionDialogOpen(false)}
            disabled={isProcessing}
          >
            Cancel
          </Button>
          <Button
            variant="contained"
            onClick={handlePayment}
            disabled={isProcessing}
            startIcon={isProcessing ? <CircularProgress size={16} /> : <PaymentIcon />}
          >
            {isProcessing ? 'Processing...' : `Pay €${selectedTier ? getDiscountedPrice(selectedTier) : 0}`}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}