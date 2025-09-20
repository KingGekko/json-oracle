# 🚀 Bolt UI + JSON Oracle API Integration Guide

Complete integration guide for connecting your Bolt UI frontend with the JSON Oracle API backend.

## 🎯 Overview

This integration connects:
- **Frontend**: Bolt UI (React + TypeScript + Tailwind CSS)
- **Backend**: JSON Oracle API (Rust + Axum + Ollama)
- **Real-time**: WebSocket streaming for live analysis updates
- **AI Models**: Ollama integration for multi-model analysis

## 📁 Project Structure

```
json-oracle/
├── src/                    # Rust API backend
│   ├── api/
│   │   ├── core_handlers.rs
│   │   ├── api_server.rs
│   │   ├── file_streaming.rs
│   │   └── domains.rs
│   └── main.rs
├── bolt_ui/                # React frontend
│   ├── src/
│   │   ├── components/
│   │   ├── services/
│   │   ├── hooks/
│   │   └── types/
│   └── package.json
└── docker-compose.yml      # Full stack deployment
```

## 🔧 Backend API Endpoints

Your JSON Oracle API provides these endpoints:

### Core Endpoints
- `GET /health` - Health check
- `GET /api/domains` - Available analysis domains
- `GET /api/models` - Available AI models
- `POST /api/ollama/process` - Analyze JSON data
- `GET /api/conversation` - Multi-model conversations
- `WebSocket /ws` - Real-time streaming

### Analysis Flow
1. **Upload JSON** → Frontend sends data to `/api/ollama/process`
2. **Process Data** → Backend analyzes with Ollama
3. **Stream Results** → WebSocket sends real-time updates
4. **Display Results** → Frontend renders insights and metrics

## 🎨 Frontend Integration

### API Service (`src/services/api.ts`)
```typescript
// Configured for your JSON Oracle API
const API_BASE = 'http://localhost:3000/api';

// Supported domains
const domains = [
  { id: 'finance', name: 'Finance', description: 'Financial analysis' },
  { id: 'healthcare', name: 'Healthcare', description: 'Medical insights' },
  { id: 'ecommerce', name: 'E-commerce', description: 'Sales optimization' },
  // ... more domains
];

// AI Models from your Ollama setup
const models = [
  { id: 'llama2', name: 'Llama 2', status: 'online' },
  { id: 'mistral', name: 'Mistral', status: 'online' },
  // ... more models
];
```

### WebSocket Service (`src/services/websocket.ts`)
```typescript
// Real-time connection to your API
const wsUrl = 'ws://localhost:3000/ws';

// Handles streaming analysis updates
- analysis_progress: Real-time progress updates
- analysis_insight: New insights as they're generated
- analysis_complete: Final results
- analysis_error: Error handling
```

## 🚀 Quick Start

### 1. Start the Backend (JSON Oracle API)
```bash
# From the root directory
cd /path/to/json-oracle

# Install dependencies
cargo build

# Start with Ollama
docker-compose up -d

# Or start directly
cargo run
```

### 2. Start the Frontend (Bolt UI)
```bash
# From the bolt_ui directory
cd bolt_ui

# Install dependencies
npm install

# Start development server
npm run dev
```

### 3. Access the Application
- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:3000
- **WebSocket**: ws://localhost:3000/ws

## 🔄 Data Flow

### 1. Upload & Configure
```typescript
// User uploads JSON data
const jsonData = { /* user's data */ };

// Select domain and models
const domain = 'finance';
const models = ['llama2', 'mistral'];

// Start analysis
const result = await apiService.analyzeJSON(jsonData, domain, models);
```

### 2. Real-time Processing
```typescript
// WebSocket receives streaming updates
wsService.subscribe('progress', (data) => {
  setProgress(data.progress);
  setCurrentStep(data.step);
});

wsService.subscribe('insight', (data) => {
  setInsights(prev => [...prev, data.insight]);
});
```

### 3. Display Results
```typescript
// Final analysis results
const analysisResult = {
  insights: [...],
  metrics: { processingTime: 2.3, accuracy: 0.94 },
  recommendations: [...]
};
```

## 🎛️ UI Components

### Dashboard
- **Stats Cards**: Total analyses, active models, processing speed
- **Activity Feed**: Recent analysis history
- **Quick Actions**: Upload data, configure models

### Upload & Analysis
- **JSON Uploader**: Drag & drop JSON files
- **Domain Selector**: Choose analysis domain
- **Model Selector**: Select AI models
- **Analysis Button**: Start processing

### Real-time Results
- **Progress Bar**: Live analysis progress
- **Streaming Insights**: Real-time insight updates
- **Final Results**: Complete analysis with metrics
- **Export Options**: JSON, CSV, PDF export

## 🔧 Configuration

### Environment Variables
```bash
# Backend (.env)
RUST_LOG=info
PORT=3000
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2

# Frontend (bolt_ui/.env)
VITE_API_URL=http://localhost:3000/api
VITE_WS_URL=ws://localhost:3000/ws
```

### CORS Configuration
Your Rust API should allow CORS for the frontend:
```rust
// In api_server.rs
let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE, AUTHORIZATION]);
```

## 🚀 Deployment

### Development
```bash
# Terminal 1: Start backend
cargo run

# Terminal 2: Start frontend
cd bolt_ui && npm run dev
```

### Production (Docker)
```bash
# Build and deploy everything
docker-compose up -d

# Access application
# Frontend: http://localhost:3000
# Backend: http://localhost:8080
```

### Oracle Cloud Deployment
```bash
# Deploy to Oracle Cloud
./oci-deploy.sh

# Update frontend API URL
VITE_API_URL=https://your-oracle-api.com/api
```

## 🧪 Testing the Integration

### 1. Health Check
```bash
curl http://localhost:3000/health
```

### 2. Upload Test Data
```json
{
  "sales_data": [
    {"month": "January", "revenue": 10000, "customers": 150},
    {"month": "February", "revenue": 12000, "customers": 180},
    {"month": "March", "revenue": 15000, "customers": 220}
  ]
}
```

### 3. Start Analysis
- Select domain: "ecommerce"
- Select models: "llama2"
- Click "Start AI Analysis"
- Watch real-time progress

### 4. Verify Results
- Check insights generation
- Verify metrics calculation
- Test export functionality

## 🔍 Troubleshooting

### Common Issues

**Backend not responding:**
```bash
# Check if Rust API is running
curl http://localhost:3000/health

# Check Ollama status
curl http://localhost:11434/api/tags
```

**Frontend connection failed:**
```bash
# Check API URL in browser console
# Verify CORS configuration
# Check WebSocket connection
```

**Analysis not starting:**
```bash
# Check Ollama models
ollama list

# Check API logs
tail -f logs/api.log
```

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Frontend debug
npm run dev -- --debug
```

## 📊 Performance Optimization

### Backend
- Enable async processing
- Use connection pooling
- Implement caching
- Optimize Ollama model loading

### Frontend
- Implement virtual scrolling for large results
- Use React.memo for components
- Optimize WebSocket message handling
- Add loading states and error boundaries

## 🔒 Security Considerations

### API Security
- Implement authentication
- Add rate limiting
- Validate input data
- Secure WebSocket connections

### Frontend Security
- Sanitize user input
- Implement CSRF protection
- Use HTTPS in production
- Validate API responses

## 🎉 Success Metrics

After integration, you should see:
- ✅ Frontend connects to backend API
- ✅ JSON upload and analysis works
- ✅ Real-time WebSocket streaming
- ✅ AI insights generation
- ✅ Export functionality
- ✅ Responsive UI with loading states

## 📞 Support

- **Backend Issues**: Check Rust API logs
- **Frontend Issues**: Check browser console
- **Integration Issues**: Verify API endpoints
- **Performance Issues**: Monitor Ollama status

---

**Your Bolt UI is now fully integrated with the JSON Oracle API!** 🚀✨

The combination provides:
- **Beautiful UI** for data upload and visualization
- **Powerful AI Analysis** with multiple models
- **Real-time Updates** via WebSocket streaming
- **Enterprise Features** for production deployment
