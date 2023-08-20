FROM node:18-alpine

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
# RUN npm run build

# CMD ["npm", "run", "start"]
# Build is delayed to wait for backend to be running
CMD ["sh", "-c", "npm run build && npm run start"]
