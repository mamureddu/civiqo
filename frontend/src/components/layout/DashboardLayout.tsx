'use client';

import React, { useState } from 'react';
import {
  AppBar,
  Box,
  Drawer,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
  Typography,
  Avatar,
  Menu,
  MenuItem,
  Divider,
  useTheme,
  Badge,
} from '@mui/material';
import {
  Menu as MenuIcon,
  Dashboard as DashboardIcon,
  Groups as GroupsIcon,
  Business as BusinessIcon,
  Museum as MuseumIcon,
  HowToVote as GovernanceIcon,
  Chat as ChatIcon,
  Person as PersonIcon,
  Settings as SettingsIcon,
  Logout as LogoutIcon,
  Notifications as NotificationsIcon,
} from '@mui/icons-material';
import { useSession, signOut } from 'next-auth/react';
import { useRouter, usePathname } from 'next/navigation';
import Link from 'next/link';
import CommunitySelector from '@/components/community/CommunitySelector';
import LanguageSwitcher from '@/components/common/LanguageSwitcher';
import { useTranslation } from 'react-i18next';
import { useCommunity } from '@/contexts/CommunityContext';

const drawerWidth = 240;

interface NavigationItem {
  label: string;
  icon: React.ReactNode;
  href: string;
  badge?: number;
}

// Navigation items will be translated in the component

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export default function DashboardLayout({ children }: DashboardLayoutProps) {
  const { data: session, status } = useSession();
  const user = session?.user;
  const isLoading = status === 'loading';
  const router = useRouter();
  const pathname = usePathname();
  const theme = useTheme();
  const { t } = useTranslation('common');
  const { activeCommunity } = useCommunity();

  const [mobileOpen, setMobileOpen] = useState(false);
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

  // Navigation items with translations
  const navigationItems: NavigationItem[] = [
    {
      label: t('navigation.dashboard'),
      icon: <DashboardIcon />,
      href: '/dashboard',
    },
    {
      label: t('navigation.communities'),
      icon: <GroupsIcon />,
      href: '/communities',
    },
    {
      label: t('navigation.businesses'),
      icon: <BusinessIcon />,
      href: '/businesses',
    },
    {
      label: t('navigation.poi'),
      icon: <MuseumIcon />,
      href: '/poi',
    },
    {
      label: t('navigation.governance'),
      icon: <GovernanceIcon />,
      href: '/governance',
    },
    {
      label: t('navigation.chat'),
      icon: <ChatIcon />,
      href: '/chat',
      badge: 3, // Example badge for unread messages
    },
  ];

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
  };

  const handleProfileMenuOpen = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleLogout = () => {
    signOut();
  };

  const drawer = (
    <Box>
      <Toolbar>
        <Typography variant="h6" noWrap component="div" color="primary" fontWeight="bold">
          {t('common.appName')}
        </Typography>
      </Toolbar>
      <Divider />

      {/* Community Selector in Sidebar */}
      <Box sx={{ p: 2 }}>
        <Typography variant="subtitle2" color="text.secondary" gutterBottom>
          {t('pages.communities.activeCommunity')}
        </Typography>
        <CommunitySelector compact />
      </Box>
      <Divider />

      <List>
        {navigationItems.map((item) => (
          <ListItem key={item.label} disablePadding>
            <ListItemButton
              component={Link}
              href={item.href}
              selected={pathname === item.href}
              sx={{
                '&.Mui-selected': {
                  backgroundColor: theme.palette.primary.main + '20',
                  borderRight: `3px solid ${theme.palette.primary.main}`,
                },
              }}
            >
              <ListItemIcon>
                {item.badge ? (
                  <Badge badgeContent={item.badge} color="error">
                    {item.icon}
                  </Badge>
                ) : (
                  item.icon
                )}
              </ListItemIcon>
              <ListItemText primary={item.label} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </Box>
  );

  if (isLoading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="100vh">
        <Typography>Loading...</Typography>
      </Box>
    );
  }

  if (!user) {
    router.push('/');
    return null;
  }

  return (
    <Box sx={{ display: 'flex' }}>
      <AppBar
        position="fixed"
        sx={{
          width: { md: `calc(100% - ${drawerWidth}px)` },
          ml: { md: `${drawerWidth}px` },
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { md: 'none' } }}
          >
            <MenuIcon />
          </IconButton>

          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1 }}>
            {activeCommunity ? activeCommunity.name : (navigationItems.find(item => item.href === pathname)?.label || t('common.appName'))}
          </Typography>

          {/* Language Switcher */}
          <Box sx={{ mr: 2, display: { xs: 'none', sm: 'block' } }}>
            <LanguageSwitcher compact />
          </Box>

          <IconButton color="inherit" sx={{ mr: 1 }}>
            <Badge badgeContent={4} color="error">
              <NotificationsIcon />
            </Badge>
          </IconButton>

          <IconButton
            onClick={handleProfileMenuOpen}
            color="inherit"
            aria-label="account of current user"
            aria-controls="primary-search-account-menu"
            aria-haspopup="true"
          >
            <Avatar
              src={user.image || undefined}
              alt={user.name || 'User'}
              sx={{ width: 32, height: 32 }}
            >
              {user.name ? user.name.charAt(0).toUpperCase() : 'U'}
            </Avatar>
          </IconButton>
        </Toolbar>
      </AppBar>

      {/* Profile Menu */}
      <Menu
        anchorEl={anchorEl}
        anchorOrigin={{
          vertical: 'bottom',
          horizontal: 'right',
        }}
        keepMounted
        transformOrigin={{
          vertical: 'top',
          horizontal: 'right',
        }}
        open={Boolean(anchorEl)}
        onClose={handleMenuClose}
      >
        <MenuItem onClick={handleMenuClose}>
          <ListItemIcon>
            <PersonIcon fontSize="small" />
          </ListItemIcon>
          <Typography variant="inherit">{user.name || user.email}</Typography>
        </MenuItem>
        <MenuItem onClick={handleMenuClose} component={Link} href="/profile">
          <ListItemIcon>
            <SettingsIcon fontSize="small" />
          </ListItemIcon>
          {t('navigation.profile')}
        </MenuItem>
        <Divider />
        {/* Language Switcher for Mobile */}
        <Box sx={{ p: 2, display: { xs: 'block', sm: 'none' } }}>
          <Typography variant="subtitle2" color="text.secondary" gutterBottom>
            {t('common.language')}
          </Typography>
          <LanguageSwitcher />
        </Box>
        <Divider sx={{ display: { xs: 'block', sm: 'none' } }} />
        <MenuItem onClick={handleLogout}>
          <ListItemIcon>
            <LogoutIcon fontSize="small" />
          </ListItemIcon>
          {t('actions.logout')}
        </MenuItem>
      </Menu>

      {/* Navigation Drawer */}
      <Box
        component="nav"
        sx={{ width: { md: drawerWidth }, flexShrink: { md: 0 } }}
        aria-label="navigation"
      >
        <Drawer
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true, // Better open performance on mobile.
          }}
          sx={{
            display: { xs: 'block', md: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
        >
          {drawer}
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: 'none', md: 'block' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
          open
        >
          {drawer}
        </Drawer>
      </Box>

      {/* Main content */}
      <Box
        component="main"
        sx={{
          flexGrow: 1,
          p: 3,
          width: { md: `calc(100% - ${drawerWidth}px)` },
          mt: { xs: 7, sm: 8 },
          backgroundColor: theme.palette.background.default,
          minHeight: 'calc(100vh - 64px)',
        }}
      >
        {children}
      </Box>
    </Box>
  );
}