import { BACKEND } from "@/config";

export default function CreatePost({ content }) {
    const handleSubmit = async (e) => {
        e.preventDefault();

        const { folder, title, description, img, markdown } = e.target;

        const res = await fetch("/api/blog/posts", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                folder: parseInt(folder.value),
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
            <h1>Create</h1>
            <form onSubmit={handleSubmit}>
                <input name="title" type="text" placeholder="Title" /><br />
                <select name="folder">
                    {content.map(folder => (
                        <option key={folder.id} value={folder.id}>{folder.title}</option>
                    ))}
                </select><br />
                <textarea name="description" placeholder="Description" /><br />
                <input name="img" type="text" placeholder="Image URL" value="placeholder.png" /><br />
                <textarea name="markdown" placeholder="Body" /><br />
                <button type="submit">Submit</button>
            </form>
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
