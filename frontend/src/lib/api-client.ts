import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { getSession } from 'next-auth/react';
import type {
  ApiResponse,
  Community,
  CreateCommunityRequest,
  Business,
  BusinessSearchResult,
  CreateBusinessRequest,
  Poll,
  Decision,
  User,
  UserWithProfile,
  PaginationParams
} from '@/types/api';

class ApiClient {
  private client: AxiosInstance;
  private baseURL: string;

  constructor() {
    this.baseURL = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:8080';

    this.client = axios.create({
      baseURL: this.baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    this.setupInterceptors();
  }

  private setupInterceptors() {
    // Request interceptor to add auth token
    this.client.interceptors.request.use(
      async (config) => {
        try {
          // For server-side requests, we'll get the token differently
          if (typeof window === 'undefined') {
            // Server-side: token will be passed explicitly or from request context
          } else {
            // Client-side: get token from Auth0
            const response = await fetch('/api/auth/token');
            if (response.ok) {
              const { accessToken } = await response.json();
              if (accessToken) {
                config.headers.Authorization = `Bearer ${accessToken}`;
              }
            }
          }
        } catch (error) {
          console.warn('Failed to get access token:', error);
        }

        return config;
      },
      (error) => {
        return Promise.reject(error);
      }
    );

    // Response interceptor for error handling
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        if (error.response?.status === 401) {
          // Redirect to login on unauthorized
          if (typeof window !== 'undefined') {
            window.location.href = '/';
          }
        }
        return Promise.reject(error);
      }
    );
  }

  // Helper method to make requests with proper typing
  private async request<T>(config: AxiosRequestConfig): Promise<ApiResponse<T>> {
    try {
      const response: AxiosResponse<ApiResponse<T>> = await this.client(config);
      return response.data;
    } catch (error: any) {
      // Handle different types of errors
      if (error.response?.data) {
        return error.response.data;
      }

      return {
        success: false,
        error: {
          message: error.message || 'An unexpected error occurred',
          code: error.code
        }
      };
    }
  }

  // Community API methods
  async getCommunities(params?: PaginationParams): Promise<ApiResponse<Community[]>> {
    return this.request<Community[]>({
      method: 'GET',
      url: '/api/communities',
      params
    });
  }

  async getCommunity(id: string): Promise<ApiResponse<Community>> {
    return this.request<Community>({
      method: 'GET',
      url: `/api/communities/${id}`
    });
  }

  async createCommunity(data: CreateCommunityRequest): Promise<ApiResponse<Community>> {
    return this.request<Community>({
      method: 'POST',
      url: '/api/communities',
      data
    });
  }

  async updateCommunity(id: string, data: Partial<CreateCommunityRequest>): Promise<ApiResponse<Community>> {
    return this.request<Community>({
      method: 'PUT',
      url: `/api/communities/${id}`,
      data
    });
  }

  async joinCommunity(id: string): Promise<ApiResponse<void>> {
    return this.request<void>({
      method: 'POST',
      url: `/api/communities/${id}/join`
    });
  }

  async leaveCommunity(id: string): Promise<ApiResponse<void>> {
    return this.request<void>({
      method: 'POST',
      url: `/api/communities/${id}/leave`
    });
  }

  // Business API methods
  async getBusinesses(communityId: string, params?: PaginationParams & {
    q?: string;
    category?: string;
    location?: { latitude: number; longitude: number };
    radius_km?: number;
  }): Promise<ApiResponse<BusinessSearchResult[]>> {
    return this.request<BusinessSearchResult[]>({
      method: 'GET',
      url: `/api/communities/${communityId}/businesses`,
      params
    });
  }

  async getBusiness(id: string): Promise<ApiResponse<Business>> {
    return this.request<Business>({
      method: 'GET',
      url: `/api/businesses/${id}`
    });
  }

  async createBusiness(communityId: string, data: CreateBusinessRequest): Promise<ApiResponse<Business>> {
    return this.request<Business>({
      method: 'POST',
      url: `/api/communities/${communityId}/businesses`,
      data
    });
  }

  async updateBusiness(id: string, data: Partial<CreateBusinessRequest>): Promise<ApiResponse<Business>> {
    return this.request<Business>({
      method: 'PUT',
      url: `/api/businesses/${id}`,
      data
    });
  }

  // Governance API methods
  async getPolls(communityId: string, params?: PaginationParams): Promise<ApiResponse<Poll[]>> {
    return this.request<Poll[]>({
      method: 'GET',
      url: `/api/communities/${communityId}/polls`,
      params
    });
  }

  async getPoll(id: string): Promise<ApiResponse<Poll>> {
    return this.request<Poll>({
      method: 'GET',
      url: `/api/polls/${id}`
    });
  }

  async createPoll(communityId: string, data: any): Promise<ApiResponse<Poll>> {
    return this.request<Poll>({
      method: 'POST',
      url: `/api/communities/${communityId}/polls`,
      data
    });
  }

  async castVote(pollId: string, data: { selected_options: number[]; comment?: string }): Promise<ApiResponse<any>> {
    return this.request<any>({
      method: 'POST',
      url: `/api/polls/${pollId}/vote`,
      data
    });
  }

  async getDecisions(communityId: string, params?: PaginationParams): Promise<ApiResponse<Decision[]>> {
    return this.request<Decision[]>({
      method: 'GET',
      url: `/api/communities/${communityId}/decisions`,
      params
    });
  }

  async createDecision(communityId: string, data: any): Promise<ApiResponse<Decision>> {
    return this.request<Decision>({
      method: 'POST',
      url: `/api/communities/${communityId}/decisions`,
      data
    });
  }

  // User API methods
  async getCurrentUser(): Promise<ApiResponse<UserWithProfile>> {
    return this.request<UserWithProfile>({
      method: 'GET',
      url: '/api/auth/me'
    });
  }

  async updateUserProfile(data: any): Promise<ApiResponse<UserWithProfile>> {
    return this.request<UserWithProfile>({
      method: 'PUT',
      url: '/api/auth/profile',
      data
    });
  }

  // File upload methods
  async getUploadUrl(fileName: string, fileType: string): Promise<ApiResponse<{ upload_url: string; file_url: string }>> {
    return this.request<{ upload_url: string; file_url: string }>({
      method: 'POST',
      url: '/api/uploads/presigned-url',
      data: { fileName, fileType }
    });
  }

  // Set auth token for server-side requests
  setAuthToken(token: string) {
    this.client.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  }

  // Remove auth token
  clearAuthToken() {
    delete this.client.defaults.headers.common['Authorization'];
  }
}

// Create a singleton instance
const apiClient = new ApiClient();
export default apiClient;