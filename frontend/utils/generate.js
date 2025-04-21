import { BACKEND, HOST } from "@/config";
import { xmlEscape as escape, cdataEscape } from "@/utils/strings";
import { getRenderedPost } from "@/utils/api";

function rssItem(post) {
  const p = post.Post || post.Link;
  const url = post.Post ? `${escape(HOST)}/blog/p/${escape(p.slug)}` : p.url;
  const permaUrl = post.Post ? `${escape(HOST)}/blog/p/${escape(String(p.id))}` : p.url;

  return `<item>
      <title>${escape(p.title)}</title>
      <link>${url}</link>
      <guid isPermaLink="true">${permaUrl}</guid>
      <media:thumbnail url="${escape(HOST)}/img/blog/${escape(p.img) || '../placeholder.png'}" />
      <description>${escape(p.description)}</description>
      <pubDate>${escape(new Date(p.timestamp).toUTCString())}</pubDate>
${post.Post ? `      <content:encoded>${cdataEscape(p.html)}</content:encoded>\n` : ''}\
    </item>`;
}

function rssFull(posts) {
  return `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" 
  xmlns:media="http://search.yahoo.com/mrss/"
  xmlns:atom="http://www.w3.org/2005/Atom"
  xmlns:content="http://purl.org/rss/1.0/modules/content/"
>
  <channel>
    <title>Blog | Jorian Woltjer</title>
    <link>${escape(HOST)}/blog</link>
    <atom:link href="${escape(HOST)}/blog/rss.xml" rel="self" type="application/rss+xml" />
    <description>A blog with cybersecurity-related articles. Writeups of challenges in Capture The Flag (CTF) events, stories about hacking and guides with code examples and detailed explanations.</description>
    <image>
      <title>Blog | Jorian Woltjer</title>
      <url>${escape(HOST)}/img/logo.png</url>
      <link>${escape(HOST)}/blog</link>
    </image>
    ${posts.map(rssItem).join('')}
  </channel>
</rss>`
}

export async function rss() {
  const res_posts = await fetch(BACKEND + "/blog/posts");
  let posts = await res_posts.json()
  // Limit to 10 most recent posts
  posts.sort((a, b) => {
    const timestampA = a[Object.keys(a)[0]].timestamp
    const timestampB = b[Object.keys(b)[0]].timestamp
    return timestampB - timestampA
  });
  posts = posts.slice(0, 10);
  posts = await Promise.all(posts.map(async post => post.Post ? { Post: await getRenderedPost(post.Post.id) } : post));

  // Generate the XML RSS feed with the data
  return rssFull(posts);
}

function sitemapUrl(path, timestamp) {
  timestamp = timestamp || new Date().toISOString();
  return `<url>
    <loc>${escape(HOST + path)}</loc>
    <lastmod>${escape(timestamp)}</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.7</priority>
  </url>`;
}

function sitemapFull(posts, folders) {
  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">
  ${sitemapUrl('/')}
  ${sitemapUrl('/blog')}
  ${posts.map(post => post.Post).filter(Boolean).map(post => sitemapUrl('/blog/p/' + post.slug, post.timestamp))}
  ${folders.map(folder => sitemapUrl('/blog/f/' + folder.slug, folder.timestamp))}
</urlset>`
}

export async function sitemap() {
  const res_posts = await fetch(BACKEND + "/blog/posts");
  const posts = await res_posts.json();

  const res_folders = await fetch(BACKEND + "/blog/folders");
  const folders = await res_folders.json();

  // Generate the XML sitemap with the data
  return sitemapFull(posts, folders);
}

export async function generateXML() {
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const fs = await import('fs');
    const path = await import('path');

    const sitemapXml = await sitemap()
    const rssXml = await rss()
    fs.writeFileSync(path.join(process.cwd(), "public", "sitemap.xml"), sitemapXml)
    fs.writeFileSync(path.join(process.cwd(), "public", "blog", "rss.xml"), rssXml)
  }
}
