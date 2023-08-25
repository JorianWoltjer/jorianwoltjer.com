import { BACKEND, BACKEND_API } from "@/config";

export default function EditFolder({ content, all_folders }) {
    const handleSubmit = async (e) => {
        e.preventDefault();

        const { parent, title, description, img } = e.target;

        const res = await fetch(BACKEND_API + "/blog/folder/" + content.id, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                parent: parseInt(parent.value) || null,
                title: title.value,
                description: description.value,
                img: img.value
            })
        });

        if (res.ok) {
            const { slug } = await res.json();
            document.location.href = "/blog/f/" + slug;
        }
    }

    return (
        <>
            <h1>Edit</h1>
            <form onSubmit={handleSubmit}>
                <input name="title" type="text" placeholder="Title" defaultValue={content.title} /><br />
                <select name="parent" defaultValue={content.parent}>
                    <option value="">-</option>
                    {all_folders.map(folder => (
                        <option key={folder.id} value={folder.id}>{folder.title}</option>
                    ))}
                </select><br />
                <textarea name="description" placeholder="Description" defaultValue={content.description} /><br />
                <input name="img" type="text" placeholder="Image URL" defaultValue={content.img} /><br />
                <button type="submit">Submit</button>
            </form>
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
