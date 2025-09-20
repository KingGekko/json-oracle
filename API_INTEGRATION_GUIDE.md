# 🔗 JSON Oracle API Integration Guide

Complete guide for integrating your JSON Oracle API into external systems and monitoring results on the dashboard.

## 🎯 Overview

Your JSON Oracle API provides comprehensive integration capabilities that allow users to:

1. **Connect External Systems** - Webhooks, REST APIs, databases, file systems
2. **Send Data for Analysis** - Automatic or manual data processing
3. **Monitor Results** - Real-time dashboard with insights and metrics
4. **Receive Notifications** - Webhooks, email, and dashboard alerts

## 🚀 Integration Flow

```
External System → JSON Oracle API → AI Analysis → Dashboard Display
     ↓                    ↓              ↓            ↓
Send Data          Process with      Generate      View Results
                   Ollama AI        Insights       & Metrics
```

## 📋 Integration Types

### 1. **Webhook Integration**
```http
POST https://your-json-oracle-api.com/api/analyze
Content-Type: application/json
X-API-Key: json_oracle_your_api_key

{
  "integration_id": "your_integration_id",
  "api_key": "json_oracle_your_api_key",
  "data": {
    "sales_data": [...],
    "customer_metrics": {...}
  },
  "domain": "ecommerce",
  "model": "llama2",
  "callback_url": "https://your-system.com/callback"
}
```

### 2. **REST API Integration**
```javascript
// Send data for analysis
const response = await fetch('https://your-json-oracle-api.com/api/analyze', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'X-API-Key': 'json_oracle_your_api_key'
  },
  body: JSON.stringify({
    integration_id: 'your_integration_id',
    api_key: 'json_oracle_your_api_key',
    data: yourJsonData,
    domain: 'finance',
    model: 'mistral'
  })
});

const result = await response.json();
console.log('Analysis result:', result);
```

### 3. **Database Integration**
```python
import requests
import json

# Send database query results for analysis
def analyze_database_data(query_results):
    payload = {
        "integration_id": "your_integration_id",
        "api_key": "json_oracle_your_api_key",
        "data": query_results,
        "domain": "logistics",
        "model": "codellama"
    }
    
    response = requests.post(
        'https://your-json-oracle-api.com/api/analyze',
        headers={'Content-Type': 'application/json'},
        data=json.dumps(payload)
    )
    
    return response.json()
```

### 4. **File System Integration**
```bash
# Monitor file changes and send for analysis
#!/bin/bash

# Watch for JSON files in directory
inotifywait -m -e modify /path/to/json/files |
while read path action file; do
    if [[ $file == *.json ]]; then
        echo "Processing $file"
        curl -X POST https://your-json-oracle-api.com/api/analyze \
             -H "Content-Type: application/json" \
             -H "X-API-Key: json_oracle_your_api_key" \
             -d "{
                 \"integration_id\": \"your_integration_id\",
                 \"api_key\": \"json_oracle_your_api_key\",
                 \"data\": $(cat \"$path$file\"),
                 \"domain\": \"manufacturing\",
                 \"model\": \"llama2\"
             }"
    fi
done
```

## 🔧 API Endpoints

### **Create Integration**
```http
POST /api/integrations
Content-Type: application/json

{
  "name": "My E-commerce System",
  "system_type": "webhook",
  "webhook_url": "https://my-system.com/webhook",
  "configuration": {
    "auto_analyze": true,
    "analysis_domain": "ecommerce",
    "ai_model": "llama2",
    "notification_settings": {
      "email_notifications": false,
      "webhook_notifications": true,
      "dashboard_alerts": true,
      "real_time_updates": true
    }
  }
}
```

**Response:**
```json
{
  "id": "integration_uuid",
  "name": "My E-commerce System",
  "system_type": "webhook",
  "api_key": "json_oracle_abc123...",
  "webhook_url": "https://my-system.com/webhook",
  "status": "active",
  "created_at": "2024-01-15T10:30:00Z",
  "configuration": { ... }
}
```

### **Send Data for Analysis**
```http
POST /api/analyze
Content-Type: application/json
X-API-Key: json_oracle_your_api_key

{
  "integration_id": "integration_uuid",
  "api_key": "json_oracle_your_api_key",
  "data": { ... },
  "domain": "ecommerce",
  "model": "llama2",
  "callback_url": "https://your-system.com/callback"
}
```

**Response:**
```json
{
  "id": "analysis_uuid",
  "integration_id": "integration_uuid",
  "system_name": "My E-commerce System",
  "data_source": "external_system",
  "analysis_result": {
    "summary": "Analysis of e-commerce data...",
    "insights": [
      {
        "type": "pattern",
        "title": "Sales Pattern Detected",
        "description": "Peak sales occur on weekends",
        "confidence": 0.92
      }
    ],
    "recommendations": [
      "Increase weekend inventory",
      "Optimize marketing campaigns for weekends"
    ],
    "metrics": {
      "data_points": 1000,
      "analysis_confidence": 0.85,
      "processing_timestamp": "2024-01-15T10:30:00Z"
    }
  },
  "status": "completed",
  "created_at": "2024-01-15T10:30:00Z",
  "processing_time": 2.3,
  "insights_count": 3,
  "recommendations_count": 2
}
```

### **Get Analysis Results**
```http
GET /api/integrations/{integration_id}/results?limit=10
Authorization: Bearer your_api_key
```

### **Get Dashboard Statistics**
```http
GET /api/integrations/stats
Authorization: Bearer your_api_key
```

## 📊 Dashboard Monitoring

### **Integration Dashboard Features**
- ✅ **Real-time Stats**: Total integrations, active systems, success rates
- ✅ **Recent Activity**: Latest analysis results and processing times
- ✅ **System Status**: Health monitoring for all connected systems
- ✅ **Performance Metrics**: Processing times, accuracy, and throughput

### **Analysis Results Display**
- ✅ **Live Results**: Real-time streaming of analysis results
- ✅ **Interactive Visualizations**: Charts, graphs, and performance metrics
- ✅ **Insight Breakdown**: Pattern detection, anomalies, trends, predictions
- ✅ **Recommendation Tracking**: Actionable insights with impact assessment

## 🔔 Notification System

### **Webhook Notifications**
When analysis completes, your system receives:
```json
{
  "event": "analysis_completed",
  "integration_id": "integration_uuid",
  "analysis_id": "analysis_uuid",
  "status": "completed",
  "result": { ... },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### **Email Notifications**
- Analysis completion alerts
- Error notifications
- Performance reports
- System health updates

### **Dashboard Alerts**
- Real-time status updates
- Performance warnings
- Integration health monitoring
- Success/failure notifications

## 🛠️ Implementation Examples

### **Node.js Integration**
```javascript
const axios = require('axios');

class JsonOracleIntegration {
  constructor(apiKey, integrationId) {
    this.apiKey = apiKey;
    this.integrationId = integrationId;
    this.baseUrl = 'https://your-json-oracle-api.com/api';
  }

  async analyzeData(data, domain = 'generic', model = 'llama2') {
    try {
      const response = await axios.post(`${this.baseUrl}/analyze`, {
        integration_id: this.integrationId,
        api_key: this.apiKey,
        data,
        domain,
        model
      });

      return response.data;
    } catch (error) {
      console.error('Analysis failed:', error.response?.data || error.message);
      throw error;
    }
  }

  async getResults(limit = 10) {
    const response = await axios.get(
      `${this.baseUrl}/integrations/${this.integrationId}/results?limit=${limit}`,
      { headers: { 'X-API-Key': this.apiKey } }
    );
    
    return response.data;
  }
}

// Usage
const integration = new JsonOracleIntegration('your_api_key', 'your_integration_id');
const result = await integration.analyzeData({ sales: [100, 200, 150] }, 'ecommerce');
```

### **Python Integration**
```python
import requests
import json
from datetime import datetime

class JsonOracleIntegration:
    def __init__(self, api_key, integration_id):
        self.api_key = api_key
        self.integration_id = integration_id
        self.base_url = 'https://your-json-oracle-api.com/api'
        self.headers = {
            'Content-Type': 'application/json',
            'X-API-Key': api_key
        }

    def analyze_data(self, data, domain='generic', model='llama2'):
        payload = {
            'integration_id': self.integration_id,
            'api_key': self.api_key,
            'data': data,
            'domain': domain,
            'model': model
        }
        
        response = requests.post(
            f'{self.base_url}/analyze',
            headers=self.headers,
            json=payload
        )
        
        response.raise_for_status()
        return response.json()

    def get_results(self, limit=10):
        response = requests.get(
            f'{self.base_url}/integrations/{self.integration_id}/results',
            params={'limit': limit},
            headers=self.headers
        )
        
        response.raise_for_status()
        return response.json()

# Usage
integration = JsonOracleIntegration('your_api_key', 'your_integration_id')
result = integration.analyze_data({'sales': [100, 200, 150]}, 'ecommerce')
```

### **PHP Integration**
```php
<?php

class JsonOracleIntegration {
    private $apiKey;
    private $integrationId;
    private $baseUrl;
    
    public function __construct($apiKey, $integrationId) {
        $this->apiKey = $apiKey;
        $this->integrationId = $integrationId;
        $this->baseUrl = 'https://your-json-oracle-api.com/api';
    }
    
    public function analyzeData($data, $domain = 'generic', $model = 'llama2') {
        $payload = [
            'integration_id' => $this->integrationId,
            'api_key' => $this->apiKey,
            'data' => $data,
            'domain' => $domain,
            'model' => $model
        ];
        
        $response = $this->makeRequest('POST', '/analyze', $payload);
        return json_decode($response, true);
    }
    
    public function getResults($limit = 10) {
        $response = $this->makeRequest('GET', "/integrations/{$this->integrationId}/results?limit={$limit}");
        return json_decode($response, true);
    }
    
    private function makeRequest($method, $endpoint, $data = null) {
        $url = $this->baseUrl . $endpoint;
        
        $options = [
            'http' => [
                'method' => $method,
                'header' => [
                    'Content-Type: application/json',
                    'X-API-Key: ' . $this->apiKey
                ],
                'content' => $data ? json_encode($data) : null
            ]
        ];
        
        $context = stream_context_create($options);
        $result = file_get_contents($url, false, $context);
        
        if ($result === false) {
            throw new Exception('API request failed');
        }
        
        return $result;
    }
}

// Usage
$integration = new JsonOracleIntegration('your_api_key', 'your_integration_id');
$result = $integration->analyzeData(['sales' => [100, 200, 150]], 'ecommerce');
?>
```

## 🔒 Security & Authentication

### **API Key Management**
- ✅ **Unique Keys**: Each integration gets a unique API key
- ✅ **Key Rotation**: Regular key rotation for security
- ✅ **Access Control**: Role-based access to integrations
- ✅ **Rate Limiting**: Request rate limits per integration

### **Data Security**
- ✅ **Encryption**: All data encrypted in transit and at rest
- ✅ **Data Retention**: Configurable data retention policies
- ✅ **Access Logs**: Comprehensive audit logging
- ✅ **Privacy Controls**: GDPR and privacy compliance

## 📈 Performance & Scaling

### **High Performance**
- ✅ **Concurrent Processing**: Multiple analyses in parallel
- ✅ **Auto-scaling**: Automatic scaling based on demand
- ✅ **Caching**: Intelligent caching for faster responses
- ✅ **Load Balancing**: Distributed processing across nodes

### **Monitoring & Metrics**
- ✅ **Real-time Metrics**: Processing times, success rates, throughput
- ✅ **Health Monitoring**: System health and availability
- ✅ **Performance Alerts**: Automatic alerts for performance issues
- ✅ **Usage Analytics**: Detailed usage and performance analytics

## 🎯 Use Cases

### **E-commerce Platform**
```javascript
// Analyze sales data and customer behavior
const salesData = {
  daily_sales: [1000, 1200, 800, 1500, 2000],
  customer_segments: ['premium', 'standard', 'budget'],
  product_categories: ['electronics', 'clothing', 'books']
};

const analysis = await integration.analyzeData(salesData, 'ecommerce', 'llama2');
// Results: Sales patterns, customer insights, product recommendations
```

### **Healthcare System**
```python
# Analyze patient data and medical records
patient_data = {
    'vital_signs': {'heart_rate': 72, 'blood_pressure': '120/80'},
    'symptoms': ['fatigue', 'headache'],
    'medications': ['aspirin', 'vitamin_d']
}

analysis = integration.analyze_data(patient_data, 'healthcare', 'mistral')
# Results: Health insights, treatment recommendations, risk assessments
```

### **Financial Trading System**
```python
# Analyze market data and trading patterns
market_data = {
    'stock_prices': [100, 102, 98, 105, 108],
    'volume': [1000000, 1200000, 800000, 1500000],
    'indicators': {'rsi': 65, 'macd': 0.5}
}

analysis = integration.analyze_data(market_data, 'finance', 'codellama')
# Results: Trading signals, market trends, risk analysis
```

---

**Your JSON Oracle API now provides comprehensive integration capabilities!** 🚀

Users can connect their systems, send data for AI analysis, and monitor results in real-time through your beautiful dashboard. The system supports multiple integration types, provides detailed analytics, and offers robust security and performance features.
