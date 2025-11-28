//! Tera integration for i18n
//!
//! This module provides helpers to integrate Fluent translations with Tera templates.

use crate::i18n::{Locale, LOCALES, get_available_languages};
use fluent_templates::Loader;
use tera::Context;
use std::collections::HashMap;

/// Add i18n context to a Tera context
/// This adds the current language and a translations object
pub fn add_i18n_context(ctx: &mut Context, locale: &Locale) {
    // Add current language info
    ctx.insert("lang", &locale.code);
    ctx.insert("languages", &get_available_languages());
    
    // Add all translations as a nested object
    // This allows templates to use {{ t.key }} syntax
    let translations = get_translations_for_locale(locale);
    ctx.insert("t", &translations);
}

/// Get all translations for a locale as a HashMap
/// Keys are flattened (e.g., "nav-home" becomes accessible)
fn get_translations_for_locale(locale: &Locale) -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    // List of all translation keys we want to expose
    // This is a subset - add more as needed
    let keys = [
        // Navigation
        "nav-home", "nav-communities", "nav-governance", "nav-businesses",
        "nav-chat", "nav-profile", "nav-search", "nav-notifications",
        
        // Header
        "header-login", "header-logout", "header-register", "header-welcome",
        
        // Actions
        "action-save", "action-cancel", "action-delete", "action-edit",
        "action-create", "action-submit", "action-confirm", "action-back",
        "action-next", "action-close", "action-search", "action-filter",
        "action-load-more", "action-view-all", "action-share",
        
        // States
        "state-loading", "state-saving", "state-empty", "state-error",
        "state-success", "state-no-results",
        
        // Communities
        "communities-title", "communities-subtitle", "communities-search-placeholder",
        "communities-filter-all", "communities-filter-public", "communities-filter-private",
        "communities-filter-my", "communities-empty", "communities-empty-subtitle",
        "community-public", "community-private", "community-verified",
        "community-join", "community-leave", "community-request-join", "community-joined",
        
        // Create Community
        "community-create-title", "community-create-subtitle",
        "community-create-name-label", "community-create-name-placeholder",
        "community-create-name-hint", "community-create-name-validation",
        "community-create-description-label", "community-create-description-placeholder",
        "community-create-description-hint", "community-create-description-max",
        "community-create-privacy-title", "community-create-public-label",
        "community-create-public-hint", "community-create-approval-label",
        "community-create-approval-hint", "community-create-submit",
        "community-create-cancel", "community-create-creating",
        "community-create-success", "community-create-error", "community-create-redirect",
        "community-create-guidelines-title",
        "community-create-guideline-1", "community-create-guideline-2",
        "community-create-guideline-3", "community-create-guideline-4",
        
        // Validation
        "community-name-required", "community-name-min", "community-name-max",
        "community-name-invalid", "community-description-max",
        
        // Dashboard
        "dashboard-title", "dashboard-subtitle",
        "dashboard-stats-communities", "dashboard-stats-posts",
        "dashboard-stats-notifications", "dashboard-stats-activity",
        "dashboard-section-communities", "dashboard-section-communities-empty",
        "dashboard-section-communities-explore", "dashboard-section-communities-create",
        "dashboard-section-activity", "dashboard-section-activity-empty",
        "dashboard-quick-actions", "dashboard-action-create-community",
        "dashboard-action-create-post", "dashboard-action-explore",
        "dashboard-community-view", "dashboard-community-settings",
        
        // Auth
        "auth-login-title", "auth-login-subtitle", "auth-login-submit",
        "auth-login-with-auth0", "auth-login-no-account", "auth-login-register-link",
        "auth-logout-title", "auth-logout-confirm", "auth-logout-submit",
        
        // Profile
        "profile-title", "profile-edit", "profile-save", "profile-saved",
        "profile-stats-communities", "profile-stats-posts",
        "profile-stats-followers", "profile-stats-following",
        
        // Governance
        "governance-title", "governance-subtitle",
        "governance-tab-proposals", "governance-tab-decisions", "governance-tab-polls",
        "proposals-title", "proposals-create", "proposals-empty",
        
        // Businesses
        "businesses-title", "businesses-subtitle", "businesses-search-placeholder",
        "businesses-empty", "businesses-empty-subtitle",
        
        // Chat
        "chat-title", "chat-subtitle", "chat-empty",
        "chat-input-placeholder", "chat-send",
        
        // Posts
        "posts-title", "posts-create", "posts-empty",
        "post-like", "post-comment", "post-share",
        "comments-title", "comments-empty", "comment-placeholder", "comment-submit",
        
        // Errors
        "error-400", "error-401", "error-403", "error-404", "error-500",
        "error-back-home", "error-try-again",
        "error-generic", "error-network",
        
        // Footer
        "footer-privacy", "footer-terms", "footer-contact", "footer-about",
        
        // Accessibility
        "a11y-menu-toggle", "a11y-language-select", "a11y-close-modal",
    ];
    
    for key in keys {
        if let Some(translation) = LOCALES.lookup(&locale.lang_id, key) {
            // Convert key from kebab-case to a valid template key
            // e.g., "nav-home" -> "nav_home" for easier template access
            let template_key = key.replace('-', "_");
            translations.insert(template_key, translation);
        }
    }
    
    translations
}

/// Extractor for Locale from request extensions
pub struct LocaleExtractor(pub Locale);

impl<S> axum::extract::FromRequestParts<S> for LocaleExtractor
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let locale = parts
            .extensions
            .get::<Locale>()
            .cloned()
            .unwrap_or_default();
        Ok(LocaleExtractor(locale))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i18n_context() {
        let locale = Locale::new("it");
        let mut ctx = Context::new();
        add_i18n_context(&mut ctx, &locale);
        
        // Check that lang is set
        assert!(ctx.contains_key("lang"));
        assert!(ctx.contains_key("languages"));
        assert!(ctx.contains_key("t"));
    }

    #[test]
    fn test_translations_map() {
        let locale = Locale::new("it");
        let translations = get_translations_for_locale(&locale);
        
        // Should have some translations
        assert!(!translations.is_empty());
    }
}
