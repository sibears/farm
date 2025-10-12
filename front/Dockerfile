# syntax=docker/dockerfile:1
# Используем официальный образ Bun
FROM oven/bun:1 AS base
WORKDIR /app

# Устанавливаем зависимости
FROM base AS deps
COPY package.json bun.lockb* ./
# Кеширование установки зависимостей Bun для ускорения повторных сборок
RUN --mount=type=cache,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

# Сборка приложения
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Переменные окружения для сборки
ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production

# Build args для настройки во время сборки
ARG CTF_BACKEND_URL=http://localhost:8080

# Устанавливаем переменные окружения для сборки
ENV CTF_BACKEND_URL=${CTF_BACKEND_URL}

# Кеширование Next.js build и Bun кеша для ускорения сборки
RUN --mount=type=cache,target=/app/.next/cache \
    --mount=type=cache,target=/root/.bun/install/cache \
    bun run build

# Production образ
FROM base AS runner
WORKDIR /app

# Создаем пользователя nextjs для безопасности
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

# Копируем необходимые файлы из builder
COPY --from=builder /app/public ./public

# Автоматически используем output traces для уменьшения размера образа
# https://nextjs.org/docs/advanced-features/output-file-tracing
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 80

# Runtime переменные окружения
ENV PORT=80
ENV HOSTNAME="0.0.0.0"
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

# Переменные могут быть переопределены при запуске контейнера
ENV CTF_BACKEND_URL=http://backend:8080

# Запускаем приложение
CMD ["bun", "server.js"]