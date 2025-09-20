# ☁️ Oracle Cloud Infrastructure Deployment Guide

Deploy your JSON Oracle API to Oracle Cloud Infrastructure (OCI) for enterprise-grade performance and reliability.

## 🎯 Why Oracle Cloud?

### **Advantages over Railway:**
- ✅ **Always Free Tier** - 2 VMs with 1GB RAM each (vs Railway's limited free tier)
- ✅ **Better Performance** - Enterprise-grade infrastructure
- ✅ **GPU Support** - For advanced AI models and Ollama
- ✅ **Global Regions** - Deploy closer to your users
- ✅ **More Control** - Full VPS/container control
- ✅ **Cost Effective** - Pay only for what you use
- ✅ **No Vendor Lock-in** - Standard Docker containers

### **Perfect for AI Workloads:**
- High-performance compute instances
- GPU instances for AI models
- Persistent storage for data
- Load balancing and auto-scaling
- Global CDN capabilities

## 🚀 Quick Deployment Options

### Option 1: One-Click Terraform (Recommended)

```bash
# 1. Install Terraform
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -
sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"
sudo apt-get update && sudo apt-get install terraform

# 2. Configure OCI CLI
oci setup config

# 3. Deploy with Terraform
cd terraform/
terraform init
terraform plan
terraform apply
```

### Option 2: Manual Deployment Script

```bash
# 1. Install OCI CLI
bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"

# 2. Configure OCI
oci setup config

# 3. Deploy
./oci-deploy.sh us-ashburn-1 your-compartment-id
```

### Option 3: Container Registry + Kubernetes

```bash
# 1. Build and push to OCI Container Registry
docker build -t json-oracle .
docker tag json-oracle iad.ocir.io/your-namespace/json-oracle:latest
docker push iad.ocir.io/your-namespace/json-oracle:latest

# 2. Deploy to OKE (Oracle Kubernetes Engine)
kubectl apply -f oci-deploy.yaml
```

## 📋 Prerequisites

### 1. Oracle Cloud Account
- Sign up at [cloud.oracle.com](https://cloud.oracle.com)
- Always Free tier includes:
  - 2 VMs (1GB RAM each)
  - 10GB storage
  - Load balancer
  - Block storage

### 2. Required Tools
```bash
# OCI CLI
bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"

# Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Terraform (optional)
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -
sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"
sudo apt-get update && sudo apt-get install terraform
```

### 3. SSH Key Setup
```bash
# Generate SSH key if you don't have one
ssh-keygen -t rsa -b 4096 -C "your-email@example.com"

# Add to OCI
oci setup keys
```

## 🔧 Configuration

### Environment Variables
```bash
# Set in OCI instance metadata or .env file
export RUST_LOG=info
export PORT=3000
export OLLAMA_BASE_URL=http://localhost:11434
export OLLAMA_MODEL=llama2
export MAX_TIMEOUT_SECONDS=120
```

### OCI Configuration
```bash
# Run OCI setup
oci setup config

# Test configuration
oci iam user get --user-id $(oci iam user list --query 'data[0].id' --raw-output)
```

## 🏗️ Architecture

```
Internet → Load Balancer → API Instance → Ollama Service
                ↓
            Auto Scaling Group
                ↓
            Monitoring & Logging
```

### Components:
- **Load Balancer** - Traffic distribution and SSL termination
- **Compute Instance** - Your JSON Oracle API
- **Ollama Service** - AI model inference
- **Block Storage** - Persistent data storage
- **VCN** - Virtual Cloud Network with security lists

## 📊 Monitoring & Maintenance

### Health Checks
```bash
# API health
curl http://your-domain.com/health

# Ollama health
curl http://your-domain.com:11434/api/tags
```

### Logs
```bash
# Application logs
sudo journalctl -u json-oracle -f

# Ollama logs
sudo journalctl -u ollama -f

# System logs
sudo tail -f /var/log/syslog
```

### Monitoring
- OCI Monitoring service
- Custom health checks
- Load balancer metrics
- Instance metrics

## 🔒 Security Best Practices

### 1. Network Security
- Security lists with minimal required ports
- Private subnets for backend services
- Public subnets only for load balancers

### 2. Instance Security
- SSH key-based authentication
- Fail2ban for intrusion prevention
- Regular security updates
- Firewall configuration

### 3. Application Security
- Environment variable management
- SSL/TLS certificates
- API rate limiting
- Input validation

## 💰 Cost Optimization

### Always Free Tier Usage
- 2 VMs (1GB RAM each) - Perfect for API + Ollama
- 10GB storage - Sufficient for logs and data
- Load balancer - Included in free tier

### Scaling Costs
- **VM.Standard.E2.1.Micro** - Always Free
- **VM.Standard.E2.1** - $0.0065/hour
- **VM.Standard.E3.Flex** - $0.018/hour (better for AI)

### Storage Costs
- Block Storage - $0.0255/GB/month
- Object Storage - $0.0255/GB/month

## 🚀 Advanced Features

### Auto Scaling
```hcl
resource "oci_autoscaling_auto_scaling_configuration" "json_oracle_autoscaling" {
  compartment_id = var.compartment_id
  display_name   = "json-oracle-autoscaling"
  
  policies {
    policy_type = "scheduled"
    capacity {
      initial = 1
      max     = 5
      min     = 1
    }
  }
}
```

### GPU Support
```bash
# Deploy GPU instance for advanced AI models
terraform apply -var="instance_shape=VM.GPU2.1"
```

### Global Deployment
```bash
# Deploy to multiple regions
./oci-deploy.sh us-ashburn-1    # US East
./oci-deploy.sh eu-frankfurt-1  # Europe
./oci-deploy.sh ap-sydney-1     # Asia Pacific
```

## 🔧 Troubleshooting

### Common Issues

**Instance won't start:**
```bash
# Check instance logs
oci compute instance get --instance-id $INSTANCE_ID
```

**API not responding:**
```bash
# Check service status
sudo systemctl status json-oracle
sudo journalctl -u json-oracle -f
```

**Ollama connection issues:**
```bash
# Check Ollama service
sudo systemctl status ollama
ollama list
```

**Load balancer issues:**
```bash
# Check backend health
oci lb backend-health get --load-balancer-id $LB_ID --backend-set-name json-oracle-backend-set
```

## 📈 Performance Tuning

### Instance Optimization
```bash
# Increase file descriptors
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf

# Optimize network settings
echo 'net.core.somaxconn = 65536' >> /etc/sysctl.conf
sysctl -p
```

### Ollama Optimization
```bash
# Increase Ollama timeout
export OLLAMA_TIMEOUT=300

# Use GPU if available
export OLLAMA_GPU_LAYERS=32
```

## 🎉 Success Metrics

After deployment, you should see:
- ✅ API responding at `http://your-domain.com/health`
- ✅ Ollama models available
- ✅ WebSocket connections working
- ✅ Load balancer distributing traffic
- ✅ Monitoring dashboards active

## 📞 Support

- **OCI Documentation**: [docs.oracle.com](https://docs.oracle.com)
- **OCI Community**: [community.oracle.com](https://community.oracle.com)
- **GitHub Issues**: [github.com/KingGekko/json-oracle](https://github.com/KingGekko/json-oracle)

---

**Your JSON Oracle API is now running on enterprise-grade Oracle Cloud Infrastructure!** ☁️🚀
