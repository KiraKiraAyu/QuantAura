# Build stage
FROM node:20-alpine AS build-stage
WORKDIR /app

RUN npm install -g pnpm

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY web/package.json ./web/package.json
RUN pnpm install --frozen-lockfile

COPY web ./web
WORKDIR /app/web
RUN pnpm run build

# Production stage
FROM nginx:alpine AS production-stage
COPY --from=build-stage /app/web/dist /usr/share/nginx/html
COPY web/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
