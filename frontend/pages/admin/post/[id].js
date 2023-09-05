import { PostForm } from "@/components";
import { BACKEND, BACKEND_API } from "@/config"

export default function EditPost({ content, all_folders }) {
    const handleSubmit = async (data) => {
        const res = await fetch(BACKEND_API + "/blog/post/" + content.id, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });

        if (res.ok) {
            const { slug } = await res.json();
            document.location.href = "/blog/p/" + slug;
        }
    }

    return (
        <>
            <h1>Edit</h1>
            <PostForm content={content} all_folders={all_folders} handleSubmit={handleSubmit} />
        </>
    )
}

export async function getServerSideProps({ params }) {
    const res_all = await fetch(BACKEND + "/blog/folders")
    const all_folders = await res_all.json()

    const { id } = params;
    const res = await fetch(BACKEND + "/blog/post/" + id)
    const content = await res.json()

    return {
        props: {
            content,
            all_folders
        }
    }
}
