import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { MarkdownEditor } from "@/components";
import { useState } from "react";
import { faFolder } from "@fortawesome/free-solid-svg-icons";
import { PostItem } from ".";

export default function PostForm({ content, all_folders, handleSubmit }) {
    const [title, setTitle] = useState(content.title || "");
    const [description, setDescription] = useState(content.description || "");
    const [img, setImg] = useState(content.img || "");
    const [folder, setFolder] = useState(parseInt(content.folder));
    const [markdown, setMarkdown] = useState(content.markdown || "");
    const [points, setPoints] = useState(content.points || 0);
    const [featured, setFeatured] = useState(content.featured || false);
    content = { title, description, img, folder, markdown, points, featured };

    const [previewWindow, setPreviewWindow] = useState(null);

    const onSubmit = (e) => {
        e.preventDefault();
        handleSubmit(content);
    }
    const sendPreview = (w) => {
        console.log(content);
        w.postMessage({
            type: "preview",
            content
        }, window.location.origin);
    }
    const preview = (e) => {
        e.preventDefault();

        if (!previewWindow || previewWindow.closed) {
            const child = window.open("/admin/preview", "preview");
            setPreviewWindow(child);

            window.addEventListener("message", (event) => {
                if (event.origin !== window.location.origin) return;
                if (event.data.type !== "preview") return;

                console.log(event.origin, event.data);
                if (event.data.ready) {
                    sendPreview(child);
                }
            });
        } else {
            sendPreview(previewWindow);
            previewWindow.focus();
        }
    }

    return <form onSubmit={onSubmit} id="form">
        <div className="form-floating mb-3">
            <input className="form-control" id="title" name="title" type="text" placeholder="Title" value={title} onChange={e => setTitle(e.target.value)} />
            <label htmlFor="title">Title</label>
        </div>
        <textarea className="form-control" name="description" placeholder="Description..." value={description} onChange={e => setDescription(e.target.value)} />
        <br />
        <input className="form-control" name="img" type="text" placeholder="Image URL..." value={img} onChange={e => setImg(e.target.value)} />
        <br />
        <div className="input-group mb-3">
            <label className="input-group-text" htmlFor="folder"><FontAwesomeIcon icon={faFolder} /></label>
            <select className="form-select" name="folder" value={folder} onChange={e => setFolder(parseInt(e.target.value))}>
                {all_folders.map(folder => (
                    <option key={folder.id} value={folder.id}>{folder.title}</option>
                ))}
            </select>
        </div>
        <PostItem {...content} />
        <MarkdownEditor markdown={markdown} onChange={setMarkdown} />
        <br />
        <div className="input-group mb-3 w-25">
            <span className="input-group-text">Points</span>
            <input className="form-control" name="points" type="number" value={points} onChange={e => setPoints(parseInt(e.target.value) || 0)} />
        </div>
        <div className="form-check form-switch">
            <label className="form-check-label" htmlFor="featured">Featured</label>
            <input className="form-check-input" id="featured" type="checkbox" name="featured" checked={featured} onChange={e => setFeatured(e.target.checked)} />
        </div>
        <br />
        <div className="float-end">
            <button className="btn btn-primary" type="submit"
                onClick={e => confirm("Are you sure you want to save?") ? null : e.preventDefault()}>Save</button>
            <button className="btn btn-secondary" onClick={preview}>Preview</button>
        </div>
    </form>
}
