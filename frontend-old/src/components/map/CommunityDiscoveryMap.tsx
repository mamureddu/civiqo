'use client';

import { useEffect, useRef, useState } from 'react';
import { MapContainer, TileLayer, Marker, Popup, Circle, useMap } from 'react-leaflet';
import { Icon, LatLngBounds } from 'leaflet';
import {
  Box,
  Typography,
  Button,
  Chip,
  Stack,
  Card,
  CardContent,
  Avatar,
  Divider,
  FormControlLabel,
  Switch,
  Paper,
} from '@mui/material';
import {
  Groups as GroupsIcon,
  People as PeopleIcon,
  Store as StoreIcon,
  LocationOn as LocationIcon,
  Museum as MuseumIcon,
  Park as ParkIcon,
  LocalLibrary as LibraryIcon,
  Church as ChurchIcon,
  AccountBalance as MonumentIcon,
  Stadium as StadiumIcon,
  TheaterComedy as TheaterIcon,
  Info as InfoIcon,
} from '@mui/icons-material';
import 'leaflet/dist/leaflet.css';

// Fix for default markers in React Leaflet
delete (Icon.Default.prototype as any)._getIconUrl;
Icon.Default.mergeOptions({
  iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon-2x.png',
  iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon.png',
  shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-shadow.png',
});

// Custom community marker icon
const communityIcon = new Icon({
  iconUrl: 'data:image/svg+xml;base64,' + btoa(`
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1976d2" width="40" height="40">
      <circle cx="12" cy="12" r="10" fill="#1976d2"/>
      <path d="M16 16s-1-1-4-1-4 1-4 1v-1c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2v1z" fill="white"/>
      <circle cx="12" cy="10" r="2" fill="white"/>
    </svg>
  `),
  iconSize: [40, 40],
  iconAnchor: [20, 40],
  popupAnchor: [0, -40],
});

interface Community {
  id: string;
  name: string;
  description: string;
  memberCount: number;
  businessCount: number;
  location: { lat: number; lng: number };
  image: string;
  subscription: { hasSubscription: boolean; startingPrice?: number };
  category: string;
}

interface PointOfInterest {
  id: string;
  name: string;
  description: string;
  category: string;
  address: string;
  latitude: number;
  longitude: number;
  image_url?: string;
  rating?: number;
  is_verified: boolean;
}

interface CommunityDiscoveryMapProps {
  communities: Community[];
  selectedCommunity?: Community | null;
  onCommunitySelect?: (community: Community | null) => void;
  showPOIs?: boolean;
}

// Component to fit map bounds to show all communities
function MapBoundsFitter({ communities }: { communities: Community[] }) {
  const map = useMap();

  useEffect(() => {
    if (communities.length > 0) {
      const bounds = new LatLngBounds(
        communities.map(community => [community.location.lat, community.location.lng])
      );

      // Add some padding around the bounds
      map.fitBounds(bounds, { padding: [20, 20] });
    }
  }, [communities, map]);

  return null;
}

// Sample POI data for Milan
const samplePOIs: PointOfInterest[] = [
  {
    id: 'poi-1',
    name: 'Milan Cathedral',
    description: 'Stunning Gothic cathedral and symbol of Milan',
    category: 'monument',
    address: 'Piazza del Duomo, Milan',
    latitude: 45.4641943,
    longitude: 9.1896346,
    image_url: 'https://images.unsplash.com/photo-1513475382585-d06e58bcb0e0?w=300',
    rating: 4.6,
    is_verified: true,
  },
  {
    id: 'poi-2',
    name: 'Sempione Park',
    description: 'Large public park perfect for outdoor activities',
    category: 'park',
    address: 'Piazza Sempione, Milan',
    latitude: 45.4715,
    longitude: 9.1738,
    image_url: 'https://images.unsplash.com/photo-1594824717313-684b769b8050?w=300',
    rating: 4.4,
    is_verified: true,
  },
  {
    id: 'poi-3',
    name: 'Biblioteca Ambrosiana',
    description: 'Historic library and art gallery',
    category: 'library',
    address: 'Piazza Pio XI, Milan',
    latitude: 45.4632,
    longitude: 9.1887,
    rating: 4.3,
    is_verified: true,
  },
];

// Create POI icons
const createPOIIcon = (category: string) => {
  const getCategoryColor = (cat: string) => {
    switch (cat) {
      case 'museum': return '#8e24aa';
      case 'park': return '#388e3c';
      case 'library': return '#1976d2';
      case 'monument': return '#5d4037';
      default: return '#616161';
    }
  };

  return new Icon({
    iconUrl: 'data:image/svg+xml;base64,' + btoa(`
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="${getCategoryColor(category)}" width="28" height="28">
        <circle cx="12" cy="12" r="8" fill="${getCategoryColor(category)}" stroke="white" stroke-width="2"/>
        <circle cx="12" cy="10" r="3" fill="white"/>
      </svg>
    `),
    iconSize: [28, 28],
    iconAnchor: [14, 28],
    popupAnchor: [0, -28],
  });
};

export default function CommunityDiscoveryMap({
  communities,
  selectedCommunity,
  onCommunitySelect,
  showPOIs = true
}: CommunityDiscoveryMapProps) {
  const mapRef = useRef<any>(null);
  const [hoveredCommunity, setHoveredCommunity] = useState<Community | null>(null);
  const [poisVisible, setPoisVisible] = useState(showPOIs);

  // Default center (Milan, Italy)
  const defaultCenter: [number, number] = [45.4642, 9.1900];

  useEffect(() => {
    if (selectedCommunity && mapRef.current) {
      const map = mapRef.current;
      map.setView([selectedCommunity.location.lat, selectedCommunity.location.lng], 14);
    }
  }, [selectedCommunity]);

  const getCommunityColor = (category: string) => {
    switch (category.toLowerCase()) {
      case 'urban': return '#1976d2';
      case 'arts': return '#9c27b0';
      case 'entertainment': return '#f57c00';
      case 'residential': return '#388e3c';
      default: return '#1976d2';
    }
  };

  return (
    <Box sx={{ height: '500px', width: '100%', position: 'relative' }}>
      <MapContainer
        ref={mapRef}
        center={defaultCenter}
        zoom={12}
        style={{ height: '100%', width: '100%' }}
        scrollWheelZoom={true}
      >
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />

        {/* Fit bounds to show all communities */}
        <MapBoundsFitter communities={communities} />

        {/* Community markers and areas */}
        {communities.map((community) => (
          <div key={community.id}>
            {/* Community influence area circle */}
            <Circle
              center={[community.location.lat, community.location.lng]}
              radius={800} // 800 meters radius
              pathOptions={{
                color: getCommunityColor(community.category),
                fillColor: getCommunityColor(community.category),
                fillOpacity: 0.1,
                weight: 2,
                opacity: 0.6,
              }}
            />

            {/* Community marker */}
            <Marker
              position={[community.location.lat, community.location.lng]}
              icon={communityIcon}
              eventHandlers={{
                click: () => onCommunitySelect?.(community),
                mouseover: () => setHoveredCommunity(community),
                mouseout: () => setHoveredCommunity(null),
              }}
            >
              <Popup>
                <Card sx={{ minWidth: 280, maxWidth: 320 }}>
                  <Box
                    sx={{
                      height: 120,
                      backgroundImage: `url(${community.image})`,
                      backgroundSize: 'cover',
                      backgroundPosition: 'center',
                      position: 'relative',
                    }}
                  >
                    <Chip
                      label={community.category}
                      size="small"
                      sx={{
                        position: 'absolute',
                        top: 8,
                        right: 8,
                        bgcolor: 'rgba(255, 255, 255, 0.9)',
                      }}
                    />
                  </Box>

                  <CardContent>
                    <Typography variant="h6" fontWeight="bold" gutterBottom>
                      {community.name}
                    </Typography>

                    <Typography
                      variant="body2"
                      color="text.secondary"
                      sx={{ mb: 2, lineHeight: 1.4 }}
                    >
                      {community.description}
                    </Typography>

                    <Stack direction="row" spacing={2} mb={2}>
                      <Stack direction="row" spacing={0.5} alignItems="center">
                        <PeopleIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                        <Typography variant="caption" color="text.secondary">
                          {community.memberCount.toLocaleString()}
                        </Typography>
                      </Stack>
                      <Stack direction="row" spacing={0.5} alignItems="center">
                        <StoreIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                        <Typography variant="caption" color="text.secondary">
                          {community.businessCount}
                        </Typography>
                      </Stack>
                    </Stack>

                    {community.subscription.hasSubscription && (
                      <Chip
                        label={`From €${community.subscription.startingPrice}/month`}
                        size="small"
                        color="secondary"
                        sx={{ mb: 2 }}
                      />
                    )}

                    <Divider sx={{ my: 2 }} />

                    <Stack direction="row" spacing={1}>
                      <Button
                        variant="outlined"
                        size="small"
                        fullWidth
                        href={`/communities/${community.id}`}
                      >
                        View Details
                      </Button>
                      <Button
                        variant="contained"
                        size="small"
                        fullWidth
                        color="primary"
                      >
                        Join Community
                      </Button>
                    </Stack>
                  </CardContent>
                </Card>
              </Popup>
            </Marker>
          </div>
        ))}

        {/* POI markers */}
        {poisVisible && samplePOIs.map((poi) => (
          <Marker
            key={poi.id}
            position={[poi.latitude, poi.longitude]}
            icon={createPOIIcon(poi.category)}
          >
            <Popup>
              <Box sx={{ minWidth: 250, p: 1 }}>
                {poi.image_url && (
                  <Box
                    component="img"
                    src={poi.image_url}
                    alt={poi.name}
                    sx={{
                      width: '100%',
                      height: 100,
                      objectFit: 'cover',
                      borderRadius: 1,
                      mb: 1
                    }}
                  />
                )}

                <Typography variant="h6" fontWeight="bold" gutterBottom>
                  {poi.name}
                </Typography>

                <Typography variant="body2" color="text.secondary" paragraph>
                  {poi.description}
                </Typography>

                <Typography variant="caption" color="text.secondary" display="block" mb={1}>
                  📍 {poi.address}
                </Typography>

                {poi.rating && (
                  <Box display="flex" alignItems="center" mb={1}>
                    <Typography variant="caption" color="text.secondary">
                      ⭐ {poi.rating}/5
                    </Typography>
                  </Box>
                )}

                <Button
                  variant="outlined"
                  size="small"
                  href={`/poi/${poi.id}`}
                  fullWidth
                >
                  View Details
                </Button>
              </Box>
            </Popup>
          </Marker>
        ))}
      </MapContainer>

      {/* Hover tooltip */}
      {hoveredCommunity && (
        <Box
          sx={{
            position: 'absolute',
            top: 16,
            left: 16,
            zIndex: 1000,
            bgcolor: 'background.paper',
            borderRadius: 2,
            boxShadow: 3,
            p: 2,
            minWidth: 200,
            pointerEvents: 'none',
          }}
        >
          <Typography variant="subtitle1" fontWeight="bold">
            {hoveredCommunity.name}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {hoveredCommunity.memberCount.toLocaleString()} members • {hoveredCommunity.businessCount} businesses
          </Typography>
        </Box>
      )}

      {/* POI Toggle Control */}
      <Box
        sx={{
          position: 'absolute',
          top: 16,
          right: 16,
          zIndex: 1000,
          bgcolor: 'background.paper',
          borderRadius: 2,
          boxShadow: 3,
          p: 2,
        }}
      >
        <FormControlLabel
          control={
            <Switch
              checked={poisVisible}
              onChange={(e) => setPoisVisible(e.target.checked)}
              size="small"
            />
          }
          label={
            <Typography variant="caption" color="text.secondary">
              Show Points of Interest
            </Typography>
          }
          labelPlacement="start"
        />
      </Box>

      {/* Map legend */}
      <Box
        sx={{
          position: 'absolute',
          bottom: 16,
          right: 16,
          zIndex: 1000,
          bgcolor: 'background.paper',
          borderRadius: 2,
          boxShadow: 3,
          p: 2,
        }}
      >
        <Typography variant="caption" color="text.secondary" sx={{ mb: 1, display: 'block', fontWeight: 'bold' }}>
          Legend
        </Typography>

        {/* Community Types */}
        <Typography variant="caption" color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
          Communities
        </Typography>
        <Stack spacing={0.5} mb={2}>
          <Stack direction="row" alignItems="center" spacing={1}>
            <Box sx={{ width: 12, height: 12, bgcolor: '#1976d2', borderRadius: '50%' }} />
            <Typography variant="caption">Urban</Typography>
          </Stack>
          <Stack direction="row" alignItems="center" spacing={1}>
            <Box sx={{ width: 12, height: 12, bgcolor: '#9c27b0', borderRadius: '50%' }} />
            <Typography variant="caption">Arts</Typography>
          </Stack>
          <Stack direction="row" alignItems="center" spacing={1}>
            <Box sx={{ width: 12, height: 12, bgcolor: '#f57c00', borderRadius: '50%' }} />
            <Typography variant="caption">Entertainment</Typography>
          </Stack>
        </Stack>

        {/* POI Types */}
        {poisVisible && (
          <>
            <Typography variant="caption" color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
              Points of Interest
            </Typography>
            <Stack spacing={0.5}>
              <Stack direction="row" alignItems="center" spacing={1}>
                <Box sx={{ width: 12, height: 12, bgcolor: '#5d4037', borderRadius: '50%' }} />
                <Typography variant="caption">Monuments</Typography>
              </Stack>
              <Stack direction="row" alignItems="center" spacing={1}>
                <Box sx={{ width: 12, height: 12, bgcolor: '#388e3c', borderRadius: '50%' }} />
                <Typography variant="caption">Parks</Typography>
              </Stack>
              <Stack direction="row" alignItems="center" spacing={1}>
                <Box sx={{ width: 12, height: 12, bgcolor: '#1976d2', borderRadius: '50%' }} />
                <Typography variant="caption">Libraries</Typography>
              </Stack>
            </Stack>
          </>
        )}
      </Box>
    </Box>
  );
}