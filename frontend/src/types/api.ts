// API Response Types matching backend shared models

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  error?: {
    message: string;
    code?: string;
  };
}

export interface PaginationParams {
  limit?: number;
  offset?: number;
}

// User Types
export interface User {
  id: string;
  auth0_id: string;
  email: string;
  email_verified: boolean;
  name?: string;
  nickname?: string;
  picture?: string;
  locale?: string;
  created_at: string;
  updated_at: string;
}

export interface UserProfile {
  user_id: string;
  display_name?: string;
  bio?: string;
  location?: string;
  website?: string;
  avatar_url?: string;
  preferences: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export interface UserWithProfile {
  user: User;
  profile?: UserProfile;
}

// Community Types
export interface Community {
  id: string;
  name: string;
  description?: string;
  community_type: CommunityType;
  is_public: boolean;
  location?: string;
  latitude?: number;
  longitude?: number;
  member_count: number;
  owner_id: string;
  settings: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export enum CommunityType {
  Geographic = 'geographic',
  Interest = 'interest',
  Organization = 'organization',
  Event = 'event'
}

export interface CommunityMember {
  id: string;
  community_id: string;
  user_id: string;
  role: CommunityRole;
  permissions: string[];
  joined_at: string;
  last_active_at?: string;
}

export enum CommunityRole {
  Member = 'member',
  Moderator = 'moderator',
  Admin = 'admin',
  Owner = 'owner'
}

export interface CreateCommunityRequest {
  name: string;
  description?: string;
  community_type: CommunityType;
  is_public: boolean;
  location?: string;
  latitude?: number;
  longitude?: number;
}

// Business Types
export interface Business {
  id: string;
  community_id: string;
  owner_id: string;
  name: string;
  description?: string;
  category: BusinessCategory;
  website?: string;
  phone?: string;
  email?: string;
  address?: string;
  latitude?: number;
  longitude?: number;
  is_verified: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export enum BusinessCategory {
  Food = 'food',
  Retail = 'retail',
  Services = 'services',
  Healthcare = 'healthcare',
  Education = 'education',
  Technology = 'technology',
  Manufacturing = 'manufacturing',
  Agriculture = 'agriculture',
  Arts = 'arts',
  Recreation = 'recreation',
  Other = 'other'
}

export interface BusinessProduct {
  id: string;
  business_id: string;
  name: string;
  description?: string;
  price?: number;
  currency: string;
  unit?: string;
  is_available: boolean;
  image_url?: string;
  created_at: string;
  updated_at: string;
}

export interface BusinessSearchResult {
  business: Business;
  distance_km?: number;
  product_count: number;
}

export interface CreateBusinessRequest {
  name: string;
  description?: string;
  category: BusinessCategory;
  website?: string;
  phone?: string;
  email?: string;
  address?: string;
  location?: Point;
}

export interface Point {
  latitude: number;
  longitude: number;
}

// Governance Types
export interface Poll {
  id: string;
  community_id: string;
  creator_id: string;
  title: string;
  description?: string;
  options: string[];
  poll_type: PollType;
  end_date: string;
  is_anonymous: boolean;
  requires_verification: boolean;
  status: PollStatus;
  created_at: string;
  updated_at: string;
}

export enum PollType {
  SingleChoice = 'single_choice',
  MultipleChoice = 'multiple_choice',
  RankedChoice = 'ranked_choice'
}

export enum PollStatus {
  Draft = 'draft',
  Active = 'active',
  Closed = 'closed',
  Cancelled = 'cancelled'
}

export interface Vote {
  id: string;
  poll_id: string;
  user_id: string;
  selected_options: number[];
  comment?: string;
  is_verified: boolean;
  created_at: string;
}

export interface Decision {
  id: string;
  community_id: string;
  creator_id: string;
  title: string;
  description: string;
  decision_type: DecisionType;
  status: DecisionStatus;
  impact_assessment?: string;
  implementation_plan?: string;
  budget_impact?: number;
  stakeholders: string[];
  deadline?: string;
  created_at: string;
  updated_at: string;
}

export enum DecisionType {
  Policy = 'policy',
  Budget = 'budget',
  Infrastructure = 'infrastructure',
  Administrative = 'administrative',
  Event = 'event'
}

export enum DecisionStatus {
  Proposed = 'proposed',
  UnderReview = 'under_review',
  Approved = 'approved',
  Rejected = 'rejected',
  Implemented = 'implemented'
}

// Chat Types
export interface ChatRoom {
  id: string;
  community_id: string;
  name: string;
  description?: string;
  room_type: ChatRoomType;
  is_public: boolean;
  max_participants?: number;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export enum ChatRoomType {
  General = 'general',
  Topic = 'topic',
  Private = 'private',
  Announcement = 'announcement'
}

export interface ChatMessage {
  id: string;
  room_id: string;
  sender_id: string;
  content: string;
  message_type: MessageType;
  encrypted_content?: string;
  reply_to?: string;
  created_at: string;
}

export enum MessageType {
  Text = 'text',
  Image = 'image',
  File = 'file',
  System = 'system'
}