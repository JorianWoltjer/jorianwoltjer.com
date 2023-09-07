import { FolderItem, CategoryFolder } from "@/components";
import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFolder } from "@fortawesome/free-solid-svg-icons";

export default function FolderForm({ content: content_, all_folders, handleSubmit }) {
    const [title, setTitle] = useState(content_.title || "");
    const [description, setDescription] = useState(content_.description || "");
    const [img, setImg] = useState(content_.img || "");
    const [parent, setParent] = useState(parseInt(content_.parent) || null);
    const content = { title, description, img, parent };

    const onSubmit = (e) => {
        e.preventDefault();
        handleSubmit(content);
    }

    return <form onSubmit={onSubmit} id="form">
        <div className="form-floating mb-3">
            <input className="form-control" id="title" name="title" type="text" placeholder="Title" value={title}
                onChange={e => setTitle(e.target.value)} autoFocus />
            <label htmlFor="title">Title</label>
        </div>
        <textarea className="form-control" name="description" placeholder="Description..." value={description} onChange={e => setDescription(e.target.value)} />
        <br />
        <input className="form-control" name="img" type="text" placeholder="Image URL..." value={img} onChange={e => setImg(e.target.value)} />
        <br />
        <div className="input-group mb-3">
            <label className="input-group-text" htmlFor="folder"><FontAwesomeIcon icon={faFolder} /></label>
            <select className="form-select" name="folder" value={parent} onChange={e => setParent(parseInt(e.target.value) || null)}>
                <option value={null}>-</option>
                {all_folders.map(folder => (
                    <option key={folder.id} value={folder.id}>{folder.title}</option>
                ))}
            </select>
        </div>
        {(parent === null) ?
            <div className="mb-4"><CategoryFolder {...content} /></div> :
            <FolderItem {...content} timestamp={content_.timestamp} views={content_.views} />}
        <div className="float-end">
            <input className="btn btn-primary" type="submit" value="Save"
                onClick={e => confirm("Are you sure you want to save?") ? null : e.preventDefault()} />
        </div>
    </form>
}
