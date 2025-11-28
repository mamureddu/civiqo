# Civiqo - English Translations
# File: businesses.ftl - Local businesses

# =============================================================================
# BUSINESS LIST
# =============================================================================

businesses-title = Local Businesses
businesses-subtitle = Discover businesses in your area
businesses-search-placeholder = Search businesses...
businesses-filter-all = All
businesses-filter-verified = Verified
businesses-filter-category = Category
businesses-sort-rating = Rating
businesses-sort-recent = Most recent
businesses-sort-name = Name A-Z
businesses-empty = No businesses found
businesses-empty-subtitle = Be the first to register your business!

# =============================================================================
# BUSINESS CARD
# =============================================================================

business-verified = Verified
business-rating = { $rating } stars
business-reviews = { $count ->
    [one] { $count } review
   *[other] { $count } reviews
}
business-category-restaurant = Restaurant
business-category-shop = Shop
business-category-service = Services
business-category-health = Health
business-category-education = Education
business-category-entertainment = Entertainment
business-category-other = Other

# =============================================================================
# BUSINESS DETAIL
# =============================================================================

business-detail-about = About
business-detail-products = Products
business-detail-reviews = Reviews
business-detail-contact = Contact
business-detail-hours = Hours

business-about-description = Description
business-about-address = Address
business-about-phone = Phone
business-about-email = Email
business-about-website = Website
business-about-owner = Owner

business-hours-monday = Monday
business-hours-tuesday = Tuesday
business-hours-wednesday = Wednesday
business-hours-thursday = Thursday
business-hours-friday = Friday
business-hours-saturday = Saturday
business-hours-sunday = Sunday
business-hours-closed = Closed
business-hours-open-now = Open now
business-hours-closed-now = Closed now

# =============================================================================
# CREATE BUSINESS
# =============================================================================

business-create-title = Register Your Business
business-create-subtitle = Add your business to the community

business-create-name-label = Business name
business-create-name-placeholder = Your business name
business-create-description-label = Description
business-create-description-placeholder = Describe your business...
business-create-category-label = Category
business-create-address-label = Address
business-create-address-placeholder = Street, number, city
business-create-phone-label = Phone
business-create-email-label = Email
business-create-website-label = Website
business-create-submit = Register Business

business-create-success = Business registered successfully!
business-create-error = Error registering business

# =============================================================================
# PRODUCTS
# =============================================================================

products-title = Products and Services
products-empty = No products available
products-add = Add product

product-name = Name
product-description = Description
product-price = Price
product-available = Available
product-unavailable = Unavailable

product-add-title = Add Product
product-add-name-label = Product name
product-add-description-label = Description
product-add-price-label = Price
product-add-submit = Add

# =============================================================================
# REVIEWS
# =============================================================================

reviews-title = Reviews
reviews-empty = No reviews yet
reviews-write = Write a review
reviews-average = Average rating

review-rating-label = Rating
review-comment-label = Comment
review-comment-placeholder = Share your experience...
review-submit = Post review
review-success = Review posted!
review-already = You've already reviewed this business

review-helpful = Helpful
review-report = Report

# Stars
rating-1 = Terrible
rating-2 = Poor
rating-3 = Average
rating-4 = Good
rating-5 = Excellent

# =============================================================================
# ORDERS
# =============================================================================

orders-title = My Orders
orders-empty = No orders
orders-filter-all = All
orders-filter-pending = Pending
orders-filter-completed = Completed
orders-filter-cancelled = Cancelled

order-status-pending = Pending
order-status-confirmed = Confirmed
order-status-preparing = Preparing
order-status-ready = Ready
order-status-delivered = Delivered
order-status-completed = Completed
order-status-cancelled = Cancelled

order-total = Total
order-items = { $count ->
    [one] { $count } item
   *[other] { $count } items
}
order-placed = Order placed on { $date }
order-details = Order details
