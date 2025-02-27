import { BACKEND } from "@/config";

export async function getRenderedPost(slug_or_id) {
  const res = await fetch(BACKEND + "/blog/post/" + slug_or_id)
  const post = await res.json()

  const res_html = await fetch(BACKEND + "/render", {
    method: "POST",
    headers: {
      "X-Internal": process.env.INTERNAL_TOKEN
    },
    body: post.markdown
  })
  post.html = await res_html.text()

  return post
}