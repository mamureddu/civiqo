-- ============================================================================
-- Migration 012: Business Reviews & Orders
-- Phase 4: Complete Business Features
-- ============================================================================

-- ============================================================================
-- PART 1: Reviews System
-- ============================================================================

CREATE TABLE IF NOT EXISTS business_reviews (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    business_id BIGINT NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    content TEXT,
    is_verified_purchase BOOLEAN DEFAULT false,
    helpful_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(business_id, user_id)
);

-- Review responses (business owner can respond)
CREATE TABLE IF NOT EXISTS review_responses (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    review_id BIGINT NOT NULL REFERENCES business_reviews(id) ON DELETE CASCADE,
    responder_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(review_id)
);

-- ============================================================================
-- PART 2: Orders System
-- ============================================================================

CREATE TABLE IF NOT EXISTS orders (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    business_id BIGINT NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'preparing', 'ready', 'delivered', 'cancelled')),
    total_amount NUMERIC(10, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'EUR',
    notes TEXT,
    delivery_address TEXT,
    delivery_type VARCHAR(20) DEFAULT 'pickup' CHECK (delivery_type IN ('pickup', 'delivery')),
    estimated_ready_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS order_items (
    id BIGINT PRIMARY KEY DEFAULT unique_rowid(),
    order_id BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id BIGINT NOT NULL REFERENCES business_products(id),
    product_name VARCHAR(255) NOT NULL, -- Snapshot at order time
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(10, 2) NOT NULL,
    total_price NUMERIC(10, 2) NOT NULL,
    notes TEXT
);

-- ============================================================================
-- PART 3: Extend businesses table
-- ============================================================================

-- Add rating cache to businesses
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS rating_avg NUMERIC(3, 2) DEFAULT 0;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS review_count INT DEFAULT 0;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS order_count INT DEFAULT 0;

-- Add verification and featured flags
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS is_verified BOOLEAN DEFAULT false;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS is_featured BOOLEAN DEFAULT false;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS verified_at TIMESTAMPTZ;

-- Add contact info
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS phone VARCHAR(50);
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS email VARCHAR(255);
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS website VARCHAR(500);

-- Add social links
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS social_links JSONB DEFAULT '{}';

-- ============================================================================
-- PART 4: Indexes
-- ============================================================================

-- Reviews indexes
CREATE INDEX IF NOT EXISTS idx_business_reviews_business_id ON business_reviews(business_id);
CREATE INDEX IF NOT EXISTS idx_business_reviews_user_id ON business_reviews(user_id);
CREATE INDEX IF NOT EXISTS idx_business_reviews_rating ON business_reviews(rating);
CREATE INDEX IF NOT EXISTS idx_business_reviews_created_at ON business_reviews(created_at DESC);

-- Orders indexes
CREATE INDEX IF NOT EXISTS idx_orders_business_id ON orders(business_id);
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_product_id ON order_items(product_id);

-- Business search optimization
CREATE INDEX IF NOT EXISTS idx_businesses_rating_avg ON businesses(rating_avg DESC);
CREATE INDEX IF NOT EXISTS idx_businesses_is_verified ON businesses(is_verified);
CREATE INDEX IF NOT EXISTS idx_businesses_is_featured ON businesses(is_featured);

-- ============================================================================
-- PART 5: Triggers for rating cache
-- ============================================================================

-- Note: CockroachDB doesn't support triggers, so rating updates will be done in application code
-- The rating_avg and review_count fields will be updated when reviews are added/modified/deleted

-- ============================================================================
-- Migration complete
-- ============================================================================
