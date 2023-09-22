/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ["nginx"],
  },
  async headers() {
    return [
      {
        source: '/:path*{/}?',
        headers: [
          {
            key: 'X-Frame-Options',
            value: 'DENY',
          },
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff',
          },
          {
            key: 'Content-Security-Policy',
            value: `
              default-src 'self';
              script-src 'self' ${process.env.NODE_ENV === 'development' ? "'unsafe-eval'" : ''};
              style-src 'self' 'unsafe-inline';
              object-src 'none';
              connect-src 'self' http://localhost:8000 ws://localhost:8000 wss://localhost:8000 ws://jorianwoltjer.com wss://jorianwoltjer.com;
              font-src 'self' fonts.gstatic.com;
              img-src 'self' data:;
              frame-src www.youtube-nocookie.com;
              frame-ancestors 'none';
          `.replace(/\s+/g, ' ').trim(),
          },
        ],
      }
    ]
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
