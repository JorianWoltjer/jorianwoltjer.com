import path from 'path';

export default async function handler(req, res) {
  const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
  if (ip !== '127.0.0.1') {  // Only from localhost (backend)
    return res.status(403).json({ message: "Forbidden" });
  } else if (req.method !== "POST") {  // Only POST
    return res.status(405).json({ message: "Method not allowed" });
  }
  const { type, slug } = req.body;
  if (!path || !type) {  // Require parameters
    return res.status(400).json({ message: "Missing parameters" });
  }
  console.log("Revalidating", { type, slug })

  try {
    if (type === "Post") {  // Post needs self and folder
      await res.revalidate(`/blog/p/${slug}`)
      await res.revalidate(`/blog/f/${path.dirname(slug)}`)

    } else if (type === "Folder") {  // Folder needs self and parent
      await res.revalidate(`/blog/f/${slug}`)
      // New folder to choose from
      await res.revalidate(`/admin/create_post`)
      await res.revalidate(`/admin/create_folder`)

      const dirname = path.dirname(slug)
      if (dirname !== ".") {
        await res.revalidate(`/blog/f/${dirname}`)
      } else {  // Root folder
        await res.revalidate(`/blog`)
      }
    }
    return res.status(204).end()
  } catch (err) {
    console.error(err)
    return res.status(500).json({ message: err.message })
  }
}