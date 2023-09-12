import { Metadata, PostForm } from "@/components";
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
            const { slug, hidden, signature } = await res.json();
            document.location.href = hidden ? `/blog/h/${slug}?s=${signature}` : `/blog/p/${slug}`;
        }
    }

    return (
        <>
            <Metadata title={"Edit Post: " + content.title} />
            <h1>Edit Post</h1>
            <PostForm content={content} all_folders={all_folders} handleSubmit={handleSubmit} />
        </>
    )
}

export async function getServerSideProps({ params, query }) {
    try {
        const { id } = params;
        const { s: signature } = query;
        if (signature !== undefined && !/^[a-f0-9]{64}$/.test(signature)) {  // Sanity check
            throw new Error("Invalid signature: " + signature)
        }

        const url = signature ? `/blog/hidden/${id}?signature=${signature}` : `/blog/post/${id}`
        const res = await fetch(BACKEND + url)
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
