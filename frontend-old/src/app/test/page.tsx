'use client'

import { useEffect, useState } from 'react'
import { useSession, signIn, signOut } from 'next-auth/react'
import apiClient from '@/lib/api-client'

interface BackendStatus {
  status: string
  service: string
  timestamp: string
}

export default function IntegrationTestPage() {
  const { data: session, status } = useSession()
  const [backendStatus, setBackendStatus] = useState<BackendStatus | null>(null)
  const [healthLoading, setHealthLoading] = useState(false)
  const [healthError, setHealthError] = useState<string | null>(null)
  const [apiTest, setApiTest] = useState<any>(null)
  const [apiLoading, setApiLoading] = useState(false)
  const [apiError, setApiError] = useState<string | null>(null)

  // Test backend health endpoint
  const testBackendHealth = async () => {
    setHealthLoading(true)
    setHealthError(null)

    try {
      // Direct health check call
      const response = await fetch('http://localhost:9001/lambda-url/api-gateway/health')

      if (response.ok) {
        const data = await response.json()
        setBackendStatus(data)
      } else {
        setHealthError(`HTTP ${response.status}: ${response.statusText}`)
      }
    } catch (error) {
      setHealthError(`Connection error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    }

    setHealthLoading(false)
  }

  // Test API client with authentication
  const testAuthenticatedAPI = async () => {
    setApiLoading(true)
    setApiError(null)

    try {
      // Try to call a basic API endpoint (when available)
      const response = await apiClient.getCurrentUser()
      setApiTest(response)
    } catch (error) {
      setApiError(`API Error: ${error instanceof Error ? error.message : 'Unknown error'}`)
    }

    setApiLoading(false)
  }

  return (
    <div className="container mx-auto p-8 max-w-4xl">
      <h1 className="text-3xl font-bold mb-8">Frontend-Backend Integration Test</h1>

      {/* Authentication Status */}
      <section className="mb-8 p-6 border rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">Authentication Status</h2>

        {status === 'loading' && (
          <p className="text-blue-600">Loading authentication status...</p>
        )}

        {status === 'authenticated' && session && (
          <div className="space-y-2">
            <p className="text-green-600 font-medium">✅ Authenticated</p>
            <div className="bg-gray-100 p-4 rounded">
              <p><strong>Name:</strong> {session.user?.name || 'Not provided'}</p>
              <p><strong>Email:</strong> {session.user?.email || 'Not provided'}</p>
              <p><strong>Access Token:</strong> {session.accessToken ? '✅ Present' : '❌ Missing'}</p>
            </div>
            <button
              onClick={() => signOut()}
              className="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600"
            >
              Sign Out
            </button>
          </div>
        )}

        {status === 'unauthenticated' && (
          <div className="space-y-2">
            <p className="text-red-600 font-medium">❌ Not authenticated</p>
            <button
              onClick={() => signIn('auth0')}
              className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
            >
              Sign In with Auth0
            </button>
          </div>
        )}
      </section>

      {/* Backend Health Check */}
      <section className="mb-8 p-6 border rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">Backend Health Check</h2>

        <button
          onClick={testBackendHealth}
          disabled={healthLoading}
          className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 disabled:opacity-50 mb-4"
        >
          {healthLoading ? 'Testing...' : 'Test Backend Health'}
        </button>

        {healthError && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            <p className="font-medium">Health Check Failed:</p>
            <p className="text-sm">{healthError}</p>
            <p className="text-xs mt-2">
              Expected endpoint: http://localhost:9001/lambda-url/api-gateway/health
            </p>
          </div>
        )}

        {backendStatus && (
          <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
            <p className="font-medium">✅ Backend Health Check Successful</p>
            <div className="bg-white p-3 mt-2 rounded text-sm">
              <pre>{JSON.stringify(backendStatus, null, 2)}</pre>
            </div>
          </div>
        )}
      </section>

      {/* API Client Test */}
      <section className="mb-8 p-6 border rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">API Client Test</h2>

        <button
          onClick={testAuthenticatedAPI}
          disabled={apiLoading || status !== 'authenticated'}
          className="bg-purple-500 text-white px-4 py-2 rounded hover:bg-purple-600 disabled:opacity-50 mb-4"
        >
          {apiLoading ? 'Testing...' : 'Test Authenticated API Call'}
        </button>

        {status !== 'authenticated' && (
          <p className="text-gray-600 text-sm mb-4">
            Sign in first to test authenticated API calls
          </p>
        )}

        {apiError && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            <p className="font-medium">API Test Failed:</p>
            <p className="text-sm">{apiError}</p>
          </div>
        )}

        {apiTest && (
          <div className="bg-purple-100 border border-purple-400 text-purple-700 px-4 py-3 rounded">
            <p className="font-medium">✅ API Test Result</p>
            <div className="bg-white p-3 mt-2 rounded text-sm">
              <pre>{JSON.stringify(apiTest, null, 2)}</pre>
            </div>
          </div>
        )}
      </section>

      {/* Environment Information */}
      <section className="mb-8 p-6 border rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">Environment Information</h2>

        <div className="space-y-2 text-sm">
          <p><strong>Frontend URL:</strong> {window.location.origin}</p>
          <p><strong>API Base URL:</strong> {process.env.NEXT_PUBLIC_API_BASE_URL}</p>
          <p><strong>Auth0 Domain:</strong> {process.env.NEXT_PUBLIC_AUTH0_DOMAIN || 'Not set'}</p>
          <p><strong>Environment:</strong> {process.env.NODE_ENV}</p>
        </div>
      </section>

      {/* Instructions */}
      <section className="p-6 border rounded-lg bg-blue-50">
        <h2 className="text-2xl font-semibold mb-4">Integration Test Instructions</h2>

        <ol className="list-decimal list-inside space-y-2 text-sm">
          <li>First, test the backend health endpoint to verify the backend is running</li>
          <li>Sign in with Auth0 to test the authentication flow</li>
          <li>Test authenticated API calls to verify the full integration</li>
          <li>Check that JWT tokens are properly passed to the backend</li>
        </ol>

        <div className="mt-4 p-4 bg-white rounded border">
          <h3 className="font-medium mb-2">Expected Results:</h3>
          <ul className="list-disc list-inside space-y-1 text-sm">
            <li>Backend health check should return service information</li>
            <li>Auth0 sign-in should redirect and return user information</li>
            <li>API calls should include JWT tokens in Authorization header</li>
            <li>CORS should allow requests from frontend to backend</li>
          </ul>
        </div>
      </section>
    </div>
  )
}