-- ============================================================================
-- Migration 003: Businesses, Products, Reviews & Orders
-- ============================================================================
-- Tables: businesses, business_hours, business_images, business_products,
--         business_reviews, review_responses, orders, order_items
-- 
-- ID Strategy:
-- - All tables use UUID (app generates via Uuid::now_v7())
-- - These are core business entities that may need federation
-- ============================================================================

-- ============================================================================
-- BUSINESSES (UUID - app generates)
-- ============================================================================

CREATE TABLE businesses (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    address TEXT,
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    phone VARCHAR(50),
    email VARCHAR(255),
    website VARCHAR(500),
    social_links JSONB DEFAULT '{}',
    is_verified BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    verified_at TIMESTAMPTZ,
    rating_avg NUMERIC(3, 2) DEFAULT 0,
    review_count INT DEFAULT 0,
    order_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_businesses_community ON businesses(community_id);
CREATE INDEX idx_businesses_owner ON businesses(owner_id);
CREATE INDEX idx_businesses_name ON businesses(name);
CREATE INDEX idx_businesses_category ON businesses(category);
CREATE INDEX idx_businesses_is_active ON businesses(is_active);
CREATE INDEX idx_businesses_is_verified ON businesses(is_verified);
CREATE INDEX idx_businesses_is_featured ON businesses(is_featured);
CREATE INDEX idx_businesses_rating_avg ON businesses(rating_avg DESC);

-- ============================================================================
-- BUSINESS HOURS (UUID - app generates)
-- ============================================================================

CREATE TABLE business_hours (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    day_of_week INT NOT NULL CHECK (day_of_week >= 0 AND day_of_week <= 6),
    open_time TIME,
    close_time TIME,
    is_closed BOOLEAN DEFAULT false,
    UNIQUE(business_id, day_of_week)
);

CREATE INDEX idx_business_hours_business ON business_hours(business_id);

-- ============================================================================
-- BUSINESS IMAGES (UUID - app generates)
-- ============================================================================

CREATE TABLE business_images (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    alt_text VARCHAR(255),
    image_type VARCHAR(50) DEFAULT 'gallery',  -- 'logo', 'cover', 'gallery'
    display_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_business_images_business ON business_images(business_id);
CREATE INDEX idx_business_images_type ON business_images(image_type);

-- ============================================================================
-- BUSINESS PRODUCTS (UUID - app generates)
-- ============================================================================

CREATE TABLE business_products (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    product_name VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2),
    currency VARCHAR(3) DEFAULT 'EUR',
    is_available BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_business_products_business ON business_products(business_id);
CREATE INDEX idx_business_products_available ON business_products(is_available);

-- ============================================================================
-- BUSINESS REVIEWS (UUID - app generates)
-- ============================================================================

CREATE TABLE business_reviews (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    content TEXT,
    is_verified_purchase BOOLEAN DEFAULT false,
    helpful_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(business_id, user_id)  -- One review per user per business
);

CREATE INDEX idx_business_reviews_business ON business_reviews(business_id);
CREATE INDEX idx_business_reviews_user ON business_reviews(user_id);
CREATE INDEX idx_business_reviews_rating ON business_reviews(rating);
CREATE INDEX idx_business_reviews_created_at ON business_reviews(created_at DESC);

-- ============================================================================
-- REVIEW RESPONSES (UUID - app generates)
-- ============================================================================

CREATE TABLE review_responses (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    review_id UUID NOT NULL REFERENCES business_reviews(id) ON DELETE CASCADE,
    responder_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(review_id)  -- One response per review
);

CREATE INDEX idx_review_responses_review ON review_responses(review_id);

-- ============================================================================
-- ORDERS (UUID - app generates)
-- ============================================================================

CREATE TABLE orders (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'pending' 
        CHECK (status IN ('pending', 'confirmed', 'preparing', 'ready', 'delivered', 'cancelled')),
    total_amount NUMERIC(10, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'EUR',
    notes TEXT,
    delivery_address TEXT,
    delivery_type VARCHAR(20) DEFAULT 'pickup' 
        CHECK (delivery_type IN ('pickup', 'delivery')),
    estimated_ready_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_orders_business ON orders(business_id);
CREATE INDEX idx_orders_user ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);

-- ============================================================================
-- ORDER ITEMS (UUID - app generates)
-- ============================================================================

CREATE TABLE order_items (
    id UUID PRIMARY KEY,  -- App generates via Uuid::now_v7()
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES business_products(id),
    product_name VARCHAR(255) NOT NULL,  -- Snapshot at order time
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(10, 2) NOT NULL,
    total_price NUMERIC(10, 2) NOT NULL,
    notes TEXT
);

CREATE INDEX idx_order_items_order ON order_items(order_id);
CREATE INDEX idx_order_items_product ON order_items(product_id);
