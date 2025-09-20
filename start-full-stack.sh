#!/bin/bash

# JSON Oracle API + Bolt UI Full Stack Startup Script
# This script starts both the backend API and frontend UI

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}🚀 Starting JSON Oracle API + Bolt UI Full Stack${NC}"
echo "=================================================="

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}❌ Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

# Check if Ollama is installed
if ! command -v ollama &> /dev/null; then
    echo -e "${YELLOW}⚠️  Ollama not found. Installing Ollama...${NC}"
    curl -fsSL https://ollama.com/install.sh | sh
fi

# Function to check if a service is running
check_service() {
    local service_name=$1
    local port=$2
    local max_attempts=30
    local attempt=1
    
    echo -e "${BLUE}🔍 Checking if $service_name is running on port $port...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "http://localhost:$port" > /dev/null 2>&1; then
            echo -e "${GREEN}✅ $service_name is running on port $port${NC}"
            return 0
        fi
        
        echo -e "${YELLOW}⏳ Waiting for $service_name... (attempt $attempt/$max_attempts)${NC}"
        sleep 2
        ((attempt++))
    done
    
    echo -e "${RED}❌ $service_name failed to start on port $port${NC}"
    return 1
}

# Start Ollama service
echo -e "${BLUE}🤖 Starting Ollama service...${NC}"
ollama serve &
OLLAMA_PID=$!

# Wait for Ollama to start
sleep 5

# Pull required models
echo -e "${BLUE}📥 Pulling AI models...${NC}"
ollama pull llama2 &
ollama pull mistral &
ollama pull codellama &

# Wait for models to download
echo -e "${YELLOW}⏳ Downloading AI models (this may take a while)...${NC}"
wait

echo -e "${GREEN}✅ AI models ready${NC}"

# Start the backend API
echo -e "${BLUE}🔧 Starting JSON Oracle API backend...${NC}"
cargo run &
API_PID=$!

# Wait for API to start
if check_service "JSON Oracle API" 3000; then
    echo -e "${GREEN}✅ Backend API is running${NC}"
else
    echo -e "${RED}❌ Backend API failed to start${NC}"
    kill $OLLAMA_PID 2>/dev/null || true
    exit 1
fi

# Start the frontend
echo -e "${BLUE}🎨 Starting Bolt UI frontend...${NC}"
cd bolt_ui
npm install
npm run dev &
FRONTEND_PID=$!
cd ..

# Wait for frontend to start
if check_service "Bolt UI Frontend" 5173; then
    echo -e "${GREEN}✅ Frontend UI is running${NC}"
else
    echo -e "${RED}❌ Frontend UI failed to start${NC}"
    kill $OLLAMA_PID $API_PID 2>/dev/null || true
    exit 1
fi

# Display success message
echo ""
echo -e "${GREEN}🎉 Full Stack Application Started Successfully!${NC}"
echo "=================================================="
echo -e "${BLUE}📊 Backend API:${NC} http://localhost:3000"
echo -e "${BLUE}🎨 Frontend UI:${NC} http://localhost:5173"
echo -e "${BLUE}🤖 Ollama API:${NC} http://localhost:11434"
echo -e "${BLUE}🔌 WebSocket:${NC} ws://localhost:3000/ws"
echo ""
echo -e "${YELLOW}📋 Available Endpoints:${NC}"
echo "  • GET  /health - Health check"
echo "  • GET  /api/domains - Available domains"
echo "  • GET  /api/models - Available AI models"
echo "  • POST /api/ollama/process - Analyze JSON data"
echo "  • GET  /api/conversation - Multi-model conversations"
echo ""
echo -e "${PURPLE}🚀 Ready to analyze JSON data with AI!${NC}"
echo ""

# Function to cleanup on exit
cleanup() {
    echo -e "${YELLOW}🛑 Shutting down services...${NC}"
    kill $OLLAMA_PID $API_PID $FRONTEND_PID 2>/dev/null || true
    echo -e "${GREEN}✅ All services stopped${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Keep script running
echo -e "${BLUE}💡 Press Ctrl+C to stop all services${NC}"
wait
