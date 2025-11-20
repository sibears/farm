#!/usr/bin/env bash
# Локальная сборка Docker образа

set -e

# Цвета
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🐳 Сборка Docker образа фронтенда${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo ""

# Параметры
IMAGE_NAME="${IMAGE_NAME:-sibears-farm-frontend}"
TAG="${TAG:-latest}"
CTF_BACKEND_URL="${CTF_BACKEND_URL:-http://localhost:8080}"

echo -e "${GREEN}📝 Параметры сборки:${NC}"
echo "  Image: $IMAGE_NAME:$TAG"
echo "  Backend URL: $CTF_BACKEND_URL"
echo ""

# Сборка образа
echo -e "${BLUE}🔨 Начинаем сборку...${NC}"
docker build \
    --build-arg CTF_BACKEND_URL="$CTF_BACKEND_URL" \
    --tag "$IMAGE_NAME:$TAG" \
    --file Dockerfile \
    .

echo ""
echo -e "${GREEN}✅ Образ собран успешно!${NC}"
echo ""
echo -e "${BLUE}Для запуска контейнера:${NC}"
echo "  docker run -p 3000:80 $IMAGE_NAME:$TAG"
echo ""
echo -e "${BLUE}Для запуска с переменными окружения:${NC}"
echo "  docker run -p 3000:80 -e CTF_BACKEND_URL=http://your-backend:8080 $IMAGE_NAME:$TAG"


