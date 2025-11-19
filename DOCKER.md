# 🐳 Docker Deployment Guide

This guide explains how to build and run TileMania using Docker.

---

## 📋 Prerequisites

- Docker 20.10+ installed
- Docker Compose 2.0+ (optional, but recommended)
- At least 2GB free disk space
- A word list (see [Lexicon Setup](#lexicon-setup))

---

## 🚀 Quick Start

### Production Build

```bash
# 1. Build the image
docker-compose build tilemania

# 2. Ensure you have a lexicon (see Lexicon Setup below)
# Download ENABLE (public domain)
curl -o assets/lexicons/ENABLE.txt http://www.puzzlers.org/pub/wordlists/enable1.txt

# 3. Run the container
docker-compose up tilemania
```

### Development Build (with hot-reload)

```bash
# Build development image
docker-compose build tilemania-dev

# Run with auto-reload
docker-compose up tilemania-dev
```

---

## 📦 Building Images

### Production Image

**Using docker-compose:**
```bash
docker-compose build tilemania
```

**Using docker directly:**
```bash
docker build -t tilemania:latest .
```

**Build args:**
```bash
docker build \
  --build-arg RUST_VERSION=1.70 \
  -t tilemania:latest \
  .
```

### Development Image

```bash
docker-compose build tilemania-dev
```

**Or:**
```bash
docker build -f Dockerfile.dev -t tilemania:dev .
```

---

## 🎮 Running the Game

### Using Docker Compose (Recommended)

**Production:**
```bash
# Start
docker-compose up tilemania

# Start in background
docker-compose up -d tilemania

# Stop
docker-compose down

# View logs
docker-compose logs -f tilemania
```

**Development:**
```bash
# Start with hot-reload
docker-compose up tilemania-dev

# Rebuild and restart
docker-compose up --build tilemania-dev
```

### Using Docker Directly

```bash
# Run production container
docker run -it \
  --name tilemania \
  -v $(pwd)/assets/lexicons:/app/assets/lexicons:ro \
  -p 8080:8080 \
  tilemania:latest

# Run development container
docker run -it \
  --name tilemania-dev \
  -v $(pwd):/app \
  -v cargo-cache:/usr/local/cargo/registry \
  -v target-cache:/app/target \
  -p 8080:8080 \
  tilemania:dev
```

---

## 📁 Lexicon Setup

### ⚠️ Important: Word List Required

TileMania requires a word list to function. The lexicon is **NOT** included in the Docker image due to licensing.

### Option 1: ENABLE (Public Domain) ✅ Recommended

```bash
# Download ENABLE lexicon (public domain)
mkdir -p assets/lexicons
curl -o assets/lexicons/ENABLE.txt \
  http://www.puzzlers.org/pub/wordlists/enable1.txt

# Verify download
wc -l assets/lexicons/ENABLE.txt
# Should show ~173,000 words
```

### Option 2: Mount Your Own Lexicon

If you have a licensed CSW24 or TWL lexicon:

```bash
# Place your lexicon in assets/lexicons/
cp /path/to/CSW24.txt assets/lexicons/

# Run with mounted lexicon
docker run -it \
  -v $(pwd)/assets/lexicons:/app/assets/lexicons:ro \
  tilemania:latest
```

### Option 3: Custom Word List

Create your own custom word list:

```bash
# Create custom lexicon
cat > assets/lexicons/custom.txt <<EOF
AA
AB
HELLO
WORLD
EOF

# Run container
docker-compose up tilemania
```

---

## 🔧 Configuration

### Environment Variables

Set via `docker-compose.yml` or command line:

```bash
docker run -it \
  -e RUST_LOG=debug \
  -e RUST_BACKTRACE=full \
  tilemania:latest
```

**Available variables:**
- `RUST_LOG` - Log level (error, warn, info, debug, trace)
- `RUST_BACKTRACE` - Backtrace level (0, 1, full)

### Volume Mounts

**Production:**
```yaml
volumes:
  - ./assets/lexicons:/app/assets/lexicons:ro  # Word lists (required)
  - ./assets/audio:/app/assets/audio:ro        # Audio files (optional)
  - ./assets/fonts:/app/assets/fonts:ro        # Fonts (optional)
  - ./assets/sprites:/app/assets/sprites:ro    # Sprites (optional)
```

**Development:**
```yaml
volumes:
  - .:/app                                      # Mount entire project
  - cargo-cache:/usr/local/cargo/registry      # Cache dependencies
  - target-cache:/app/target                   # Cache build artifacts
```

### Resource Limits

Adjust in `docker-compose.yml`:

```yaml
deploy:
  resources:
    limits:
      cpus: '2.0'
      memory: 2G
    reservations:
      cpus: '0.5'
      memory: 512M
```

---

## 🛠️ Development Workflow

### Hot-Reload Development

```bash
# Start development container with auto-reload
docker-compose up tilemania-dev

# Container will automatically rebuild when you edit code
# Changes are detected via cargo-watch
```

### Running Tests

```bash
# Run tests in development container
docker-compose run tilemania-dev cargo test

# Run with coverage
docker-compose run tilemania-dev \
  cargo tarpaulin --verbose --all-features
```

### Running Clippy/Format

```bash
# Check formatting
docker-compose run tilemania-dev cargo fmt -- --check

# Run clippy
docker-compose run tilemania-dev cargo clippy

# Auto-format code
docker-compose run tilemania-dev cargo fmt
```

### Shell Access

```bash
# Get shell in running container
docker exec -it tilemania-dev /bin/bash

# Or run a new container with shell
docker-compose run --rm tilemania-dev /bin/bash
```

---

## 🚢 Production Deployment

### Building for Production

```bash
# Build optimized production image
docker build \
  --no-cache \
  -t tilemania:v1.0.0 \
  -t tilemania:latest \
  .
```

### Multi-Architecture Builds

```bash
# Build for multiple platforms (requires buildx)
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t tilemania:latest \
  --push \
  .
```

### Pushing to Registry

```bash
# Tag for registry
docker tag tilemania:latest your-registry.com/tilemania:latest

# Push
docker push your-registry.com/tilemania:latest
```

### Running in Production

```bash
# Using docker-compose with production config
docker-compose -f docker-compose.yml up -d tilemania

# Or use docker run with restart policy
docker run -d \
  --name tilemania-prod \
  --restart unless-stopped \
  -v $(pwd)/assets/lexicons:/app/assets/lexicons:ro \
  -p 8080:8080 \
  tilemania:latest
```

---

## 🔍 Troubleshooting

### Container Won't Start

**Check logs:**
```bash
docker-compose logs tilemania
```

**Common issues:**
1. **No lexicon found:**
   ```
   Error: No word list found!
   ```
   **Solution:** Ensure lexicon is mounted correctly.

2. **Permission denied:**
   ```
   Error: Permission denied (os error 13)
   ```
   **Solution:** Check file permissions on mounted volumes.

3. **Out of memory:**
   ```
   Error: Cannot allocate memory
   ```
   **Solution:** Increase Docker memory limit in Docker Desktop settings.

### Build Failures

**Clean rebuild:**
```bash
# Remove all images and caches
docker-compose down --rmi all --volumes

# Rebuild from scratch
docker-compose build --no-cache tilemania
```

**Check disk space:**
```bash
docker system df
docker system prune -a  # Clean up unused images
```

### Development Container Issues

**Cargo lock errors:**
```bash
# Remove lock file and rebuild
rm Cargo.lock
docker-compose build --no-cache tilemania-dev
```

**Cached dependencies not updating:**
```bash
# Remove cargo cache volume
docker volume rm tilemania_cargo-cache
docker-compose build tilemania-dev
```

---

## 📊 Image Information

### Image Sizes

- **Production:** ~500MB (optimized)
- **Development:** ~2GB (includes tools)

### Layers

```bash
# Inspect image layers
docker history tilemania:latest

# Show image details
docker inspect tilemania:latest
```

### Security Scanning

```bash
# Scan for vulnerabilities (requires Docker Scout or Trivy)
docker scout cves tilemania:latest

# Or using Trivy
trivy image tilemania:latest
```

---

## 🎯 Best Practices

### Production

1. ✅ Use specific version tags (not `latest`)
2. ✅ Run as non-root user (already configured)
3. ✅ Set resource limits
4. ✅ Use health checks
5. ✅ Mount lexicons as read-only
6. ✅ Scan images for vulnerabilities
7. ✅ Use multi-stage builds (already configured)

### Development

1. ✅ Use named volumes for caching
2. ✅ Mount source code as volume
3. ✅ Use hot-reload (cargo-watch)
4. ✅ Keep development image separate
5. ✅ Use docker-compose for orchestration

---

## 📝 Advanced Usage

### Custom Dockerfile

Create your own Dockerfile based on the provided ones:

```dockerfile
FROM tilemania:latest

# Add custom assets
COPY my-assets/ /app/assets/

# Custom configuration
ENV RUST_LOG=info

CMD ["./tilemania"]
```

### Docker Compose Override

Create `docker-compose.override.yml`:

```yaml
version: '3.8'

services:
  tilemania:
    environment:
      - CUSTOM_VAR=value
    ports:
      - "9090:8080"
```

### Networking

```yaml
services:
  tilemania:
    networks:
      - game-network

networks:
  game-network:
    driver: bridge
```

---

## 🆘 Getting Help

**Issues with Docker setup?**
1. Check logs: `docker-compose logs`
2. Verify lexicon: `ls -lh assets/lexicons/`
3. Test build: `docker build --progress=plain .`
4. See main documentation: `README.md`
5. Open an issue on GitHub

---

**Last Updated:** 2025-11-19
**Docker Version:** 20.10+
**Compose Version:** 2.0+
