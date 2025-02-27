FROM node:lts-alpine

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
# Allow `node` user to create .next/ folder and write XML files
RUN mkdir .next && touch public/sitemap.xml public/blog/rss.xml
RUN chown -R node:node .next public/sitemap.xml public/blog/rss.xml

USER node

# Build is delayed to wait for backend to be running
CMD ["sh", "-c", "npm run build && npm run start"]
