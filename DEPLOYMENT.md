# 🚀 Deployment Guide

This guide covers deploying your AI JSON Analysis API to various platforms.

## 📋 Prerequisites

- Rust 1.75+ installed
- Docker installed (for containerized deployments)
- Ollama server running (for AI analysis)

## 🌐 Platform-Specific Deployment

### 1. Railway (Recommended)

**Best for:** Persistent connections, WebSocket support, easy deployment

```bash
# Install Railway CLI
npm install -g @railway/cli

# Login to Railway
railway login

# Deploy
railway deploy
```

**Features:**
- ✅ WebSocket support
- ✅ Persistent file watching
- ✅ Automatic scaling
- ✅ Built-in monitoring

### 2. Fly.io

**Best for:** Global deployment, edge computing

```bash
# Install Fly CLI
curl -L https://fly.io/install.sh | sh

# Deploy
fly deploy
```

**Features:**
- ✅ Global edge deployment
- ✅ WebSocket support
- ✅ Auto-scaling
- ✅ Health checks

### 3. DigitalOcean App Platform

**Best for:** Simple deployment, managed infrastructure

1. Connect your GitHub repository
2. Select "Docker" as build type
3. Use the provided `Dockerfile`
4. Set environment variables

**Features:**
- ✅ Managed infrastructure
- ✅ Auto-scaling
- ✅ Load balancing
- ✅ SSL certificates

### 4. Heroku

**Best for:** Quick deployment, familiar platform

```bash
# Install Heroku CLI
# Create app
heroku create your-app-name

# Set environment variables
heroku config:set RUST_LOG=info
heroku config:set PORT=8080

# Deploy
git push heroku main
```

**Features:**
- ✅ Easy deployment
- ✅ Add-ons ecosystem
- ✅ Monitoring tools
- ❌ WebSocket limitations

### 5. Vercel (Serverless)

**Best for:** Serverless functions, edge computing

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
vercel --prod
```

**Limitations:**
- ❌ No WebSocket support
- ❌ No persistent file watching
- ❌ 30-second function timeout
- ✅ Great for simple API calls

## 🐳 Docker Deployment

### Local Development

```bash
# Build and run with Docker Compose
docker-compose up --build

# Access API at http://localhost:3000
# Access Ollama at http://localhost:11434
```

### Production Deployment

```bash
# Build production image
docker build -t ai-json-analysis-api .

# Run with environment variables
docker run -p 3000:3000 \
  -e RUST_LOG=info \
  -e OLLAMA_BASE_URL=http://your-ollama-server:11434 \
  ai-json-analysis-api
```

## ⚙️ Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level |
| `PORT` | `3000` | Server port |
| `OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama server URL |
| `OLLAMA_MODEL` | `llama2` | Default AI model |
| `MAX_TIMEOUT_SECONDS` | `120` | Request timeout |

## 🔧 Platform-Specific Configuration

### Railway
- Uses `railway.toml` for configuration
- Automatic environment detection
- Built-in health checks

### Fly.io
- Uses `fly.toml` for configuration
- Configure regions and scaling
- Health check endpoints

### Vercel
- Uses `vercel.json` for configuration
- Serverless function limits
- Edge network deployment

## 📊 Monitoring & Health Checks

All deployments include health check endpoints:

```bash
# Check API health
curl https://your-app.com/health

# Response:
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:00:00Z",
  "service": "ai-json-analysis-api"
}
```

## 🔒 Security Considerations

1. **Environment Variables**: Never commit sensitive data
2. **API Keys**: Use platform-specific secret management
3. **CORS**: Configure for your frontend domain
4. **Rate Limiting**: Implement for production use
5. **HTTPS**: All platforms provide SSL certificates

## 🚀 Quick Start Commands

### Railway (Fastest)
```bash
railway login && railway deploy
```

### Fly.io (Global)
```bash
fly launch && fly deploy
```

### Docker (Local)
```bash
docker-compose up --build
```

### Vercel (Serverless)
```bash
vercel --prod
```

## 💡 Tips for Production

1. **Start with Railway** - Best overall experience
2. **Use Docker** - Consistent across platforms
3. **Monitor logs** - Set up proper logging
4. **Scale gradually** - Start small, scale as needed
5. **Backup data** - Implement data persistence
6. **Health checks** - Monitor API availability

## 🆘 Troubleshooting

### Common Issues

**WebSocket not working:**
- Check platform WebSocket support
- Use Railway or Fly.io instead of Vercel

**File watching not working:**
- Use persistent storage
- Implement polling as fallback

**Ollama connection issues:**
- Check Ollama server availability
- Verify network connectivity
- Use external Ollama service

**Memory issues:**
- Increase container memory limits
- Optimize Rust binary size
- Use release builds
