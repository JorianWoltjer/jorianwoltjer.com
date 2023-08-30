import { BACKEND, SLUG_REGEX } from "@/config";
import Breadcrumbs from "@/components/Breadcrumbs";
import { useEffect } from "react";
import { useRouter } from 'next/router'
import Link from 'next/link';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css'
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faEdit } from "@fortawesome/free-solid-svg-icons";

export default function Post({ content, admin_interface }) {
    const router = useRouter()

    useEffect(() => {
        hljs.initHighlighting();
    }, []);

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        {admin_interface &&
            <Link className="big-button" href={`/admin/post/${content.id}`}><FontAwesomeIcon icon={faEdit} /> Edit</Link>
        }
        <div className='post-content' dangerouslySetInnerHTML={{ __html: content.html }} />
    </>
}

export async function getStaticPaths() {
    let posts = await fetch(BACKEND + "/blog/posts").then(res => res.json());

    return {
        paths: posts.map(post => ({
            params: {
                slug: post.slug.split("/")
            }
        })),
        fallback: true
    }
}

export async function getStaticProps({ params }) {
    try {
        const slug = params.slug.join("/")
        if (!SLUG_REGEX.test(slug)) {  // Sanity check
            throw new Error("Invalid slug: " + slug)
        }
        const res = await fetch(BACKEND + "/blog/post/" + slug)
        const content = await res.json()

        const res_html = await fetch(BACKEND + "/render", {
            method: "POST",
            body: content.markdown
        })
        content.html = await res_html.text()

        return {
            props: {
                content
            }
        }
    } catch (err) {
        console.error(err)
        return {
            notFound: true
        }
    }
}
