//! Internationalization (i18n) module for Civiqo
//!
//! This module provides multi-language support using Mozilla's Fluent localization system.
//! It includes:
//! - Static locale loading from .ftl files
//! - Language detection from cookies and Accept-Language headers
//! - Middleware for injecting locale into request extensions
//! - Helper functions for template integration

use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use fluent_templates::{static_loader, Loader, fluent_bundle::FluentValue};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

/// Supported languages
pub const SUPPORTED_LANGUAGES: &[&str] = &["it", "en"];
pub const DEFAULT_LANGUAGE: &str = "it";
pub const LANGUAGE_COOKIE_NAME: &str = "civiqo_lang";

// Load all locale files at compile time
static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "it",
    };
}

/// Locale wrapper for request extensions
#[derive(Clone, Debug)]
pub struct Locale {
    pub lang_id: LanguageIdentifier,
    pub code: String,
}

impl Locale {
    pub fn new(code: &str) -> Self {
        let lang_id: LanguageIdentifier = code.parse().unwrap_or_else(|_| {
            DEFAULT_LANGUAGE.parse().expect("Default language must be valid")
        });
        Self {
            lang_id,
            code: code.to_string(),
        }
    }

    /// Get a translated string by key
    pub fn t(&self, key: &str) -> String {
        LOCALES.lookup(&self.lang_id, key)
    }

    /// Get a translated string with arguments
    pub fn t_with_args(&self, key: &str, args: &HashMap<String, FluentValue>) -> String {
        LOCALES.lookup_with_args(&self.lang_id, key, args)
    }

    /// Check if this is the default language
    pub fn is_default(&self) -> bool {
        self.code == DEFAULT_LANGUAGE
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::new(DEFAULT_LANGUAGE)
    }
}

/// Extract language preference from request
/// Priority: 1. Cookie, 2. Accept-Language header, 3. Default
pub fn extract_language(headers: &HeaderMap) -> String {
    // 1. Check cookie first
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(value) = cookie.strip_prefix(&format!("{}=", LANGUAGE_COOKIE_NAME)) {
                    let lang = value.trim();
                    if SUPPORTED_LANGUAGES.contains(&lang) {
                        return lang.to_string();
                    }
                }
            }
        }
    }

    // 2. Check Accept-Language header
    if let Some(accept_lang) = headers.get("accept-language") {
        if let Ok(accept_str) = accept_lang.to_str() {
            // Parse Accept-Language header (e.g., "it-IT,it;q=0.9,en-US;q=0.8,en;q=0.7")
            let languages = accept_language::parse(accept_str);
            for lang in languages {
                // Extract primary language code (e.g., "it" from "it-IT")
                let primary = lang.split('-').next().unwrap_or(&lang);
                if SUPPORTED_LANGUAGES.contains(&primary) {
                    return primary.to_string();
                }
            }
        }
    }

    // 3. Default to Italian
    DEFAULT_LANGUAGE.to_string()
}

/// Middleware to inject locale into request extensions
pub async fn locale_middleware(mut request: Request, next: Next) -> Response {
    let lang = extract_language(request.headers());
    let locale = Locale::new(&lang);
    request.extensions_mut().insert(locale);
    next.run(request).await
}

/// Get all available languages for the language switcher
pub fn get_available_languages() -> Vec<LanguageInfo> {
    vec![
        LanguageInfo {
            code: "it".to_string(),
            name: "Italiano".to_string(),
            flag: "🇮🇹".to_string(),
        },
        LanguageInfo {
            code: "en".to_string(),
            name: "English".to_string(),
            flag: "🇬🇧".to_string(),
        },
    ]
}

/// Language information for UI display
#[derive(Clone, Debug, serde::Serialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub flag: String,
}

/// Helper to create translation context for Tera templates
pub fn create_translation_context(locale: &Locale) -> HashMap<String, String> {
    let mut context = HashMap::new();
    context.insert("lang".to_string(), locale.code.clone());
    context.insert("lang_name".to_string(), 
        get_available_languages()
            .iter()
            .find(|l| l.code == locale.code)
            .map(|l| l.name.clone())
            .unwrap_or_else(|| locale.code.clone())
    );
    context
}

/// Macro to simplify translation calls in handlers
#[macro_export]
macro_rules! t {
    ($locale:expr, $key:expr) => {
        $locale.t($key)
    };
    ($locale:expr, $key:expr, $($arg_name:expr => $arg_value:expr),*) => {{
        let mut args = std::collections::HashMap::new();
        $(
            args.insert($arg_name.to_string(), fluent_templates::fluent_bundle::FluentValue::from($arg_value));
        )*
        $locale.t_with_args($key, &args)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_locale_creation() {
        let locale = Locale::new("it");
        assert_eq!(locale.code, "it");
        assert!(locale.is_default());

        let locale_en = Locale::new("en");
        assert_eq!(locale_en.code, "en");
        assert!(!locale_en.is_default());
    }

    #[test]
    fn test_locale_default() {
        let locale = Locale::default();
        assert_eq!(locale.code, DEFAULT_LANGUAGE);
    }

    #[test]
    fn test_extract_language_from_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static("civiqo_lang=en; other=value"));
        assert_eq!(extract_language(&headers), "en");
    }

    #[test]
    fn test_extract_language_from_accept_header() {
        let mut headers = HeaderMap::new();
        headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9,it;q=0.8"));
        assert_eq!(extract_language(&headers), "en");
    }

    #[test]
    fn test_extract_language_default() {
        let headers = HeaderMap::new();
        assert_eq!(extract_language(&headers), DEFAULT_LANGUAGE);
    }

    #[test]
    fn test_translation_basic() {
        let locale = Locale::new("it");
        // This will return the key in brackets if not found during tests
        // In production with proper locale loading, it returns the translation
        let result = locale.t("nav-home");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_available_languages() {
        let languages = get_available_languages();
        assert_eq!(languages.len(), 2);
        assert!(languages.iter().any(|l| l.code == "it"));
        assert!(languages.iter().any(|l| l.code == "en"));
    }
}
