FROM node:22-alpine AS builder

WORKDIR /app

# Install dependencies
COPY package.json ./
COPY package-lock.json ./
RUN npm install .

COPY ./ .
ENV NODE_ENV=production
ENV PORT=3000
RUN npm run build
RUN npm prune --production

FROM node:22.12-alpine AS release

WORKDIR /app

COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/build ./build
COPY --from=builder /app/public ./public
COPY --from=builder /app/package.json ./package.json

EXPOSE 3000
CMD ["npm", "run", "start"]