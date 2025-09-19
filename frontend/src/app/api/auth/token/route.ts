import { getServerSession } from 'next-auth';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const session = await getServerSession();

    if (!session) {
      return NextResponse.json({ accessToken: null }, { status: 401 });
    }

    return NextResponse.json({ accessToken: session.accessToken || null });
  } catch (error) {
    console.error('Error getting access token:', error);
    return NextResponse.json({ accessToken: null }, { status: 401 });
  }
}