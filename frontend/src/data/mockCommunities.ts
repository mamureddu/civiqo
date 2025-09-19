import { Community } from '@/contexts/CommunityContext';

export const mockUserCommunities: Community[] = [
  {
    id: '1',
    name: 'Centro Milano',
    description: 'Il cuore del distretto commerciale e culturale di Milano',
    member_count: 2847,
    created_at: '2024-01-01T00:00:00Z',
    updated_at: '2024-01-01T00:00:00Z',
    subscription_status: 'supporter' as const,
    member_since: '2024-01-01T00:00:00Z',
  },
  {
    id: '2',
    name: 'Quartiere Brera',
    description: 'Centro creativo con gallerie, caffè e comunità artistiche',
    member_count: 1204,
    created_at: '2024-01-01T00:00:00Z',
    updated_at: '2024-01-01T00:00:00Z',
    subscription_status: 'free' as const,
    member_since: '2024-01-05T00:00:00Z',
  },
  {
    id: '3',
    name: 'Navigli',
    description: 'Zona storica dei canali con vita notturna e attività locali',
    member_count: 3156,
    created_at: '2024-01-01T00:00:00Z',
    updated_at: '2024-01-01T00:00:00Z',
    subscription_status: 'vip' as const,
    member_since: '2024-01-10T00:00:00Z',
  },
];