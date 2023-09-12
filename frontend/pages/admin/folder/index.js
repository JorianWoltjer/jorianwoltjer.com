import { FolderForm, Metadata } from "@/components";
import { BACKEND, BACKEND_API } from "@/config";
import { useRouter } from 'next/router'

export default function CreateFolder({ all_folders }) {
    const router = useRouter()
    const { parent } = router.query

    const handleSubmit = async (data) => {
        const res = await fetch(BACKEND_API + "/blog/folders", {
            method: "POST",
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
            <Metadata title="Create Folder" />
            <h1>Create Folder</h1>
            <FolderForm content={{ parent }} all_folders={all_folders} handleSubmit={handleSubmit} />
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
