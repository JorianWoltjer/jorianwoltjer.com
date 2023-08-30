import { BACKEND } from "@/config";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFlag, faTerminal, faLaptopCode } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";

const icons = {
  "flag": faFlag,
  "terminal": faTerminal,
  "laptop": faLaptopCode
}

export default function Blog({ content, admin_interface }) {
  return (
    <>
      <h1 className="my-4">Blog</h1>
      {content.map(folder => {
        if (folder.parent === null) {
          return <Link key={folder.id} className="big-button" href={`/blog/f/${folder.slug}`}>
            <FontAwesomeIcon icon={icons[folder.img]} />
            {folder.title}
          </Link>
        }
      })}
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
