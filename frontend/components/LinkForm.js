import { LinkItem } from "@/components";
import { faFolder } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useState } from "react";

const noSubmit = e => e.key == "Enter" ? e.preventDefault() : null;

export default function LinkForm({ content: content_, all_folders, handleSubmit }) {
  const [title, setTitle] = useState(content_.title || "");
  const [url, setUrl] = useState(content_.url || "");
  const [description, setDescription] = useState(content_.description || "");
  const [img, setImg] = useState(content_.img || "");
  const [folder, setFolder] = useState(parseInt(content_.folder));
  const [featured, setFeatured] = useState(content_.featured || false);
  const content = { title, url, description, img, folder, featured };

  const onSubmit = (e) => {
    e.preventDefault();
    handleSubmit(content);
  }

  return <form onSubmit={onSubmit} id="form">
    <div className="input-group mb-3">
      <div className="form-floating">
        <input className="form-control" id="title" name="title" type="text" placeholder="Title" value={title}
          onChange={e => setTitle(e.target.value)} onKeyDown={noSubmit} required autoFocus />
        <label htmlFor="title">Title</label>
      </div>
      <div className="form-floating">
        <input className="form-control text-body-secondary" id="url" name="url" type="url" placeholder="URL" value={url}
          onChange={e => setUrl(e.target.value)} onKeyDown={noSubmit} required />
        <label htmlFor="url" className="text-body-secondary">URL</label>
      </div>
    </div>
    <textarea className="form-control" name="description" placeholder="Description..." value={description} onChange={e => setDescription(e.target.value)} />
    <br />
    <input className="form-control" name="img" type="text" placeholder="Image path..." value={img}
      onChange={e => setImg(e.target.value)} onKeyDown={noSubmit} />
    <br />
    <div className="input-group mb-3">
      <label className="input-group-text" htmlFor="folder"><FontAwesomeIcon icon={faFolder} /></label>
      <select className="form-select" name="folder" value={folder} onChange={e => setFolder(parseInt(e.target.value))}>
        {all_folders.map(folder => (
          <option key={folder.id} value={folder.id}>{folder.title}</option>
        ))}
      </select>
    </div>
    <LinkItem {...content} timestamp={content_.timestamp} views={content_.views} />
    <br />
    <div className="form-check form-switch">
      <label className="form-check-label" htmlFor="featured">Featured</label>
      <input className="form-check-input" id="featured" type="checkbox" name="featured" checked={featured} onChange={e => setFeatured(e.target.checked)} />
    </div>
    <br />
    <div className="float-end">
      <button className="btn btn-primary" type="submit"
        onClick={e => confirm("Are you sure you want to save?") ? null : e.preventDefault()}>Save</button>
    </div>
  </form>
}
