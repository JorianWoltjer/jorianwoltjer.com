import { MarkdownEditor, PostItem } from "@/components";
import { BACKEND_API } from "@/config";
import { slugify, timeDifference, toLocalTime } from "@/utils/strings";
import { faFolder, faTimesCircle } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useState } from "react";
import useSWRImmutable from 'swr/immutable';

const fetcher = url => fetch(url).then(res => res.json());
const noSubmit = e => e.key == "Enter" ? e.preventDefault() : null;

export default function PostForm({ content: content_, all_folders, handleSubmit }) {
  const [title, setTitle] = useState(content_.title || "");
  const [slug, setSlug] = useState(content_.slug?.split("/").at(-1) || "");
  const [description, setDescription] = useState(content_.description || "");
  const [img, setImg] = useState(content_.img || "");
  const [folder, setFolder] = useState(parseInt(content_.folder));
  const [markdown, setMarkdown] = useState(content_.markdown || "");
  const [points, setPoints] = useState(content_.points || 0);
  const [featured, setFeatured] = useState(content_.featured || false);
  const [hidden, setHidden] = useState(content_.hidden || false);
  const [autoreleaseCheck, setAutoreleaseCheck] = useState(Boolean(content_.autorelease) || false);
  const [autorelease, setAutorelease] = useState(autoreleaseCheck ? new Date(content_.autorelease) : new Date(Date.now() + 60 * 60 * 24 * 1000));
  const [tags, setTags] = useState(content_.tags || []);
  const content = { title, slug, description, img, folder, markdown, points, featured, hidden, tags, autorelease: autoreleaseCheck ? autorelease.toISOString() : null };

  const [previewWindow, setPreviewWindow] = useState(null);
  const { data: all_tags } = useSWRImmutable(BACKEND_API + "/blog/tags", fetcher);

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
    <div className="input-group mb-3">
      <div className="form-floating">
        <input className="form-control" id="title" name="title" type="text" placeholder="Title" value={title}
          onChange={e => { setTitle(e.target.value), setSlug(slugify(e.target.value)) }} onKeyDown={noSubmit} required autoFocus />
        <label htmlFor="title">Title</label>
      </div>
      <div className="form-floating">
        <input className={`form-control text-body-secondary ${(title && !slug) && 'is-invalid'}`}
          id="slug" name="slug" type="text" placeholder="URL" value={slug}
          onChange={e => setSlug(e.target.value)} onBlur={e => setSlug(slugify(slug))}
          onKeyDown={noSubmit} required />
        <label htmlFor="slug" className="text-body-secondary">URL</label>
      </div>
    </div>
    <textarea className="form-control" name="description" placeholder="Description..." value={description} onChange={e => setDescription(e.target.value)} />
    <br />
    <input className="form-control" name="img" type="text" placeholder="Image URL..." value={img}
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
    <PostItem {...content} timestamp={content_.timestamp} views={content_.views} />
    <MarkdownEditor markdown={markdown} onChange={setMarkdown} />
    <br />
    <div className="input-group mb-3" style={{ maxWidth: "20ch" }}>
      <span className="input-group-text">Points</span>
      <input className="form-control" name="points" type="number" value={points}
        onChange={e => setPoints(parseInt(e.target.value) || 0)} onKeyDown={noSubmit} />
    </div>
    <div className="tags">
      <label htmlFor="tag-add" className="pe-2">Tags:</label>
      {tags.map(tag =>
        <span key={tag.name} className={`tag tag-${tag.color}`}>{tag.name}
          <FontAwesomeIcon icon={faTimesCircle} className="tag-delete"
            onClick={() => setTags(tags.filter(t => t.name != tag.name))} />
        </span>
      )}
      <input className="tag tag-add" id="tag-add" list="all-tags" placeholder="+ Add" autoComplete="off" onChange={e => {
        let new_tag = all_tags.find(tag => tag.name == e.target.value);
        if (new_tag === undefined || tags.find(tag => tag.id == new_tag.id)) return;
        setTags([...tags, new_tag]);
        e.target.value = "";
      }} onKeyDown={e => {
        if (e.key == "Enter") {
          e.preventDefault();
          let new_tag = all_tags.find(tag => tag.name.toLowerCase().startsWith(e.target.value.toLowerCase()));
          if (new_tag === undefined || tags.find(tag => tag.id == new_tag.id)) return;
          setTags([...tags, new_tag]);
          e.target.value = "";
        }
      }} />
      <datalist id="all-tags"
        onChange={e => setTags(Array.from(e.target.selectedOptions, option => all_tags.find(tag => tag.name == option.value)))}>
        {all_tags?.filter(tag => !tags.find(t => t.id == tag.id)).map(tag => (
          <option key={tag.name} value={tag.name} />
        ))}
      </datalist>
    </div>
    <div className="form-check form-switch">
      <label className="form-check-label" htmlFor="featured">Featured</label>
      <input className="form-check-input" id="featured" type="checkbox" name="featured" checked={featured} onChange={e => setFeatured(e.target.checked)} />
    </div>
    <div className="form-check form-switch">
      <label className="form-check-label" htmlFor="hidden">Hidden</label>
      <input className="form-check-input" id="hidden" type="checkbox" name="hidden" checked={hidden} onChange={e => setHidden(e.target.checked)} />
    </div>
    {hidden && <>
      <div className="form-check form-switch">
        <label className="form-check-label" htmlFor="autoreleasecheck">Auto-Release</label>
        <input className="form-check-input" id="autoreleasecheck" type="checkbox" name="autoreleasecheck" checked={autoreleaseCheck} onChange={e => setAutoreleaseCheck(e.target.checked)} />
        <label className="form-label" htmlFor="autorelease">&nbsp;at&nbsp;</label>
        <input id="autorelease" name="autorelease" type="datetime-local" value={autorelease && toLocalTime(autorelease)}
          onChange={e => {
            setAutoreleaseCheck(true);
            setAutorelease(new Date(e.target.value))
          }} />
        &nbsp;({timeDifference(autorelease.getTime())})
      </div>
    </>}
    <br />
    <div className="float-end">
      <button className="btn btn-primary" type="submit"
        onClick={e => confirm("Are you sure you want to save?") ? null : e.preventDefault()}>Save</button>
      <button className="btn btn-secondary" onClick={preview}>Preview</button>
    </div>
  </form>
}
