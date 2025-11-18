use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;

use super::pages::AppState;

/// Navigation fragment
pub async fn nav_fragment(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(r#"
    <div class="container mx-auto px-4 py-4">
        <div class="flex items-center justify-between">
            <a href="/" class="text-xl font-bold text-gray-800 hover:text-blue-600">
                Community Manager
            </a>
            <div class="flex gap-6">
                <a href="/communities" class="text-gray-600 hover:text-blue-600">Communities</a>
                <a href="/businesses" class="text-gray-600 hover:text-blue-600">Businesses</a>
                <a href="/governance" class="text-gray-600 hover:text-blue-600">Governance</a>
                <a href="/auth/login" class="text-blue-600 hover:text-blue-700 font-medium">Sign In</a>
            </div>
        </div>
    </div>
    "#.to_string())
}

/// Recent communities fragment
pub async fn recent_communities(State(_state): State<Arc<AppState>>) -> Html<String> {
    // TODO: Fetch from database
    Html(r#"
    <div class="community-card fade-in">
        <h3 class="text-xl font-bold mb-2">Tech Community Milano</h3>
        <p class="text-gray-600 mb-4">A community for tech enthusiasts in Milan</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 245 members</span>
            <a href="/communities/tech-milano" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    <div class="community-card fade-in" style="animation-delay: 100ms;">
        <h3 class="text-xl font-bold mb-2">Green Living Roma</h3>
        <p class="text-gray-600 mb-4">Sustainable living and eco-friendly practices</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 189 members</span>
            <a href="/communities/green-roma" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    <div class="community-card fade-in" style="animation-delay: 200ms;">
        <h3 class="text-xl font-bold mb-2">Startup Torino</h3>
        <p class="text-gray-600 mb-4">Connect with entrepreneurs and innovators</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
            <span>👥 312 members</span>
            <a href="/communities/startup-torino" class="text-blue-600 hover:text-blue-700">View →</a>
        </div>
    </div>
    "#.to_string())
}

#[derive(Deserialize)]
pub struct CommunitiesQuery {
    #[serde(default)]
    q: String,
    #[serde(default)]
    filter: String,
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 {
    1
}

/// Communities list fragment
pub async fn communities_list(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<CommunitiesQuery>,
) -> Html<String> {
    // TODO: Fetch from database with filters
    let filter_text = if !query.q.is_empty() {
        format!(" matching '{}'", query.q)
    } else {
        String::new()
    };
    
    Html(format!(r#"
    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Tech Community Milano</h3>
            <p class="text-gray-600 mb-4">A community for tech enthusiasts in Milan{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 245 members</span>
                <a href="/communities/tech-milano" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Green Living Roma</h3>
            <p class="text-gray-600 mb-4">Sustainable living and eco-friendly practices{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 189 members</span>
                <a href="/communities/green-roma" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
        <div class="community-card">
            <h3 class="text-xl font-bold mb-2">Startup Torino</h3>
            <p class="text-gray-600 mb-4">Connect with entrepreneurs and innovators{}</p>
            <div class="flex items-center justify-between text-sm text-gray-500">
                <span>👥 312 members</span>
                <a href="/communities/startup-torino" class="text-blue-600 hover:text-blue-700">View →</a>
            </div>
        </div>
    </div>
    "#, filter_text, filter_text, filter_text))
}

/// Chat room header fragment
pub async fn chat_header(State(_state): State<Arc<AppState>>) -> Html<String> {
    Html(r#"
    <div class="flex items-center justify-between">
        <div>
            <h2 class="text-xl font-bold">Tech Community Chat</h2>
            <p class="text-sm text-gray-500">👥 12 members online</p>
        </div>
        <div class="flex gap-2">
            <button class="px-3 py-1 text-sm text-gray-600 hover:text-gray-900">
                ℹ️ Info
            </button>
            <button class="px-3 py-1 text-sm text-gray-600 hover:text-gray-900">
                ⚙️ Settings
            </button>
        </div>
    </div>
    "#.to_string())
}
