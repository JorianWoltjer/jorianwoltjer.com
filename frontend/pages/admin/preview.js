import { Metadata, PostContent } from '@/components';
import { BACKEND_API } from '@/config';
import { useEffect, useState } from 'react';

export default function Preview() {
    const [title, setTitle] = useState("");
    const [slug, setSlug] = useState("");
    const [html, setHtml] = useState("");
    const [points, setPoints] = useState(0);
    const [hidden, setHidden] = useState(false);
    const [tags, setTags] = useState([]);

    const content = { title, slug, html, points, hidden, tags };

    useEffect(() => {
        // Add postmessage listener
        window.addEventListener("message", async (event) => {
            if (event.origin !== window.location.origin) return;
            if (event.data.type !== "preview") return;

            console.log(event.origin, event.data)
            // Render post just like in blog/p/[...slug].js getStaticProps()
            const res = await fetch(BACKEND_API + "/blog/preview", {
                method: "POST",
                body: JSON.stringify(event.data.content),
                headers: {
                    "Content-Type": "application/json"
                }
            })
            const content = await res.json()
            setTitle(content.title);
            setSlug(content.slug);
            setPoints(content.points);
            setHidden(content.hidden);
            setTags(content.tags);

            const res_html = await fetch(BACKEND_API + "/render", {
                method: "POST",
                body: content.markdown
            })
            setHtml(await res_html.text());
        });
        window.opener.postMessage({ type: "preview", ready: true }, window.location.origin);
    }, []);

    return <>
        <Metadata title={"Preview: " + title} />
        <PostContent content={content} />
    </>
}