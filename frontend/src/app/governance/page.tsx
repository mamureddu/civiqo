'use client';

import { useState, useEffect } from 'react';
import { useSession } from 'next-auth/react';
import {
  Box,
  Container,
  Grid,
  Card,
  CardContent,
  CardActions,
  Typography,
  Button,
  Chip,
  LinearProgress,
  Stack,
  Alert,
  Fab,
  Avatar,
  Divider,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  CircularProgress,
} from '@mui/material';
import {
  HowToVote as VoteIcon,
  Add as AddIcon,
  CheckCircle as CheckIcon,
  Schedule as ScheduleIcon,
  Close as CloseIcon,
  TrendingUp as TrendingUpIcon,
  People as PeopleIcon,
  Search as SearchIcon,
} from '@mui/icons-material';
import Link from 'next/link';
import DashboardLayout from '@/components/layout/DashboardLayout';
import { formatDistanceToNow, isPast } from 'date-fns';

interface Poll {
  id: string;
  title: string;
  description: string;
  options: PollOption[];
  createdBy: string;
  creatorName: string;
  creatorAvatar?: string;
  communityId: string;
  communityName: string;
  startDate: Date;
  endDate: Date;
  status: 'active' | 'completed' | 'draft';
  totalVotes: number;
  hasUserVoted: boolean;
  userVote?: string;
  category: 'policy' | 'budget' | 'event' | 'infrastructure' | 'community';
}

interface PollOption {
  id: string;
  text: string;
  votes: number;
  percentage: number;
}

// Mock data for demonstration
const mockPolls: Poll[] = [
  {
    id: '1',
    title: 'New Community Park Proposal',
    description: 'Should we allocate $50,000 from the community budget to build a new park on Main Street?',
    options: [
      { id: 'yes', text: 'Yes, build the park', votes: 127, percentage: 68 },
      { id: 'no', text: 'No, use funds elsewhere', votes: 45, percentage: 24 },
      { id: 'modify', text: 'Yes, but with modifications', votes: 15, percentage: 8 },
    ],
    createdBy: 'user1',
    creatorName: 'Sarah Johnson',
    communityId: 'comm1',
    communityName: 'Downtown Community',
    startDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 3), // 3 days ago
    endDate: new Date(Date.now() + 1000 * 60 * 60 * 24 * 4), // 4 days from now
    status: 'active',
    totalVotes: 187,
    hasUserVoted: false,
    category: 'infrastructure',
  },
  {
    id: '2',
    title: 'Annual Community Festival Date',
    description: 'When should we hold our annual community festival this year?',
    options: [
      { id: 'june', text: 'First weekend of June', votes: 89, percentage: 45 },
      { id: 'july', text: 'Third weekend of July', votes: 67, percentage: 34 },
      { id: 'august', text: 'Second weekend of August', votes: 42, percentage: 21 },
    ],
    createdBy: 'user2',
    creatorName: 'Mike Chen',
    communityId: 'comm1',
    communityName: 'Downtown Community',
    startDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 5),
    endDate: new Date(Date.now() - 1000 * 60 * 60 * 24), // Ended yesterday
    status: 'completed',
    totalVotes: 198,
    hasUserVoted: true,
    userVote: 'june',
    category: 'event',
  },
  {
    id: '3',
    title: 'Traffic Safety Improvements',
    description: 'Which traffic safety improvement should be our top priority?',
    options: [
      { id: 'lights', text: 'Install traffic lights at Oak & Main', votes: 76, percentage: 52 },
      { id: 'crosswalk', text: 'Add pedestrian crosswalks', votes: 45, percentage: 31 },
      { id: 'signs', text: 'More stop signs and speed bumps', votes: 25, percentage: 17 },
    ],
    createdBy: 'user3',
    creatorName: 'Emily Rodriguez',
    communityId: 'comm2',
    communityName: 'Westside Neighborhood',
    startDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 2),
    endDate: new Date(Date.now() + 1000 * 60 * 60 * 24 * 5),
    status: 'active',
    totalVotes: 146,
    hasUserVoted: true,
    userVote: 'lights',
    category: 'infrastructure',
  },
];

export default function GovernancePage() {
  const { data: session } = useSession();
  const user = session?.user;
  const [polls, setPolls] = useState<Poll[]>(mockPolls);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [categoryFilter, setCategoryFilter] = useState('all');

  const handleVote = async (pollId: string, optionId: string) => {
    try {
      // Simulate API call
      setPolls(prev => prev.map(poll => {
        if (poll.id === pollId) {
          const updatedOptions = poll.options.map(option => {
            if (option.id === optionId) {
              return { ...option, votes: option.votes + 1 };
            }
            return option;
          });

          const newTotalVotes = poll.totalVotes + 1;
          const updatedOptionsWithPercentage = updatedOptions.map(option => ({
            ...option,
            percentage: Math.round((option.votes / newTotalVotes) * 100),
          }));

          return {
            ...poll,
            options: updatedOptionsWithPercentage,
            totalVotes: newTotalVotes,
            hasUserVoted: true,
            userVote: optionId,
          };
        }
        return poll;
      }));
    } catch (err) {
      setError('Failed to submit vote');
    }
  };

  const filteredPolls = polls.filter(poll => {
    const matchesSearch = poll.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         poll.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || poll.status === statusFilter;
    const matchesCategory = categoryFilter === 'all' || poll.category === categoryFilter;
    return matchesSearch && matchesStatus && matchesCategory;
  });

  const PollCard = ({ poll }: { poll: Poll }) => {
    const isActive = poll.status === 'active' && !isPast(poll.endDate);
    const canVote = isActive && !poll.hasUserVoted;

    return (
      <Card
        sx={{
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          transition: 'transform 0.2s, box-shadow 0.2s',
          '&:hover': {
            transform: 'translateY(-2px)',
            boxShadow: 4,
          },
        }}
      >
        <CardContent sx={{ flexGrow: 1 }}>
          {/* Header */}
          <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={2}>
            <Stack spacing={1}>
              <Chip
                label={poll.category}
                size="small"
                color="primary"
                variant="outlined"
              />
              <Chip
                icon={poll.status === 'active' ? <ScheduleIcon /> : poll.status === 'completed' ? <CheckIcon /> : <CloseIcon />}
                label={poll.status.charAt(0).toUpperCase() + poll.status.slice(1)}
                size="small"
                color={poll.status === 'active' ? 'success' : poll.status === 'completed' ? 'info' : 'default'}
              />
            </Stack>
          </Box>

          <Typography variant="h6" fontWeight="bold" gutterBottom>
            {poll.title}
          </Typography>

          <Typography variant="body2" color="text.secondary" paragraph>
            {poll.description}
          </Typography>

          {/* Creator Info */}
          <Stack direction="row" spacing={1} alignItems="center" mb={2}>
            <Avatar
              src={poll.creatorAvatar}
              sx={{ width: 24, height: 24 }}
            >
              {poll.creatorName.charAt(0)}
            </Avatar>
            <Typography variant="caption" color="text.secondary">
              by {poll.creatorName} in {poll.communityName}
            </Typography>
          </Stack>

          <Divider sx={{ my: 2 }} />

          {/* Poll Options */}
          <Stack spacing={2}>
            {poll.options.map((option) => (
              <Box key={option.id}>
                <Stack direction="row" justifyContent="space-between" alignItems="center" mb={1}>
                  <Typography variant="body2" fontWeight={poll.userVote === option.id ? 'bold' : 'normal'}>
                    {option.text}
                    {poll.userVote === option.id && (
                      <CheckIcon sx={{ fontSize: 16, color: 'success.main', ml: 1 }} />
                    )}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {option.percentage}% ({option.votes})
                  </Typography>
                </Stack>
                <LinearProgress
                  variant="determinate"
                  value={option.percentage}
                  sx={{
                    height: 8,
                    borderRadius: 4,
                    backgroundColor: 'grey.200',
                    '& .MuiLinearProgress-bar': {
                      backgroundColor: poll.userVote === option.id ? 'success.main' : 'primary.main',
                    },
                  }}
                />
                {canVote && (
                  <Button
                    size="small"
                    variant="outlined"
                    sx={{ mt: 1 }}
                    onClick={() => handleVote(poll.id, option.id)}
                  >
                    Vote for this option
                  </Button>
                )}
              </Box>
            ))}
          </Stack>

          {/* Poll Stats */}
          <Stack direction="row" spacing={2} alignItems="center" mt={3}>
            <Stack direction="row" spacing={0.5} alignItems="center">
              <PeopleIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
              <Typography variant="caption" color="text.secondary">
                {poll.totalVotes} votes
              </Typography>
            </Stack>
            <Typography variant="caption" color="text.secondary">
              {poll.status === 'active'
                ? `Ends ${formatDistanceToNow(poll.endDate)} from now`
                : `Ended ${formatDistanceToNow(poll.endDate)} ago`
              }
            </Typography>
          </Stack>
        </CardContent>

        <CardActions sx={{ p: 2, pt: 0 }}>
          <Button
            component={Link}
            href={`/governance/polls/${poll.id}`}
            variant="outlined"
            size="small"
            fullWidth
          >
            View Details
          </Button>
        </CardActions>
      </Card>
    );
  };

  return (
    <DashboardLayout>
      <Container maxWidth="lg">
        {/* Header */}
        <Box mb={4}>
          <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
            Community Governance
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Participate in democratic decision-making for your community
          </Typography>
        </Box>

        {/* Search and Filters */}
        <Card sx={{ mb: 4 }}>
          <CardContent>
            <Grid container spacing={3} alignItems="center">
              <Grid xs={12} md={4}>
                <TextField
                  fullWidth
                  placeholder="Search polls..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  InputProps={{
                    startAdornment: <SearchIcon sx={{ mr: 1, color: 'text.secondary' }} />,
                  }}
                />
              </Grid>

              <Grid xs={12} md={3}>
                <FormControl fullWidth>
                  <InputLabel>Status</InputLabel>
                  <Select
                    value={statusFilter}
                    label="Status"
                    onChange={(e) => setStatusFilter(e.target.value)}
                  >
                    <MenuItem value="all">All Polls</MenuItem>
                    <MenuItem value="active">Active</MenuItem>
                    <MenuItem value="completed">Completed</MenuItem>
                    <MenuItem value="draft">Draft</MenuItem>
                  </Select>
                </FormControl>
              </Grid>

              <Grid xs={12} md={3}>
                <FormControl fullWidth>
                  <InputLabel>Category</InputLabel>
                  <Select
                    value={categoryFilter}
                    label="Category"
                    onChange={(e) => setCategoryFilter(e.target.value)}
                  >
                    <MenuItem value="all">All Categories</MenuItem>
                    <MenuItem value="policy">Policy</MenuItem>
                    <MenuItem value="budget">Budget</MenuItem>
                    <MenuItem value="event">Event</MenuItem>
                    <MenuItem value="infrastructure">Infrastructure</MenuItem>
                    <MenuItem value="community">Community</MenuItem>
                  </Select>
                </FormControl>
              </Grid>

              <Grid xs={12} md={2}>
                <Button
                  fullWidth
                  variant="outlined"
                  onClick={() => {
                    setSearchTerm('');
                    setStatusFilter('all');
                    setCategoryFilter('all');
                  }}
                >
                  Clear
                </Button>
              </Grid>
            </Grid>
          </CardContent>
        </Card>

        {/* Create Poll Button */}
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
          <Typography variant="h6" color="text.secondary">
            {filteredPolls.length} poll{filteredPolls.length !== 1 ? 's' : ''} found
          </Typography>
          <Button
            component={Link}
            href="/governance/create"
            variant="contained"
            startIcon={<AddIcon />}
            sx={{ display: { xs: 'none', md: 'flex' } }}
          >
            Create Poll
          </Button>
        </Box>

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

        {/* Polls Grid */}
        {!loading && (
          <>
            {filteredPolls.length > 0 ? (
              <Grid container spacing={3}>
                {filteredPolls.map((poll) => (
                  <Grid xs={12} md={6} lg={4} key={poll.id}>
                    <PollCard poll={poll} />
                  </Grid>
                ))}
              </Grid>
            ) : (
              <Box textAlign="center" py={8}>
                <VoteIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
                <Typography variant="h6" color="text.secondary" gutterBottom>
                  No polls found
                </Typography>
                <Typography variant="body2" color="text.secondary" mb={3}>
                  {searchTerm || statusFilter !== 'all' || categoryFilter !== 'all'
                    ? 'Try adjusting your search criteria'
                    : 'Be the first to create a poll for community decision-making!'
                  }
                </Typography>
                <Button
                  component={Link}
                  href="/governance/create"
                  variant="contained"
                  startIcon={<AddIcon />}
                >
                  Create First Poll
                </Button>
              </Box>
            )}
          </>
        )}

        {/* Floating Action Button */}
        <Fab
          color="primary"
          aria-label="create poll"
          sx={{
            position: 'fixed',
            bottom: 24,
            right: 24,
            display: { xs: 'flex', md: 'none' },
          }}
          component={Link}
          href="/governance/create"
        >
          <AddIcon />
        </Fab>
      </Container>
    </DashboardLayout>
  );
}