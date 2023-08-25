import { BACKEND } from "@/config";
import Link from "next/link";

export default function Blog({ content, admin_interface }) {
  return (
    <>
      <h1>Blog</h1>
      {admin_interface && (<>
        <Link href="/admin/post">Create Post</Link>
        <Link href="/admin/folder">Create Folder</Link>
      </>)}
      <ul>
        {content.map(folder => {
          if (folder.parent === null) {
            return <li key={folder.slug}>
              <a href={`/blog/f/${folder.slug}`}>{folder.title}</a>
            </li>
          }
        })}
      </ul>
    </>
  )
}

export async function getStaticProps() {
  const res = await fetch(BACKEND + "/blog/folders")
  const content = await res.json()

  return {
    props: {
      content
    }
  }
}
