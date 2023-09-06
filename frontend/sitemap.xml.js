import { BACKEND } from "@/config";

const HOST = 'https://jorianwoltjer.com';

function url(path, timestamp) {
    timestamp = timestamp || new Date().toISOString();
    return `<url>
        <loc>${HOST + path}</loc>
        <lastmod>${timestamp}</lastmod>
        <changefreq>daily</changefreq>
        <priority>0.7</priority>
    </url>`;
}

function generateSitemap(posts, folders) {
    return `<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">
        ${url('/')}
        ${url('/blog')}
        ${posts.map(post => url('/blog/p/' + post.slug, post.timestamp))}
        ${folders.map(folder => url('/blog/f/' + folder.slug, folder.timestamp))}
    </urlset>`
}

function SiteMap() { }

export async function getServerSideProps({ res }) {
    const res_posts = await fetch(BACKEND + "/blog/posts");
    const posts = await res_posts.json();

    const res_folders = await fetch(BACKEND + "/blog/folders");
    const folders = await res_folders.json();

    // Generate the XML sitemap with the data
    const sitemap = generateSitemap(posts, folders);

    res.setHeader('Content-Type', 'text/xml');
    res.write(sitemap);
    res.end();

    return {
        props: {},
    };
}

export default SiteMap;