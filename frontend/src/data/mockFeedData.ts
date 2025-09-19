interface FeedItem {
  id: string;
  type: 'business_post' | 'community_announcement' | 'new_member' | 'poll' | 'event' | 'chat_activity';
  title: string;
  content: string;
  author: {
    id: string;
    name: string;
    avatar?: string;
    type: 'business' | 'member' | 'admin';
  };
  timestamp: string;
  metadata?: {
    business_id?: string;
    event_date?: string;
    poll_id?: string;
    member_count?: number;
    engagement_count?: number;
  };
  image_url?: string;
  actions?: {
    canLike: boolean;
    canShare: boolean;
    canComment: boolean;
  };
}

export const mockFeedItems: FeedItem[] = [
  {
    id: '1',
    type: 'business_post',
    title: 'community.feed.businessPost.specialOffer',
    content: 'community.feed.businessPost.specialOfferContent',
    author: {
      id: 'business-1',
      name: 'Milano Coffee Roasters',
      avatar: 'https://images.unsplash.com/photo-1447933601403-0c6688de566e?w=100',
      type: 'business'
    },
    timestamp: '2024-01-15T10:30:00Z',
    metadata: {
      business_id: 'business-1',
      engagement_count: 12
    },
    image_url: 'https://images.unsplash.com/photo-1565299624946-b28f40a0ca4b?w=400',
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '2',
    type: 'community_announcement',
    title: 'community.feed.announcement.newGuidelines',
    content: 'community.feed.announcement.newGuidelinesContent',
    author: {
      id: 'admin-1',
      name: 'community.feed.author.administration',
      type: 'admin'
    },
    timestamp: '2024-01-15T09:15:00Z',
    metadata: {
      engagement_count: 45
    },
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '3',
    type: 'new_member',
    title: 'community.feed.newMembers.title',
    content: 'community.feed.newMembers.content',
    author: {
      id: 'system',
      name: 'community.feed.author.system',
      type: 'admin'
    },
    timestamp: '2024-01-15T08:45:00Z',
    metadata: {
      member_count: 5
    },
    actions: {
      canLike: true,
      canShare: false,
      canComment: true
    }
  },
  {
    id: '4',
    type: 'event',
    title: 'community.feed.event.monthlyMeeting',
    content: 'community.feed.event.monthlyMeetingContent',
    author: {
      id: 'admin-1',
      name: 'community.feed.author.administration',
      type: 'admin'
    },
    timestamp: '2024-01-14T16:20:00Z',
    metadata: {
      event_date: '2024-01-16T19:00:00Z',
      engagement_count: 28
    },
    actions: {
      canLike: true,
      canShare: true,
      canComment: true
    }
  },
  {
    id: '5',
    type: 'poll',
    title: 'community.feed.poll.newPark',
    content: 'community.feed.poll.newParkContent',
    author: {
      id: 'admin-1',
      name: 'community.feed.author.administration',
      type: 'admin'
    },
    timestamp: '2024-01-14T14:10:00Z',
    metadata: {
      poll_id: 'poll-1',
      engagement_count: 67
    },
    actions: {
      canLike: false,
      canShare: true,
      canComment: true
    }
  }
];

export const mockStats = {
  communities: 3,
  businesses: 127,
  activePolls: 5,
  unreadMessages: 12,
};

export const mockRecentActivity = [
  {
    id: '1',
    titleKey: 'community.feed.activity.newBusiness',
    descriptionKey: 'community.feed.activity.newBusinessDesc',
    time: 'common.time.hoursAgo',
    timeValue: 2,
  },
  {
    id: '2',
    titleKey: 'community.feed.activity.newPoll',
    descriptionKey: 'community.feed.activity.newPollDesc',
    time: 'common.time.hoursAgo',
    timeValue: 4,
  },
  {
    id: '3',
    titleKey: 'community.feed.activity.newMembers',
    descriptionKey: 'community.feed.activity.newMembersDesc',
    time: 'common.time.daysAgo',
    timeValue: 1,
  },
];

export type { FeedItem };