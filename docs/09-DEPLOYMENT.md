# 09 — Deployment Guide

## Deployment Options

| Option | Best For | Effort | Cost |
|--------|----------|--------|------|
| Docker Compose (single VPS) | Solo admin, < 100 users | Low | €8/mo |
| Docker Compose (multiple VPS) | Up to 1000 users | Medium | €30/mo |
| Kubernetes | > 1000 users, HA | High | €100+/mo |
| Managed (future SaaS) | Zero ops | Zero | TBD |

---

## Option 1: Single VPS — Docker Compose (Recommended for Start)

### Server Requirements
```
Minimum:
  CPU:    2 vCPU
  RAM:    4 GB
  Disk:   40 GB SSD
  OS:     Ubuntu 24.04 LTS
  
Recommended (Hetzner CX21):
  CPU:    2 vCPU (AMD)
  RAM:    4 GB
  Disk:   40 GB NVMe SSD
  Price:  €4.15/month
  Network: 20 TB/month included
```

### Initial Server Setup
```bash
#!/bin/bash
# Run as root on fresh Ubuntu 24.04

# Update system
apt-get update && apt-get upgrade -y

# Install Docker
curl -fsSL https://get.docker.com | sh
usermod -aG docker ubuntu

# Install Docker Compose v2
apt-get install -y docker-compose-plugin

# Install Nginx + Certbot
apt-get install -y nginx certbot python3-certbot-nginx

# Firewall
ufw default deny incoming
ufw default allow outgoing
ufw allow 22/tcp    # SSH
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw allow 3478/udp  # STUN/TURN
ufw allow 3478/tcp  # STUN/TURN
ufw allow 5349/udp  # TURN TLS
ufw allow 5349/tcp  # TURN TLS
ufw --force enable

# Create app directory
mkdir -p /opt/borderless
chown ubuntu:ubuntu /opt/borderless
```

### Deploy the Application
```bash
# As ubuntu user
cd /opt/borderless
git clone https://github.com/rythmmcosta/Borderless .

# Configure environment
cp .env.example .env
nano .env  # Fill in all values

# Start all services
docker compose -f deploy/docker-compose.yml up -d

# Check status
docker compose -f deploy/docker-compose.yml ps
docker compose -f deploy/docker-compose.yml logs -f api

# Verify health
curl http://localhost:8080/v1/health
```

### SSL with Let's Encrypt
```bash
# Point your domain DNS to server IP first
certbot --nginx -d your-domain.com -d api.your-domain.com

# Nginx config: /etc/nginx/sites-available/borderless
cat > /etc/nginx/sites-available/borderless << 'NGINX'
server {
    listen 443 ssl http2;
    server_name your-domain.com api.your-domain.com;

    ssl_certificate     /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    
    ssl_protocols TLSv1.3;
    
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;

    location /v1/ {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_read_timeout 300s;
    }
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}

server {
    listen 80;
    server_name _;
    return 301 https://$host$request_uri;
}
NGINX

ln -sf /etc/nginx/sites-available/borderless /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```

### Automated Backups
```bash
#!/bin/bash
# /opt/borderless/backup.sh
set -e

BACKUP_DIR="/opt/borderless/backups"
DATE=$(date +%Y%m%d_%H%M%S)
DB_CONTAINER="borderless-db-1"

mkdir -p "$BACKUP_DIR"

# PostgreSQL backup
docker exec "$DB_CONTAINER" \
    pg_dump -U postgres borderless | gzip > "$BACKUP_DIR/db_$DATE.sql.gz"

# Keep only last 30 days
find "$BACKUP_DIR" -name "*.sql.gz" -mtime +30 -delete

echo "Backup complete: db_$DATE.sql.gz"
# Add to crontab: 0 2 * * * /opt/borderless/backup.sh
```

---

## Option 2: Kubernetes (Production)

### Kubernetes Manifests

```yaml
# deploy/kubernetes/api-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: borderless-api
  namespace: borderless
spec:
  replicas: 2
  selector:
    matchLabels:
      app: borderless-api
  template:
    metadata:
      labels:
        app: borderless-api
    spec:
      containers:
        - name: api
          image: ghcr.io/rythmmcosta/borderless:latest
          ports:
            - containerPort: 8080
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: borderless-secrets
                  key: database-url
            - name: JWT_SECRET
              valueFrom:
                secretKeyRef:
                  name: borderless-secrets
                  key: jwt-secret
          resources:
            requests:
              memory: "128Mi"
              cpu: "100m"
            limits:
              memory: "512Mi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /v1/health
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 10
```

### Helm Chart

```bash
# Install via Helm
helm repo add borderless https://rythmmcosta.github.io/Borderless/helm
helm install borderless borderless/borderless \
  --namespace borderless \
  --create-namespace \
  --set adminEmail=your@email.com \
  --set jwtSecret=$(openssl rand -hex 32) \
  --set ingress.host=borderless.yourdomain.com
```

---

## Environment Variables Reference

```bash
# Required
HOST=0.0.0.0
PORT=8080
DATABASE_URL=postgresql://user:pass@host:5432/borderless
REDIS_URL=redis://:password@host:6379
JWT_SECRET=minimum-32-chars-random-secret
ADMIN_EMAIL=your@email.com

# Optional
JWT_EXPIRY_SECS=900
REFRESH_EXPIRY_SECS=2592000
CORS_ORIGINS=http://localhost:3000

# Supabase (optional)
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-anon-key
SUPABASE_SERVICE_KEY=your-service-role-key

# TURN relay
TURN_SERVER_URL=turn:your-domain.com:3478
TURN_USERNAME=borderless
TURN_PASSWORD=random-secret

# Logging
RUST_LOG=borderless_server=info,tower_http=info,sqlx=warn
```

---

## Monitoring & Observability

### Health Endpoints
```
GET /v1/health              → 200 OK { "status": "ok", "version": "0.1.0" }
GET /v1/health/db           → 200 if PostgreSQL reachable
GET /v1/health/redis        → 200 if Redis reachable
```

### Metrics (Prometheus)
```
borderless_http_requests_total{method, path, status}
borderless_http_request_duration_seconds{method, path}
borderless_active_sessions
borderless_online_devices
borderless_clipboard_syncs_total
borderless_file_transfers_total{status}
```

---

## Upgrade Procedure

```bash
cd /opt/borderless
git pull origin main
docker compose -f deploy/docker-compose.yml pull
docker compose -f deploy/docker-compose.yml up -d --no-deps api
docker compose -f deploy/docker-compose.yml logs -f api
```

---

## Production Checklist

- [ ] All environment variables set in production
- [ ] Secrets not in git (`.env` in `.gitignore`)
- [ ] TLS certificate valid and auto-renewing
- [ ] HTTPS redirect working
- [ ] Database backup running
- [ ] Monitoring configured (health check alerts)
- [ ] Admin account created and tested
- [ ] Rate limiting tested
- [ ] CORS configured for production domain
- [ ] Firewall rules reviewed
- [ ] Audit log working
- [ ] Remote session notification working
