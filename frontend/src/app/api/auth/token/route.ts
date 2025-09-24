import { getServerSession } from 'next-auth';
import { NextRequest, NextResponse } from 'next/server';
import NextAuth from 'next-auth'
import Auth0Provider from 'next-auth/providers/auth0'

// Auth configuration matching the main NextAuth setup
const authOptions = {
  providers: [
    Auth0Provider({
      clientId: process.env.AUTH0_CLIENT_ID!,
      clientSecret: process.env.AUTH0_CLIENT_SECRET!,
      issuer: `https://${process.env.AUTH0_DOMAIN}`,
    })
  ],
  callbacks: {
    async jwt({ token, account, profile }: any) {
      if (account) {
        token.accessToken = account.access_token
        token.idToken = account.id_token
      }
      return token
    },
    async session({ session, token }: any) {
      session.accessToken = token.accessToken
      session.idToken = token.idToken
      return session
    },
  },
}

export async function GET(request: NextRequest) {
  try {
    const session = await getServerSession(authOptions);

    if (!session) {
      return NextResponse.json({ accessToken: null }, { status: 401 });
    }

    return NextResponse.json({ accessToken: session.accessToken || null });
  } catch (error) {
    console.error('Error getting access token:', error);
    return NextResponse.json({ accessToken: null }, { status: 401 });
  }
}