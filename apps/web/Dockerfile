FROM node:24-bookworm AS base
WORKDIR /app
COPY apps/web/package.json ./package.json
RUN corepack enable && corepack prepare pnpm@10 --activate
RUN pnpm install

COPY apps/web .
RUN pnpm build

FROM node:24-bookworm-slim AS runtime
WORKDIR /app
RUN corepack enable && corepack prepare pnpm@10 --activate
COPY --from=base /app/package.json ./package.json
COPY --from=base /app/node_modules ./node_modules
COPY --from=base /app/build ./build
EXPOSE 3000
CMD ["pnpm", "start"]
