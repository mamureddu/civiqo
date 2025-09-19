'use client';

import { useEffect, useRef } from 'react';
import { MapContainer, TileLayer, Marker, Popup, useMap } from 'react-leaflet';
import { Icon, LatLngBounds } from 'leaflet';
import { Box, Typography, Button, Chip, Rating } from '@mui/material';
import { Business as BusinessIcon, LocationOn as LocationIcon } from '@mui/icons-material';
import type { Business } from '@/types/api';
import 'leaflet/dist/leaflet.css';

// Fix for default markers in React Leaflet
delete (Icon.Default.prototype as any)._getIconUrl;
Icon.Default.mergeOptions({
  iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon-2x.png',
  iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon.png',
  shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-shadow.png',
});

// Custom business marker icon
const businessIcon = new Icon({
  iconUrl: 'data:image/svg+xml;base64,' + btoa(`
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#1976d2" width="32" height="32">
      <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/>
    </svg>
  `),
  iconSize: [32, 32],
  iconAnchor: [16, 32],
  popupAnchor: [0, -32],
});

interface BusinessMapProps {
  businesses: Business[];
  selectedBusiness?: Business | null;
  onBusinessSelect?: (business: Business | null) => void;
}

// Component to fit map bounds to show all businesses
function MapBoundsFitter({ businesses }: { businesses: Business[] }) {
  const map = useMap();

  useEffect(() => {
    if (businesses.length > 0) {
      const validBusinesses = businesses.filter(b => b.latitude && b.longitude);

      if (validBusinesses.length > 0) {
        const bounds = new LatLngBounds(
          validBusinesses.map(business => [business.latitude!, business.longitude!])
        );

        // Add some padding around the bounds
        map.fitBounds(bounds, { padding: [20, 20] });
      }
    }
  }, [businesses, map]);

  return null;
}

export default function BusinessMap({ businesses, selectedBusiness, onBusinessSelect }: BusinessMapProps) {
  const mapRef = useRef<any>(null);

  // Filter businesses that have coordinates
  const mappableBusinesses = businesses.filter(business =>
    business.latitude && business.longitude
  );

  // Default center (can be adjusted based on your community's location)
  const defaultCenter: [number, number] = [40.7128, -74.0060]; // New York City

  useEffect(() => {
    if (selectedBusiness && selectedBusiness.latitude && selectedBusiness.longitude && mapRef.current) {
      const map = mapRef.current;
      map.setView([selectedBusiness.latitude, selectedBusiness.longitude], 15);
    }
  }, [selectedBusiness]);

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

        {/* Fit bounds to show all businesses */}
        <MapBoundsFitter businesses={mappableBusinesses} />

        {/* Business markers */}
        {mappableBusinesses.map((business) => (
          <Marker
            key={business.id}
            position={[business.latitude!, business.longitude!]}
            icon={businessIcon}
            eventHandlers={{
              click: () => onBusinessSelect?.(business),
            }}
          >
            <Popup>
              <Box sx={{ minWidth: 200, p: 1 }}>
                <Typography variant="h6" fontWeight="bold" gutterBottom>
                  {business.name}
                </Typography>

                <Chip
                  label={business.category}
                  size="small"
                  color="primary"
                  variant="outlined"
                  sx={{ mb: 1 }}
                />

                {business.rating && (
                  <Box display="flex" alignItems="center" mb={1}>
                    <Rating value={business.rating} readOnly size="small" />
                    <Typography variant="body2" color="text.secondary" ml={1}>
                      ({business.rating})
                    </Typography>
                  </Box>
                )}

                {business.description && (
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
                    {business.description}
                  </Typography>
                )}

                {business.address && (
                  <Box display="flex" alignItems="center" mb={1}>
                    <LocationIcon sx={{ fontSize: 16, color: 'text.secondary', mr: 0.5 }} />
                    <Typography variant="caption" color="text.secondary">
                      {business.address}
                    </Typography>
                  </Box>
                )}

                <Button
                  variant="outlined"
                  size="small"
                  href={`/businesses/${business.id}`}
                  fullWidth
                  sx={{ mt: 1 }}
                >
                  View Details
                </Button>
              </Box>
            </Popup>
          </Marker>
        ))}
      </MapContainer>

      {/* Show message if no businesses have coordinates */}
      {mappableBusinesses.length === 0 && businesses.length > 0 && (
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
          <BusinessIcon sx={{ fontSize: 48, color: 'text.secondary', mb: 1 }} />
          <Typography variant="h6" color="text.secondary" gutterBottom>
            No locations to display
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Businesses need coordinates to appear on the map
          </Typography>
        </Box>
      )}
    </Box>
  );
}