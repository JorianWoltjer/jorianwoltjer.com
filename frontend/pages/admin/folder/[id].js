import { FolderForm, Metadata } from "@/components";
import { BACKEND, BACKEND_API } from "@/config";

export default function EditFolder({ content, all_folders }) {
  const handleSubmit = async (data) => {
    const res = await fetch(BACKEND_API + "/blog/folder/" + content.id, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(data)
    });

    if (res.ok) {
      const { slug } = await res.json();
      document.location.href = "/blog/f/" + slug;
    }
  }

  return (
    <>
      <Metadata title={"Edit Folder: " + content.title} />
      <h1>Edit Folder</h1>
      <FolderForm content={content} all_folders={all_folders} handleSubmit={handleSubmit} />
    </>
  )
}

export async function getServerSideProps({ params }) {
  try {
    const res_all = await fetch(BACKEND + "/blog/folders")
    const all_folders = await res_all.json()

    const { id } = params;
    if (!/^\d+$/.test(id)) {
      throw new Error("Invalid folder ID: " + id)
    }
    const res = await fetch(BACKEND + "/blog/folder/" + id)
    const content = await res.json()

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
