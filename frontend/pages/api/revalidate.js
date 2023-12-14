import path from 'path';
import fs from 'fs';

function getBuildId() {
  return fs.readFileSync(".next/BUILD_ID", 'utf8')
}

async function purgeCloudflareCache(files) {
  // Convert to absolute URLs and add /_next/data URLs
  const BUILD_ID = getBuildId();
  files = files
    .concat(files.map(path => `/_next/data/${BUILD_ID}${path}.json`))
    .map(path => new URL(path, process.env.NEXT_PUBLIC_SITE_URL).toString())

  console.log("Cloudflare Purge:", files)

  // Send request to Cloudflare
  const response = await fetch(`https://api.cloudflare.com/client/v4/zones/${process.env.CLOUDFLARE_ZONE_ID}/purge_cache`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${process.env.CLOUDFLARE_API_KEY}`
    },
    body: JSON.stringify({ files })
  })
  const data = await response.json()
  if (!response.ok) {
    throw new Error(data.errors[0].message)
  }
  return data
}

export default async function handler(req, res) {
  // X-Internal header is set to "false" by nginx
  const is_internal = req.headers["x-internal"] === process.env.INTERNAL_TOKEN;
  if (!is_internal) {
    return res.status(403).json({ message: "Forbidden" });
  } else if (req.method !== "POST") {  // Only POST
    return res.status(405).json({ message: "Method not allowed" });
  }

  const revalidations = new Set();
  revalidations.add(`/blog`)  // For root folders and featured posts

  for (const request of req.body) {
    const { type, slug } = request;
    if (!path || !type) {  // Required parameters
      return res.status(400).json({ message: "Missing parameters" });
    }
    console.log("Revalidation request:", { type, slug })

    if (type === "Post") {  // Post needs self and folder
      revalidations.add(`/blog/p/${slug}`)
      revalidations.add(`/blog/f/${path.dirname(slug)}`)

    } else if (type === "Folder") {  // Folder needs self and parent
      revalidations.add(`/blog/f/${slug}`)

      const dirname = path.dirname(slug)
      revalidations.add(`/blog/f/${dirname}`)
    } else if (type === "Custom") {
      // For manually revalidating pages: [{"type": "Custom", "slug": "/projects"}]
      revalidations.add(slug)
    }
  }

  console.time("Revalidation")
  try {
    for (const url of revalidations) {
      console.log("Revalidating:", url)
      await res.revalidate(url)
    }
  } catch (err) {
    console.timeEnd("Revalidation")
    console.error(err)
    return res.status(500).json({ message: err.message })
  }
  console.timeEnd("Revalidation")

  if (process.env.CLOUDFLARE_ZONE_ID && process.env.CLOUDFLARE_API_KEY) {
    console.log(await purgeCloudflareCache(Array.from(revalidations)))
  }

  return res.status(204).end()
}