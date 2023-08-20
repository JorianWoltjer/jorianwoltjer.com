export default async function handler(req, res) {
  const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
  if (ip !== '127.0.0.1') {  // Only from localhost (backend)
    return res.status(403).json({ message: "Forbidden" });
  } else if (req.method !== "POST") {  // Only POST
    return res.status(405).json({ message: "Method not allowed" });
  }
  const { path } = req.body;
  if (!path) {  // Require path
    return res.status(400).json({ message: "Missing path" });
  }

  try {
    await res.revalidate(path)
    return res.json({ revalidated: true })
  } catch (err) {
    return res.status(500).json({ message: err.message })
  }
}
