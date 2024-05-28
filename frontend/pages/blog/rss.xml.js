import { BACKEND } from "@/config";
import { xmlEscape } from "@/utils/strings";

const HOST = 'https://jorianwoltjer.com';

function item(post) {
    return `<item>
        <media:thumbnail url="${HOST}/img/blog/${post.img || '../placeholder.png'}" />
        <title>${post.title}</title>
        <description>${post.description}</description>
        <link>${HOST}/blog/p/${post.slug}</link>
        <guid isPermaLink="true">${HOST}/blog/p/${post.id}</guid>
        <pubDate>${new Date(post.timestamp).toUTCString()}</pubDate>
    </item>`;
}

function generateRSS(posts) {
    posts = xmlEscape(posts);
    return `<?xml version="1.0" encoding="UTF-8"?>
    <rss version="2.0" xmlns:media="http://search.yahoo.com/mrss/" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
    <title>Blog | Jorian Woltjer</title>
    <link>${HOST}/blog</link>
    <atom:link href="${HOST}/blog/rss.xml" rel="self" type="application/rss+xml" />
    <description>A blog with cybersecurity-related articles. Writeups of challenges in Capture The Flag (CTF) events, stories about hacking and guides with code examples and detailed explanations.</description>
    <image>
        <title>Blog | Jorian Woltjer</title>
        <url>${HOST}/img/logo.png</url>
        <link>${HOST}/blog</link>
    </image>
    ${posts.map(item).join('')}
    </channel>
</rss>`
}

function RSSFeed() { }

// Server-Side because NextJS can't return XML content-type :(, good thing Cloudflare caches it
export async function getServerSideProps({ res }) {
    const res_posts = await fetch(BACKEND + "/blog/posts");
    const posts = await res_posts.json();

    // Generate the XML RSS feed with the data
    const sitemap = generateRSS(posts);

    res.setHeader('Content-Type', 'text/xml');
    res.write(sitemap);
    res.end();

    return {
        props: {},
    };
}

export default RSSFeed;