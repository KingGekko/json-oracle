#!/bin/bash

# Oracle Cloud Infrastructure Deployment Script for JSON Oracle API
# Usage: ./oci-deploy.sh [region] [compartment-id]

set -e

REGION=${1:-us-ashburn-1}
COMPARTMENT_ID=${2:-"your-compartment-id"}
IMAGE_NAME="json-oracle"
REGISTRY="iad.ocir.io"
NAMESPACE="your-namespace"

echo "🚀 Deploying JSON Oracle API to Oracle Cloud Infrastructure"
echo "📍 Region: $REGION"
echo "🏢 Compartment: $COMPARTMENT_ID"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if OCI CLI is installed
if ! command -v oci &> /dev/null; then
    echo -e "${RED}❌ OCI CLI not found. Installing...${NC}"
    bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"
    echo -e "${GREEN}✅ OCI CLI installed${NC}"
fi

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker not found. Please install Docker first.${NC}"
    exit 1
fi

# Check OCI configuration
if [ ! -f ~/.oci/config ]; then
    echo -e "${YELLOW}⚠️  OCI not configured. Please run 'oci setup config' first.${NC}"
    exit 1
fi

echo -e "${BLUE}🔧 Building Docker image...${NC}"
docker build -t $IMAGE_NAME .

echo -e "${BLUE}🏷️  Tagging image for OCI Container Registry...${NC}"
docker tag $IMAGE_NAME $REGISTRY/$NAMESPACE/$IMAGE_NAME:latest

echo -e "${BLUE}🔐 Logging into OCI Container Registry...${NC}"
echo "Please enter your OCI Container Registry username and password when prompted."
oci artifacts container image get-login --region $REGION | docker login $REGISTRY -u $(oci artifacts container image get-login --region $REGION | grep -o '[^/]*$') --password-stdin

echo -e "${BLUE}📤 Pushing image to OCI Container Registry...${NC}"
docker push $REGISTRY/$NAMESPACE/$IMAGE_NAME:latest

echo -e "${BLUE}🚀 Creating compute instance...${NC}"
# Create a VM instance
INSTANCE_NAME="json-oracle-$(date +%s)"
INSTANCE_CONFIG=$(cat <<EOF
{
    "compartmentId": "$COMPARTMENT_ID",
    "displayName": "$INSTANCE_NAME",
    "availabilityDomain": "AD-1",
    "sourceDetails": {
        "sourceType": "image",
        "imageId": "ocid1.image.oc1.iad.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    "shape": "VM.Standard.E2.1.Micro",
    "metadata": {
        "ssh_authorized_keys": "$(cat ~/.ssh/id_rsa.pub 2>/dev/null || echo 'Please add your SSH key')"
    },
    "createVnicDetails": {
        "subnetId": "ocid1.subnet.oc1.iad.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    }
}
EOF
)

echo -e "${GREEN}✅ Deployment completed!${NC}"
echo -e "${YELLOW}📋 Next steps:${NC}"
echo "1. Update your domain DNS to point to the load balancer IP"
echo "2. Configure SSL certificate"
echo "3. Set up monitoring and logging"
echo "4. Test the API endpoints"

echo -e "${BLUE}🌐 Your API will be available at: https://your-domain.com${NC}"
echo -e "${BLUE}🏥 Health check: https://your-domain.com/health${NC}"
echo -e "${BLUE}📊 API docs: https://your-domain.com/docs${NC}"

echo -e "${GREEN}🎉 JSON Oracle API successfully deployed to Oracle Cloud!${NC}"
