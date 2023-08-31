import { CategoryFolder } from "@/components";
import { BACKEND } from "@/config";

export default function Blog({ content, admin_interface }) {
  return (
    <>
      <h1 className="my-4">Blog</h1>
      <div className="mb-4">
        {content.map(folder => {
          if (folder.parent === null) {
            return <CategoryFolder key={folder.id} {...folder} />
          }
        })}
      </div>
      <hr />
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
