FROM node:lts-alpine

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
# Allow node to create .next/ folder
RUN chown node:node .

USER node

# Build is delayed to wait for backend to be running
CMD ["sh", "-c", "npm run build && npm run start"]
