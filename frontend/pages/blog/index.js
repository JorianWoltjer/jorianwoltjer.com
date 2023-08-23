import { BACKEND, BACKEND_API } from "@/config";
import Link from "next/link";
import { useEffect, useState } from "react";

export default function Blog({ content }) {
  // TODO: move this to global thing
  const [admin_interface, set_admin_interface] = useState(false);

  useEffect(() => {
    if (document.cookie.includes("admin_interface=true")) {
      set_admin_interface(true);
    }
  }, [])

  const logout = async () => {
    await fetch(BACKEND_API + "/logout", {
      method: "GET",
      credentials: "same-origin"
    })
    document.cookie = "admin_interface=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";

    set_admin_interface(false);
  }

  return (
    <>
      {admin_interface && (<>
        <a onClick={logout} href="#">Logout</a>
      </>)}
      <h1>Blog</h1>
      {admin_interface && (<>
        <Link href="/admin/create_post">Create Post</Link>
        <Link href="/admin/create_folder">Create Folder</Link>
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
