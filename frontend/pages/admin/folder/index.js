import { BACKEND, BACKEND_API } from "@/config";
import { useRouter } from 'next/router'

export default function CreateFolder({ all_folders }) {
    const router = useRouter()
    const { folder } = router.query

    const handleSubmit = async (e) => {
        e.preventDefault();

        const { parent, title, description, img } = e.target;

        const res = await fetch(BACKEND_API + "/blog/folders", {
            method: "POST",
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
            <h1>Create</h1>
            <form onSubmit={handleSubmit}>
                <input name="title" type="text" placeholder="Title" /><br />
                <select name="parent" defaultValue={folder}>
                    <option value="">-</option>
                    {all_folders.map(folder => (
                        <option key={folder.id} value={folder.id}>{folder.title}</option>
                    ))}
                </select><br />
                <textarea name="description" placeholder="Description" /><br />
                <input name="img" type="text" placeholder="Image URL" defaultValue="placeholder.png" /><br />
                <button type="submit">Submit</button>
            </form>
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
