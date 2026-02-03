FROM oven/bun:1 AS base
WORKDIR /app

ARG BACKEND_URL=http://backend:8080

FROM base AS deps
COPY package.json bun.lockb* ./
RUN --mount=type=cache,target=/root/.bun/install/cache \
	bun install --frozen-lockfile

FROM base AS builder
ARG BACKEND_URL
COPY --from=deps /app/node_modules ./node_modules
COPY . .

ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production
ENV BACKEND_URL=${BACKEND_URL}

RUN --mount=type=cache,target=/app/.next/cache \
	--mount=type=cache,target=/root/.bun/install/cache \
	bun run build

FROM base AS runner
ARG BACKEND_URL

WORKDIR /app

COPY --from=builder /app/public ./public

COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

EXPOSE 80

ENV PORT=80
ENV HOSTNAME="0.0.0.0"
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV BACKEND_URL=${BACKEND_URL}

CMD ["bun", "server.js"]
