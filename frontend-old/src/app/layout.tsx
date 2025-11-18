import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Providers from '@/components/providers/Providers';
import DynamicHtmlLang from '@/components/common/DynamicHtmlLang';

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Community Manager",
  description: "Local community management platform with real-time chat, business directory, and democratic governance tools",
  keywords: ["community", "local", "governance", "business", "chat", "democracy"],
  authors: [{ name: "Community Manager Team" }],
};

export const viewport = {
  width: 'device-width',
  initialScale: 1,
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="it">
      <body className={inter.className}>
        <Providers>
          <DynamicHtmlLang />
          {children}
        </Providers>
      </body>
    </html>
  );
}
