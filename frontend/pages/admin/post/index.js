import { Metadata, PostForm } from "@/components";
import { BACKEND, BACKEND_API } from "@/config";
import { useRouter } from 'next/router';

export default function CreatePost({ all_folders }) {
  const router = useRouter()
  const { parent } = router.query

  const handleSubmit = async (data) => {
    const res = await fetch(BACKEND_API + "/blog/posts", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      const { slug, hidden, signature } = await res.json();
      document.location.href = hidden ? `/blog/h/${slug}?s=${signature}` : `/blog/p/${slug}`;
    }
  }

  return (
    <>
      <Metadata title="Create Post" />
      <h1>Create Post</h1>
      <PostForm content={{ folder: parent }} all_folders={all_folders} handleSubmit={handleSubmit} />
    </>
  )
}

export async function getServerSideProps() {
  const res = await fetch(BACKEND + "/blog/folders")
  const all_folders = await res.json()

  return {
    props: {
      all_folders
    }
  }
}
