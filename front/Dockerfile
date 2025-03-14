FROM node:23-alpine AS builder

WORKDIR /srv

COPY package*.json ./
COPY .babelrc ./

ENV NODE_OPTIONS=--openssl-legacy-provider

RUN --mount=type=cache,target=/root/.npm \
    npm install

COPY ./src ./src
COPY ./public ./public

RUN --mount=type=cache,target=/root/.npm \
    npm run build

FROM nginx:1.27.4-alpine AS runner
LABEL authors="sibears"
COPY --from=builder /srv/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]