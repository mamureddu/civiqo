'use client';

import { useEffect, useRef } from 'react';
import { MapContainer, TileLayer, Marker, Popup, useMap } from 'react-leaflet';
import { Icon, LatLngBounds } from 'leaflet';
import {
  Box,
  Typography,
  Button,
  Chip,
  Rating,
  Stack,
  Avatar,
  IconButton,
} from '@mui/material';
import {
  Museum as MuseumIcon,
  Park as ParkIcon,
  LocalLibrary as LibraryIcon,
  Church as ChurchIcon,
  AccountBalance as MonumentIcon,
  Stadium as StadiumIcon,
  TheaterComedy as TheaterIcon,
  Info as InfoIcon,
  LocationOn as LocationIcon,
  AccessTime as TimeIcon,
  Star as StarIcon,
  Favorite as FavoriteIcon,
  FavoriteBorder as FavoriteBorderIcon,
  Share as ShareIcon,
} from '@mui/icons-material';
import 'leaflet/dist/leaflet.css';

// Fix for default markers in React Leaflet
delete (Icon.Default.prototype as any)._getIconUrl;
Icon.Default.mergeOptions({
  iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon-2x.png',
  iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon.png',
  shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-shadow.png',
});

// POI Interface
interface PointOfInterest {
  id: string;
  name: string;
  description: string;
  category: string;
  address: string;
  latitude: number;
  longitude: number;
  phone?: string;
  website?: string;
  opening_hours?: string;
  admission_fee?: string;
  image_url?: string;
  rating?: number;
  review_count: number;
  is_verified: boolean;
  created_at: string;
  community_id: string;
}

interface POIMapProps {
  pois: PointOfInterest[];
  selectedPOI?: PointOfInterest | null;
  onPOISelect?: (poi: PointOfInterest | null) => void;
}

// Create custom POI markers based on category
const createPOIIcon = (category: string) => {
  const getIconSvg = (category: string) => {
    const iconColor = getCategoryColor(category);
    const iconPath = getCategoryIconPath(category);

    return `
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="${iconColor}" width="36" height="36">
        <circle cx="12" cy="12" r="10" fill="${iconColor}" stroke="white" stroke-width="2"/>
        <g transform="translate(6, 6)">
          ${iconPath}
        </g>
      </svg>
    `;
  };

  return new Icon({
    iconUrl: 'data:image/svg+xml;base64,' + btoa(getIconSvg(category)),
    iconSize: [36, 36],
    iconAnchor: [18, 36],
    popupAnchor: [0, -36],
  });
};

const getCategoryColor = (category: string) => {
  switch (category) {
    case 'museum': return '#8e24aa';
    case 'park': return '#388e3c';
    case 'library': return '#1976d2';
    case 'religious': return '#f57c00';
    case 'monument': return '#5d4037';
    case 'sports': return '#d32f2f';
    case 'theater': return '#7b1fa2';
    default: return '#616161';
  }
};

const getCategoryIconPath = (category: string) => {
  // Simplified icon paths for different POI categories
  switch (category) {
    case 'museum':
      return '<path fill="white" d="M2 2h8v2H8v8h2v2H2v-2h2V4H2V2zm4 4h2v4H6V6z"/>';
    case 'park':
      return '<path fill="white" d="M3 3h6c1.1 0 2 .9 2 2v6c0 1.1-.9 2-2 2H3c-1.1 0-2-.9-2-2V5c0-1.1.9-2 2-2z"/>';
    case 'library':
      return '<path fill="white" d="M2 2h8v10H2V2zm2 2v6h4V4H4z"/>';
    case 'religious':
      return '<path fill="white" d="M5 1v3H3v3h2v5h2V7h2V4H7V1H5z"/>';
    case 'monument':
      return '<path fill="white" d="M1 10h10v2H1v-2zm2-8h6v6H3V2z"/>';
    case 'sports':
      return '<path fill="white" d="M6 2c2.2 0 4 1.8 4 4s-1.8 4-4 4-4-1.8-4-4 1.8-4 4-4z"/>';
    case 'theater':
      return '<path fill="white" d="M2 2h8v8H2V2zm2 2v4h4V4H4z"/>';
    default:
      return '<circle fill="white" cx="6" cy="6" r="4"/>';
  }
};

const getCategoryIcon = (category: string) => {
  switch (category) {
    case 'museum': return <MuseumIcon fontSize="small" />;
    case 'park': return <ParkIcon fontSize="small" />;
    case 'library': return <LibraryIcon fontSize="small" />;
    case 'religious': return <ChurchIcon fontSize="small" />;
    case 'monument': return <MonumentIcon fontSize="small" />;
    case 'sports': return <StadiumIcon fontSize="small" />;
    case 'theater': return <TheaterIcon fontSize="small" />;
    default: return <InfoIcon fontSize="small" />;
  }
};

const getCategoryLabel = (category: string) => {
  switch (category) {
    case 'museum': return 'Museum';
    case 'park': return 'Park';
    case 'library': return 'Library';
    case 'religious': return 'Religious Site';
    case 'monument': return 'Monument';
    case 'sports': return 'Sports';
    case 'theater': return 'Theater';
    default: return 'Other';
  }
};

// Component to fit map bounds to show all POIs
function MapBoundsFitter({ pois }: { pois: PointOfInterest[] }) {
  const map = useMap();

  useEffect(() => {
    if (pois.length > 0) {
      const bounds = new LatLngBounds(
        pois.map(poi => [poi.latitude, poi.longitude])
      );

      // Add some padding around the bounds
      map.fitBounds(bounds, { padding: [20, 20] });
    }
  }, [pois, map]);

  return null;
}

export default function POIMap({ pois, selectedPOI, onPOISelect }: POIMapProps) {
  const mapRef = useRef<any>(null);

  // Default center (Milan, Italy)
  const defaultCenter: [number, number] = [45.4642, 9.1900];

  useEffect(() => {
    if (selectedPOI && mapRef.current) {
      const map = mapRef.current;
      map.setView([selectedPOI.latitude, selectedPOI.longitude], 16);
    }
  }, [selectedPOI]);

  return (
    <Box sx={{ height: '100%', width: '100%', position: 'relative' }}>
      <MapContainer
        ref={mapRef}
        center={defaultCenter}
        zoom={13}
        style={{ height: '100%', width: '100%' }}
        scrollWheelZoom={true}
      >
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />

        {/* Fit bounds to show all POIs */}
        <MapBoundsFitter pois={pois} />

        {/* POI markers */}
        {pois.map((poi) => (
          <Marker
            key={poi.id}
            position={[poi.latitude, poi.longitude]}
            icon={createPOIIcon(poi.category)}
            eventHandlers={{
              click: () => onPOISelect?.(poi),
            }}
          >
            <Popup maxWidth={350}>
              <Box sx={{ minWidth: 300, p: 1 }}>
                {poi.image_url && (
                  <Box
                    component="img"
                    src={poi.image_url}
                    alt={poi.name}
                    sx={{
                      width: '100%',
                      height: 120,
                      objectFit: 'cover',
                      borderRadius: 1,
                      mb: 2
                    }}
                  />
                )}

                <Box display="flex" alignItems="flex-start" justifyContent="space-between" mb={1}>
                  <Typography variant="h6" fontWeight="bold" sx={{ flex: 1, mr: 1 }}>
                    {poi.name}
                  </Typography>
                  <Box display="flex" flexDirection="column" alignItems="flex-end" gap={0.5}>
                    <Chip
                      icon={getCategoryIcon(poi.category)}
                      label={getCategoryLabel(poi.category)}
                      size="small"
                      sx={{
                        bgcolor: getCategoryColor(poi.category),
                        color: 'white',
                        '& .MuiChip-icon': { color: 'white' }
                      }}
                    />
                    {poi.is_verified && (
                      <Chip
                        label="Verified"
                        size="small"
                        color="success"
                        variant="outlined"
                      />
                    )}
                  </Box>
                </Box>

                {poi.rating && (
                  <Box display="flex" alignItems="center" mb={1}>
                    <Rating value={poi.rating} readOnly size="small" precision={0.1} />
                    <Typography variant="body2" color="text.secondary" ml={1}>
                      {poi.rating} ({poi.review_count.toLocaleString()})
                    </Typography>
                  </Box>
                )}

                <Typography
                  variant="body2"
                  color="text.secondary"
                  sx={{
                    mb: 2,
                    display: '-webkit-box',
                    overflow: 'hidden',
                    WebkitBoxOrient: 'vertical',
                    WebkitLineClamp: 2,
                  }}
                >
                  {poi.description}
                </Typography>

                <Stack spacing={1} mb={2}>
                  <Stack direction="row" spacing={1} alignItems="center">
                    <LocationIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                    <Typography variant="caption" color="text.secondary">
                      {poi.address}
                    </Typography>
                  </Stack>

                  {poi.opening_hours && (
                    <Stack direction="row" spacing={1} alignItems="center">
                      <TimeIcon sx={{ fontSize: 16, color: 'text.secondary' }} />
                      <Typography variant="caption" color="text.secondary">
                        {poi.opening_hours}
                      </Typography>
                    </Stack>
                  )}

                  {poi.admission_fee && (
                    <Stack direction="row" spacing={1} alignItems="center">
                      <StarIcon sx={{ fontSize: 16, color: 'warning.main' }} />
                      <Typography variant="caption" color="text.secondary">
                        {poi.admission_fee}
                      </Typography>
                    </Stack>
                  )}
                </Stack>

                <Stack direction="row" spacing={1} justifyContent="space-between" alignItems="center">
                  <Button
                    variant="outlined"
                    size="small"
                    href={`/poi/${poi.id}`}
                    sx={{ flex: 1 }}
                  >
                    View Details
                  </Button>

                  <Box>
                    <IconButton size="small">
                      <FavoriteBorderIcon fontSize="small" />
                    </IconButton>
                    <IconButton size="small">
                      <ShareIcon fontSize="small" />
                    </IconButton>
                  </Box>
                </Stack>
              </Box>
            </Popup>
          </Marker>
        ))}
      </MapContainer>

      {/* Legend */}
      <Box
        sx={{
          position: 'absolute',
          bottom: 16,
          left: 16,
          zIndex: 1000,
          bgcolor: 'background.paper',
          borderRadius: 2,
          boxShadow: 3,
          p: 2,
          maxWidth: 200,
        }}
      >
        <Typography variant="caption" color="text.secondary" sx={{ mb: 1, display: 'block', fontWeight: 'bold' }}>
          Points of Interest
        </Typography>
        <Stack spacing={0.5}>
          {['museum', 'park', 'library', 'monument'].map((category) => (
            <Stack key={category} direction="row" alignItems="center" spacing={1}>
              <Box
                sx={{
                  width: 12,
                  height: 12,
                  bgcolor: getCategoryColor(category),
                  borderRadius: '50%'
                }}
              />
              <Typography variant="caption">{getCategoryLabel(category)}</Typography>
            </Stack>
          ))}
        </Stack>
      </Box>

      {/* Show message if no POIs */}
      {pois.length === 0 && (
        <Box
          sx={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            bgcolor: 'background.paper',
            p: 3,
            borderRadius: 1,
            boxShadow: 2,
            textAlign: 'center',
            zIndex: 1000,
          }}
        >
          <MuseumIcon sx={{ fontSize: 48, color: 'text.secondary', mb: 1 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            No points of interest
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Add some POIs to see them on the map
          </Typography>
        </Box>
      )}
    </Box>
  );
}