#!/bin/bash

BASE_URL="http://localhost:9001"

echo "🚀 Creating demo data for Community Manager..."
echo ""

# Create users
echo "👥 Creating users..."
USER1=$(curl -s -X POST $BASE_URL/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"pass123"}' | jq -r '.data.id')

USER2=$(curl -s -X POST $BASE_URL/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"bob","email":"bob@example.com","password":"pass123"}' | jq -r '.data.id')

USER3=$(curl -s -X POST $BASE_URL/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"charlie","email":"charlie@example.com","password":"pass123"}' | jq -r '.data.id')

echo "  ✅ Created: alice ($USER1)"
echo "  ✅ Created: bob ($USER2)"
echo "  ✅ Created: charlie ($USER3)"
echo ""

# Create communities
echo "🏘️  Creating communities..."
COMM1=$(curl -s -X POST $BASE_URL/api/communities \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Tech Enthusiasts\",\"description\":\"A community for technology lovers and developers\",\"created_by\":\"$USER1\"}" | jq -r '.data.id')

COMM2=$(curl -s -X POST $BASE_URL/api/communities \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Gaming Community\",\"description\":\"For gamers of all kinds - PC, console, mobile\",\"created_by\":\"$USER2\"}" | jq -r '.data.id')

COMM3=$(curl -s -X POST $BASE_URL/api/communities \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Local Food Lovers\",\"description\":\"Discover and share the best local restaurants\",\"created_by\":\"$USER3\"}" | jq -r '.data.id')

echo "  ✅ Created: Tech Enthusiasts ($COMM1)"
echo "  ✅ Created: Gaming Community ($COMM2)"
echo "  ✅ Created: Local Food Lovers ($COMM3)"
echo ""

# Create posts
echo "📝 Creating posts..."

# Tech community posts
curl -s -X POST $BASE_URL/api/communities/$COMM1/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Welcome to Tech Enthusiasts!\",\"content\":\"This is a place for all tech lovers. Share your projects, ask questions, and learn together!\",\"author_id\":\"$USER1\"}" > /dev/null

curl -s -X POST $BASE_URL/api/communities/$COMM1/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"What are you working on?\",\"content\":\"Share your current projects! I'm building a community platform with Rust and HTMX.\",\"author_id\":\"$USER2\"}" > /dev/null

# Gaming community posts
curl -s -X POST $BASE_URL/api/communities/$COMM2/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"First Post - Let's Game!\",\"content\":\"Welcome gamers! What are you playing this week?\",\"author_id\":\"$USER2\"}" > /dev/null

curl -s -X POST $BASE_URL/api/communities/$COMM2/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Looking for teammates\",\"content\":\"Anyone want to team up for some co-op games this weekend?\",\"author_id\":\"$USER3\"}" > /dev/null

# Food community posts
curl -s -X POST $BASE_URL/api/communities/$COMM3/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Best Pizza in Town\",\"content\":\"Just discovered an amazing pizzeria on Main Street. The margherita is incredible!\",\"author_id\":\"$USER3\"}" > /dev/null

curl -s -X POST $BASE_URL/api/communities/$COMM3/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Restaurant Recommendations?\",\"content\":\"Looking for good sushi places. Any suggestions?\",\"author_id\":\"$USER1\"}" > /dev/null

echo "  ✅ Created 6 posts across communities"
echo ""

echo "🎉 Demo data created successfully!"
echo ""
echo "📊 View your data at:"
echo "  🌐 Communities page: $BASE_URL/communities"
echo "  🔍 Database test: $BASE_URL/test-db"
echo "  📡 API Users: $BASE_URL/api/users"
echo "  📡 API Communities: $BASE_URL/api/communities"
echo ""
echo "💡 Try these commands:"
echo "  curl $BASE_URL/api/users | jq"
echo "  curl $BASE_URL/api/communities | jq"
echo "  curl $BASE_URL/api/communities/$COMM1/posts | jq"
