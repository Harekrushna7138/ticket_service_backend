#!/bin/bash

# Support Ticketing System Setup Script
# This script will help you set up the project quickly

set -e

echo "🚀 Support Ticketing System Setup"
echo "=================================="

# Check if Docker is installed
if command -v docker &> /dev/null; then
    echo "✅ Docker is installed"
else
    echo "❌ Docker is not installed. Please install Docker first:"
    echo "   https://docs.docker.com/get-docker/"
    exit 1
fi

# Check if Docker Compose is installed
if command -v docker-compose &> /dev/null; then
    echo "✅ Docker Compose is installed"
else
    echo "❌ Docker Compose is not installed. Please install Docker Compose first:"
    echo "   https://docs.docker.com/compose/install/"
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp env.example .env
    echo "✅ .env file created"
else
    echo "✅ .env file already exists"
fi

# Build and start the application
echo "🐳 Starting the application with Docker Compose..."
docker-compose up -d --build

echo ""
echo "⏳ Waiting for services to start..."
sleep 10

# Check if services are running
if docker-compose ps | grep -q "Up"; then
    echo "✅ Services are running!"
else
    echo "❌ Services failed to start. Check logs with: docker-compose logs"
    exit 1
fi

echo ""
echo "🎉 Setup complete! Your application is running at:"
echo "   http://127.0.0.1:3000"
echo ""
echo "📋 Useful commands:"
echo "   View logs: docker-compose logs -f"
echo "   Stop services: docker-compose down"
echo "   Restart services: docker-compose restart"
echo ""
echo "🧪 Test the API:"
echo "   curl http://127.0.0.1:3000/health"
echo ""
echo "📚 Check the README.md for more information and API examples" 