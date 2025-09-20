#!/bin/bash

# AI JSON Analysis API Deployment Script
# Usage: ./deploy.sh [platform]

set -e

PLATFORM=${1:-railway}

echo "🚀 Deploying AI JSON Analysis API to $PLATFORM..."

case $PLATFORM in
    "railway")
        echo "📡 Deploying to Railway..."
        if ! command -v railway &> /dev/null; then
            echo "Installing Railway CLI..."
            npm install -g @railway/cli
        fi
        railway login
        railway deploy
        ;;
    
    "fly")
        echo "🪰 Deploying to Fly.io..."
        if ! command -v fly &> /dev/null; then
            echo "Installing Fly CLI..."
            curl -L https://fly.io/install.sh | sh
        fi
        fly deploy
        ;;
    
    "docker")
        echo "🐳 Building and running with Docker..."
        docker-compose up --build -d
        echo "✅ API running at http://localhost:3000"
        ;;
    
    "vercel")
        echo "▲ Deploying to Vercel..."
        if ! command -v vercel &> /dev/null; then
            echo "Installing Vercel CLI..."
            npm install -g vercel
        fi
        vercel --prod
        ;;
    
    "heroku")
        echo "🟣 Deploying to Heroku..."
        if ! command -v heroku &> /dev/null; then
            echo "Please install Heroku CLI first"
            exit 1
        fi
        heroku create
        git push heroku main
        ;;
    
    *)
        echo "❌ Unknown platform: $PLATFORM"
        echo "Available platforms: railway, fly, docker, vercel, heroku"
        exit 1
        ;;
esac

echo "✅ Deployment to $PLATFORM completed!"
echo "📋 Next steps:"
echo "   1. Set environment variables"
echo "   2. Configure Ollama server"
echo "   3. Test the API endpoints"
echo "   4. Monitor logs and performance"
