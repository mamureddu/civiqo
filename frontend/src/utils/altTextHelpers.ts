/**
 * Utility functions for generating accessible alt text for images
 */

/**
 * Generate alt text for business images
 */
export function getBusinessImageAlt(businessName: string): string {
  return `Photo of ${businessName} business`;
}

/**
 * Generate alt text for user profile images
 */
export function getUserAvatarAlt(userName: string): string {
  return `Profile photo of ${userName}`;
}

/**
 * Generate alt text for community images
 */
export function getCommunityImageAlt(communityName: string): string {
  return `Image representing ${communityName} community`;
}

/**
 * Generate alt text for feed item images
 */
export function getFeedItemImageAlt(title: string): string {
  return `Image for ${title}`;
}

/**
 * Generate alt text for POI (Points of Interest) images
 */
export function getPOIImageAlt(poiName: string, category?: string): string {
  return category
    ? `Photo of ${poiName}, a ${category.toLowerCase()}`
    : `Photo of ${poiName}`;
}

/**
 * Generate alt text for event images
 */
export function getEventImageAlt(eventTitle: string): string {
  return `Image for event: ${eventTitle}`;
}

/**
 * Generate descriptive alt text for generic images
 * Use when specific context is not available
 */
export function getGenericImageAlt(description?: string): string {
  return description || 'Image';
}

/**
 * Generate alt text for decorative images (empty string for screen readers to ignore)
 */
export function getDecorativeImageAlt(): string {
  return '';
}