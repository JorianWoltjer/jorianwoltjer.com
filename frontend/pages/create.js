export default function Create() {
    const handleSubmit = async (e) => {
        e.preventDefault();

        const { title, body } = e.target.elements;

        const res = await fetch("/api/create", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                title: title.value,
                body: body.value
            })
        });

        if (res.ok) {
            alert("Created post!");
        }
    }

    return (
        <>
            <h1>Create</h1>
            <form onSubmit={handleSubmit}>
                <input name="title" type="text" placeholder="Title" /><br />
                <textarea name="body" placeholder="Body" /><br />
                <button type="submit">Submit</button>
            </form>
        </>
    )
}
