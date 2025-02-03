FROM node:21-alpine as builder
LABEL authors="x5113nc3x"

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

FROM nginx:1.24.0-alpine
COPY --from=builder /srv/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]