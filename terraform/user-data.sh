#!/bin/bash

# User data script for Oracle Cloud Infrastructure
# This script sets up the JSON Oracle API on Ubuntu

set -e

# Update system
apt-get update
apt-get upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
usermod -aG docker ubuntu

# Install Docker Compose
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Install Git
apt-get install -y git

# Create application directory
mkdir -p /opt/json-oracle
cd /opt/json-oracle

# Clone the repository (you'll need to update this with your actual repo)
git clone https://github.com/KingGekko/json-oracle.git .

# Create environment file
cat > .env << EOF
RUST_LOG=info
PORT=${api_port}
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
MAX_TIMEOUT_SECONDS=120
EOF

# Create systemd service for the API
cat > /etc/systemd/system/json-oracle.service << EOF
[Unit]
Description=JSON Oracle API
After=docker.service
Requires=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/opt/json-oracle
ExecStart=/usr/local/bin/docker-compose up -d
ExecStop=/usr/local/bin/docker-compose down
TimeoutStartSec=0
User=ubuntu

[Install]
WantedBy=multi-user.target
EOF

# Enable and start the service
systemctl enable json-oracle.service
systemctl start json-oracle.service

# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull a default model
ollama pull llama2

# Create Ollama service
cat > /etc/systemd/system/ollama.service << EOF
[Unit]
Description=Ollama Service
After=network.target

[Service]
Type=simple
User=ollama
ExecStart=/usr/local/bin/ollama serve
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

# Create ollama user
useradd -r -s /bin/false ollama
mkdir -p /usr/share/ollama
chown ollama:ollama /usr/share/ollama

# Enable and start Ollama
systemctl enable ollama.service
systemctl start ollama.service

# Install Nginx for reverse proxy
apt-get install -y nginx

# Configure Nginx
cat > /etc/nginx/sites-available/json-oracle << EOF
server {
    listen 80;
    server_name _;

    location / {
        proxy_pass http://localhost:${api_port};
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
    }
}
EOF

# Enable the site
ln -s /etc/nginx/sites-available/json-oracle /etc/nginx/sites-enabled/
rm /etc/nginx/sites-enabled/default

# Restart Nginx
systemctl restart nginx

# Install fail2ban for security
apt-get install -y fail2ban

# Configure fail2ban
cat > /etc/fail2ban/jail.local << EOF
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 3

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
EOF

systemctl enable fail2ban
systemctl start fail2ban

# Install monitoring tools
apt-get install -y htop iotop nethogs

# Create log rotation for application logs
cat > /etc/logrotate.d/json-oracle << EOF
/opt/json-oracle/logs/*.log {
    daily
    missingok
    rotate 7
    compress
    delaycompress
    notifempty
    create 644 ubuntu ubuntu
}
EOF

# Set up automatic security updates
apt-get install -y unattended-upgrades
echo 'Unattended-Upgrade::Automatic-Reboot "false";' >> /etc/apt/apt.conf.d/50unattended-upgrades

# Create a health check script
cat > /opt/json-oracle/health-check.sh << EOF
#!/bin/bash
curl -f http://localhost:${api_port}/health || exit 1
EOF

chmod +x /opt/json-oracle/health-check.sh

# Add health check to crontab
echo "*/5 * * * * /opt/json-oracle/health-check.sh" | crontab -

echo "JSON Oracle API setup completed successfully!"
echo "API will be available at: http://$(curl -s ifconfig.me)"
echo "Health check: http://$(curl -s ifconfig.me)/health"
