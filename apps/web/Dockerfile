FROM node:24-bookworm AS base
WORKDIR /app
ENV PNPM_HOME=/pnpm
ENV PATH=$PNPM_HOME:$PATH

COPY apps/web/package.json ./package.json
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://get.pnpm.io/install.sh | env PNPM_VERSION=10.23.0 SHELL=/bin/bash bash -
RUN pnpm install

COPY apps/web .
RUN pnpm build

FROM node:24-bookworm-slim AS runtime
WORKDIR /app
ENV PNPM_HOME=/pnpm
ENV PATH=$PNPM_HOME:$PATH
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://get.pnpm.io/install.sh | env PNPM_VERSION=10.23.0 SHELL=/bin/bash bash -
COPY --from=base /app/package.json ./package.json
COPY --from=base /app/node_modules ./node_modules
COPY --from=base /app/build ./build
EXPOSE 3000
CMD ["pnpm", "start"]
