import { Metadata, LinkForm } from "@/components";
import { BACKEND, BACKEND_API } from "@/config";

export default function EditLink({ content, all_folders }) {
  const handleSubmit = async (data) => {
    const res = await fetch(BACKEND_API + "/blog/link/" + content.id, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      const { folder } = await res.json();
      document.location.href = `/blog/f/${folder}`;
    }
  }

  return (
    <>
      <Metadata title={"Edit " + content.title} />
      <h1>Edit Link</h1>
      <LinkForm content={content} all_folders={all_folders} handleSubmit={handleSubmit} />
    </>
  )
}

export async function getServerSideProps({ params, query }) {
  try {
    const { id } = params;
    if (!/^\d+$/.test(id)) {
      throw new Error("Invalid link ID: " + id)
    }

    const res = await fetch(BACKEND + `/blog/link/${id}`)
    const content = await res.json()

    const res_all = await fetch(BACKEND + "/blog/folders")
    const all_folders = await res_all.json()

    return {
      props: {
        content,
        all_folders
      }
    }
  } catch (err) {  // Not found
    console.error(err)
    return {
      notFound: true
    }
  }
}
