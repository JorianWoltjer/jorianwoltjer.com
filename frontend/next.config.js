/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ["nginx"],
  },
  async redirects() {
    return [
      {
        source: '/sitemap',
        destination: '/sitemap.xml',
        permanent: true,
      },
      {
        source: '/rss',
        destination: '/blog/rss.xml',
        permanent: true,
      },
      {
        source: '/blog/rss',
        destination: '/blog/rss.xml',
        permanent: true,
      },
    ]
  }
}

module.exports = nextConfig
