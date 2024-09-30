# Stage 1: Build the SvelteKit App
FROM node:18 AS builder
WORKDIR /usr/src/sveltekit-app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# Stage 2: Caddy Reverse Proxy
FROM caddy:latest
COPY --from=builder /usr/src/sveltekit-app/build /srv
COPY Caddyfile /etc/caddy/Caddyfile

EXPOSE 80
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile"]
