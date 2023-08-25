import { BACKEND, BACKEND_API } from "@/config"

export default function EditPost({ content, all_folders }) {
    const handleSubmit = async (e) => {
        e.preventDefault();

        const { folder, title, description, img, markdown } = e.target;

        const res = await fetch(BACKEND_API + "/blog/post/" + content.id, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                folder: parseInt(folder.value) || null,
                title: title.value,
                description: description.value,
                img: img.value,
                markdown: markdown.value
            })
        });

        if (res.ok) {
            const { slug } = await res.json();
            document.location.href = "/blog/p/" + slug;
        }
    }

    return (
        <>
            <h1>Edit</h1>
            <form onSubmit={handleSubmit}>
                <input name="title" type="text" placeholder="Title" defaultValue={content.title} /><br />
                <select name="folder" defaultValue={content.folder}>
                    {all_folders.map(folder => (
                        <option key={folder.id} value={folder.id}>{folder.title}</option>
                    ))}
                </select><br />
                <textarea name="description" placeholder="Description" defaultValue={content.description} /><br />
                <input name="img" type="text" placeholder="Image URL" defaultValue={content.img} /><br />
                <textarea name="markdown" placeholder="Markdown" defaultValue={content.markdown} /><br />
                <button type="submit">Submit</button>
            </form>
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
