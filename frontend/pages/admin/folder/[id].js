import { FolderForm } from "@/components";
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
            <h1>Edit Folder</h1>
            <FolderForm content={content} all_folders={all_folders} handleSubmit={handleSubmit} />
        </>
    )
}

export async function getServerSideProps({ params }) {
    const res_all = await fetch(BACKEND + "/blog/folders")
    const all_folders = await res_all.json()

    const { id } = params;
    const res = await fetch(BACKEND + "/blog/folder/" + id)
    const content = await res.json()

    return {
        props: {
            content,
            all_folders
        }
    }
}
