# ⚡ Quick Start Guide

Get your AI JSON Analysis API deployed in minutes!

## 🚀 Fastest Deployment (Railway)

```bash
# 1. Install Railway CLI
npm install -g @railway/cli

# 2. Login and deploy
railway login
railway deploy

# 3. Set environment variables
railway variables set OLLAMA_BASE_URL=http://your-ollama-server:11434
railway variables set OLLAMA_MODEL=llama2
```

## 🐳 Local Development

```bash
# 1. Start Ollama server
ollama serve

# 2. Pull a model
ollama pull llama2

# 3. Run the API
docker-compose up --build

# 4. Test the API
curl http://localhost:3000/health
```

## 🌐 Platform-Specific Quick Deploy

### Railway (Recommended)
```bash
./deploy.sh railway
```

### Fly.io
```bash
./deploy.sh fly
```

### Docker (Local)
```bash
./deploy.sh docker
```

### Vercel (Serverless)
```bash
./deploy.sh vercel
```

## 📝 Test Your Deployment

```bash
# Health check
curl https://your-app.com/health

# Analyze JSON data
curl -X POST https://your-app.com/api/ollama/process \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "data.json",
    "prompt": "Analyze this data",
    "model": "llama2"
  }'
```

## 🔧 Environment Setup

1. Copy `env.example` to `.env`
2. Update Ollama server URL
3. Choose your AI model
4. Set log level

## 📊 What You Get

- ✅ REST API with AI analysis
- ✅ Real-time file streaming
- ✅ Multi-model conversations
- ✅ Health monitoring
- ✅ Auto-scaling
- ✅ Global deployment

## 🆘 Need Help?

- Check `DEPLOYMENT.md` for detailed instructions
- Review platform-specific documentation
- Test locally with Docker first
- Monitor logs for issues

**Ready to deploy? Pick your platform and run the deploy script!** 🚀
