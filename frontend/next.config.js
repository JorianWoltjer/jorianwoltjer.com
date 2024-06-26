/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    remotePatterns: [{ protocol: "http", hostname: "nginx", port: "" }],
  },
  async redirects() {
    return [
      {
        source: '/sitemap',
        destination: '/sitemap.xml',
        permanent: true,
      },
      ...[
        '/rss',
        '/feed.xml',
        '/rss.xml',
        '/blog/rss',
        '/blog/feed.xml',
        '/blog.rss',
      ].map(source => ({
        source,
        destination: '/blog/rss.xml',
        permanent: true,
      }))
    ]
  }
}

module.exports = nextConfig
