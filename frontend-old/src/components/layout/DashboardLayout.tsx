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
import ThemeToggle from '@/components/common/ThemeToggle';
import SkipNavigationLink from '@/components/common/SkipNavigationLink';
import { ContentSkeleton } from '@/components/common/SkeletonLoaders';
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
    <Box className="glass" sx={{ height: '100%', border: 'none', borderRadius: 0 }}>
      <Toolbar sx={{
        background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
        borderRadius: 0,
        boxShadow: '0 2px 8px rgba(0, 102, 204, 0.2)'
      }}>
        <Typography
          variant="h6"
          noWrap
          component="div"
          className="font-display"
          sx={{
            color: 'white',
            fontWeight: 600,
            textShadow: '0 1px 2px rgba(0,0,0,0.1)'
          }}
        >
          {t('common.appName')}
        </Typography>
      </Toolbar>
      <Divider sx={{ borderColor: 'rgba(0, 102, 204, 0.1)' }} />

      {/* Community Selector in Sidebar */}
      <Box sx={{
        p: 2,
        background: 'linear-gradient(135deg, rgba(0, 102, 204, 0.05), rgba(230, 126, 34, 0.05))',
        borderRadius: 1,
        mx: 1,
        mt: 2
      }}>
        <Typography
          variant="subtitle2"
          color="text.secondary"
          gutterBottom
          className="font-body"
          sx={{ fontWeight: 500 }}
        >
          {t('pages.communities.activeCommunity')}
        </Typography>
        <CommunitySelector compact />
      </Box>
      <Divider sx={{ borderColor: 'rgba(0, 102, 204, 0.1)', mt: 2 }} />

      <List role="menubar" aria-label={t('navigation.menuItems')} sx={{ px: 1, pt: 2 }}>
        {navigationItems.map((item) => (
          <ListItem key={item.label} disablePadding sx={{ mb: 0.5 }}>
            <ListItemButton
              component={Link}
              href={item.href}
              selected={pathname === item.href}
              role="menuitem"
              aria-current={pathname === item.href ? 'page' : undefined}
              aria-label={item.badge ? t('navigation.menuItemWithBadge', { label: item.label, count: item.badge }) : item.label}
              className="hover-lift"
              sx={{
                borderRadius: 2,
                mb: 0.5,
                mx: 0.5,
                transition: 'all 0.3s ease',
                '&:hover': {
                  background: 'linear-gradient(135deg, rgba(0, 102, 204, 0.08), rgba(0, 102, 204, 0.12))',
                  transform: 'translateX(4px)',
                },
                '&.Mui-selected': {
                  background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
                  color: 'white',
                  borderRadius: 2,
                  '& .MuiListItemIcon-root': {
                    color: 'white',
                  },
                  '& .MuiListItemText-primary': {
                    fontWeight: 600,
                  },
                  '&:hover': {
                    background: 'linear-gradient(135deg, var(--mediterranean-600), var(--mediterranean-700))',
                  },
                },
              }}
            >
              <ListItemIcon sx={{ minWidth: 40 }}>
                {item.badge ? (
                  <Badge
                    badgeContent={item.badge}
                    color="error"
                    aria-label={t('navigation.badge', { count: item.badge })}
                    sx={{
                      '& .MuiBadge-badge': {
                        background: 'linear-gradient(135deg, var(--terracotta-500), var(--terracotta-600))',
                        boxShadow: '0 2px 4px rgba(230, 126, 34, 0.3)',
                      }
                    }}
                  >
                    {item.icon}
                  </Badge>
                ) : (
                  item.icon
                )}
              </ListItemIcon>
              <ListItemText
                primary={item.label}
                primaryTypographyProps={{
                  className: 'font-body',
                  sx: { fontWeight: 500 }
                }}
              />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      {/* Decorative Italian pattern at bottom */}
      <Box
        sx={{
          position: 'absolute',
          bottom: 0,
          left: 0,
          right: 0,
          height: 80,
          background: 'linear-gradient(180deg, transparent, rgba(107, 142, 35, 0.1))',
          opacity: 0.3
        }}
        className="pattern-olive"
      />
    </Box>
  );

  if (isLoading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="100vh" p={3}>
        <ContentSkeleton lines={4} showHeader={true} showButton={true} />
      </Box>
    );
  }

  if (!user) {
    router.push('/');
    return null;
  }

  return (
    <>
      <SkipNavigationLink />
      <Box sx={{ display: 'flex' }} role="application" aria-label={t('navigation.appLayout')}>
      <AppBar
        position="fixed"
        className="glass"
        sx={{
          width: { md: `calc(100% - ${drawerWidth}px)` },
          ml: { md: `${drawerWidth}px` },
          background: 'linear-gradient(135deg, var(--mediterranean-500), var(--mediterranean-600))',
          backdropFilter: 'blur(10px)',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
          boxShadow: '0 4px 20px rgba(0, 102, 204, 0.15)',
        }}
        role="banner"
        aria-label={t('navigation.header')}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label={t('navigation.openMenu')}
            aria-expanded={mobileOpen}
            aria-controls="navigation-drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { md: 'none' } }}
          >
            <MenuIcon />
          </IconButton>

          <Typography
            variant="h6"
            noWrap
            component="div"
            className="font-display"
            sx={{
              flexGrow: 1,
              fontWeight: 600,
              textShadow: '0 1px 2px rgba(0,0,0,0.1)',
              background: 'linear-gradient(45deg, white, rgba(255,255,255,0.9))',
              backgroundClip: 'text',
              color: 'transparent'
            }}
          >
            {activeCommunity ? activeCommunity.name : (navigationItems.find(item => item.href === pathname)?.label || t('common.appName'))}
          </Typography>

          {/* Theme Toggle */}
          <Box sx={{ mr: 1 }}>
            <ThemeToggle compact />
          </Box>

          {/* Language Switcher */}
          <Box sx={{ mr: 2, display: { xs: 'none', sm: 'block' } }}>
            <LanguageSwitcher compact />
          </Box>

          <IconButton
            color="inherit"
            className="hover-glow"
            sx={{
              mr: 1,
              background: 'rgba(255, 255, 255, 0.1)',
              backdropFilter: 'blur(5px)',
              '&:hover': {
                background: 'rgba(255, 255, 255, 0.2)',
                transform: 'scale(1.05)',
              }
            }}
            aria-label={t('navigation.notifications')}
            aria-describedby="notifications-badge"
          >
            <Badge
              badgeContent={4}
              color="error"
              aria-label={t('navigation.unreadNotifications', { count: 4 })}
              sx={{
                '& .MuiBadge-badge': {
                  background: 'linear-gradient(135deg, var(--terracotta-500), var(--terracotta-600))',
                  boxShadow: '0 2px 8px rgba(230, 126, 34, 0.4)',
                  animation: 'float 2s ease-in-out infinite',
                }
              }}
            >
              <NotificationsIcon />
            </Badge>
          </IconButton>

          <IconButton
            onClick={handleProfileMenuOpen}
            color="inherit"
            className="hover-lift"
            aria-label={t('navigation.userMenu')}
            aria-controls="user-profile-menu"
            aria-haspopup="true"
            aria-expanded={Boolean(anchorEl)}
            sx={{
              p: 0.5,
              '&:hover': {
                background: 'rgba(255, 255, 255, 0.1)',
              }
            }}
          >
            <Avatar
              src={user.image || undefined}
              alt={user.name || 'User'}
              className="hover-glow"
              sx={{
                width: 36,
                height: 36,
                border: '2px solid rgba(255, 255, 255, 0.3)',
                background: 'linear-gradient(135deg, var(--terracotta-400), var(--mediterranean-400))',
                fontWeight: 600,
                fontSize: '0.9rem',
                transition: 'all 0.3s ease',
              }}
            >
              {user.name ? user.name.charAt(0).toUpperCase() : 'U'}
            </Avatar>
          </IconButton>
        </Toolbar>
      </AppBar>

      {/* Profile Menu */}
      <Menu
        id="user-profile-menu"
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
        aria-labelledby="user-profile-button"
        role="menu"
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
        {/* Theme and Language Settings for Mobile */}
        <Box sx={{ p: 2, display: { xs: 'block', sm: 'none' } }}>
          <Typography variant="subtitle2" color="text.secondary" gutterBottom>
            {t('common.settings')}
          </Typography>
          <Box display="flex" gap={2} alignItems="center" mb={2}>
            <Box>
              <Typography variant="caption" color="text.secondary">
                {t('theme.mode')}
              </Typography>
              <Box mt={0.5}>
                <ThemeToggle />
              </Box>
            </Box>
            <Box>
              <Typography variant="caption" color="text.secondary">
                {t('common.language')}
              </Typography>
              <Box mt={0.5}>
                <LanguageSwitcher />
              </Box>
            </Box>
          </Box>
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
        aria-label={t('navigation.main')}
        role="navigation"
      >
        <Drawer
          id="navigation-drawer"
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true,
            'aria-labelledby': 'mobile-navigation-title',
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
          aria-label={t('navigation.desktop')}
        >
          {drawer}
        </Drawer>
      </Box>

      {/* Main content */}
      <Box
        id="main-content"
        component="main"
        tabIndex={-1}
        sx={{
          flexGrow: 1,
          p: 3,
          width: { md: `calc(100% - ${drawerWidth}px)` },
          mt: { xs: 7, sm: 8 },
          background: `
            radial-gradient(circle at 20% 20%, rgba(0, 102, 204, 0.03) 0%, transparent 50%),
            radial-gradient(circle at 80% 80%, rgba(230, 126, 34, 0.03) 0%, transparent 50%),
            var(--background)
          `,
          minHeight: 'calc(100vh - 64px)',
          outline: 'none',
          position: 'relative',
          '&::before': {
            content: '""',
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            background: 'url("data:image/svg+xml,%3Csvg width="20" height="20" xmlns="http://www.w3.org/2000/svg"%3E%3Cdefs%3E%3Cpattern id="a" patternUnits="userSpaceOnUse" width="20" height="20"%3E%3Ccircle fill="%23006699" fill-opacity="0.02" cx="10" cy="10" r="1"/%3E%3C/pattern%3E%3C/defs%3E%3Crect width="100%25" height="100%25" fill="url(%23a)"/%3E%3C/svg%3E")',
            pointerEvents: 'none',
            zIndex: 0
          }
        }}
        role="main"
        aria-label={t('navigation.mainContent')}
      >
        <Box sx={{ position: 'relative', zIndex: 1 }} className="animate-fade-in">
          {children}
        </Box>
      </Box>
      </Box>
    </>
  );
}