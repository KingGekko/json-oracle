# AI JSON Analysis API

A high-performance, domain-agnostic API for AI-powered JSON data analysis with real-time streaming capabilities.

## Features

- **Multi-Domain Support**: Finance, Healthcare, E-commerce, Logistics, and more
- **Real-Time Streaming**: WebSocket support for live data updates
- **AI Analysis**: Ollama integration for intelligent data processing
- **Multi-Model Conversations**: Multiple AI models can collaborate on analysis
- **Optimized Performance**: Ultra-fast processing with parallel operations

## API Structure

```
api/
├── mod.rs              # Main module declarations
├── api_server.rs       # Server startup and configuration
├── core_handlers.rs    # Main API endpoints and handlers
├── file_streaming.rs   # Real-time JSON file streaming
├── domains.rs          # Domain types and configurations
└── prompts.rs          # Dynamic prompt generation system
```

## Endpoints

### Core Endpoints
- `GET /health` - Health check
- `POST /api/watch` - Start watching a JSON file
- `GET /api/watch/{file_path}` - Stop watching a file
- `GET /api/files` - List watched files
- `GET /api/content/{file_path}` - Get file content
- `GET /api/stream/{file_path}` - WebSocket stream for real-time updates

### AI Analysis
- `POST /api/ollama/process` - Process JSON file with AI analysis
- `POST /api/ollama/conversation` - Multi-model AI conversation

### Utility
- `GET /api/available-files` - List available JSON files

## Usage Examples

### Basic AI Analysis
```bash
curl -X POST http://localhost:3000/api/ollama/process \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "data.json",
    "prompt": "Analyze this data and provide insights",
    "model": "llama2"
  }'
```

### Multi-Model Conversation
```bash
curl -X POST http://localhost:3000/api/ollama/conversation \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "data.json",
    "initial_prompt": "Analyze this financial data",
    "models": ["llama2", "mistral"],
    "conversation_rounds": 3,
    "conversation_type": "collaboration"
  }'
```

### Real-Time Streaming
```javascript
const ws = new WebSocket('ws://localhost:3000/api/stream/data.json');
ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('File updated:', update);
};
```

## Configuration

The API uses environment variables for configuration:

- `OLLAMA_BASE_URL` - Ollama server URL (default: http://localhost:11434)
- `OLLAMA_MODEL` - Default AI model (default: llama2)
- `MAX_TIMEOUT_SECONDS` - Request timeout (default: 120)

## Supported Domains

- **Finance**: Portfolio analysis, risk assessment, trading recommendations
- **Healthcare**: Patient data analysis, anomaly detection, clinical insights
- **E-commerce**: Sales optimization, customer behavior, inventory management
- **Logistics**: Route optimization, supply chain analysis, performance metrics
- **Generic**: Universal data analysis for any domain

## Performance

- **Ultra-Fast Processing**: Parallel file reading and AI processing
- **Real-Time Updates**: WebSocket streaming with minimal latency
- **Optimized Threading**: Maximum performance with concurrent operations
- **Configurable Timeouts**: Adjustable processing limits

## Getting Started

1. Start the Ollama server
2. Set environment variables
3. Run the API server
4. Send requests to analyze your JSON data

The API automatically detects file changes and streams updates in real-time, making it perfect for monitoring systems, dashboards, and automated analysis workflows.
