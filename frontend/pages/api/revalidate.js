import path from 'path';

export default async function handler(req, res) {
  const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
  if (ip !== '127.0.0.1') {  // Only from localhost (backend)
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

  return res.status(204).end()
}