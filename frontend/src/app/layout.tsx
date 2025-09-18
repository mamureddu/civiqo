import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { UserProvider } from '@auth0/nextjs-auth0/client';
import { ThemeProvider } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';
import theme from '@/theme/theme';

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Community Manager",
  description: "Local community management platform with real-time chat, business directory, and democratic governance tools",
  keywords: ["community", "local", "governance", "business", "chat", "democracy"],
  authors: [{ name: "Community Manager Team" }],
  viewport: "width=device-width, initial-scale=1",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <UserProvider>
          <ThemeProvider theme={theme}>
            <CssBaseline />
            {children}
          </ThemeProvider>
        </UserProvider>
      </body>
    </html>
  );
}
