'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface Community {
  id: string;
  name: string;
  description: string;
  member_count: number;
  created_at: string;
  updated_at: string;
  location?: {
    latitude: number;
    longitude: number;
    address: string;
  };
  settings?: {
    is_public: boolean;
    requires_approval: boolean;
    subscription_enabled: boolean;
  };
  subscription_status?: 'free' | 'supporter' | 'vip';
  member_since?: string;
}

interface CommunityContextType {
  activeCommunity: Community | null;
  userCommunities: Community[];
  setActiveCommunity: (community: Community | null) => void;
  setUserCommunities: (communities: Community[]) => void;
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

const CommunityContext = createContext<CommunityContextType | undefined>(undefined);

interface CommunityProviderProps {
  children: ReactNode;
}

export function CommunityProvider({ children }: CommunityProviderProps) {
  const [activeCommunity, setActiveCommunityState] = useState<Community | null>(null);
  const [userCommunities, setUserCommunities] = useState<Community[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // Load active community from localStorage on mount
  useEffect(() => {
    try {
      const saved = localStorage.getItem('activeCommunity');
      if (saved) {
        const community = JSON.parse(saved);
        setActiveCommunityState(community);
      }
    } catch (error) {
      console.error('Failed to load active community from localStorage:', error);
    }
  }, []);

  // Save active community to localStorage when it changes
  const setActiveCommunity = (community: Community | null) => {
    setActiveCommunityState(community);
    try {
      if (community) {
        localStorage.setItem('activeCommunity', JSON.stringify(community));
      } else {
        localStorage.removeItem('activeCommunity');
      }
    } catch (error) {
      console.error('Failed to save active community to localStorage:', error);
    }
  };

  const value: CommunityContextType = {
    activeCommunity,
    userCommunities,
    setActiveCommunity,
    setUserCommunities,
    isLoading,
    setIsLoading,
  };

  return (
    <CommunityContext.Provider value={value}>
      {children}
    </CommunityContext.Provider>
  );
}

export function useCommunity() {
  const context = useContext(CommunityContext);
  if (context === undefined) {
    throw new Error('useCommunity must be used within a CommunityProvider');
  }
  return context;
}

export type { Community };