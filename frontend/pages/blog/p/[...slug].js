import { BACKEND, SLUG_REGEX } from "@/config";
import { Breadcrumbs, Loading } from "@/components";
import { useEffect } from "react";
import { useRouter } from 'next/router'
import Link from 'next/link';

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faEdit } from "@fortawesome/free-solid-svg-icons";
import PostContent from "@/components/PostContent";

export default function Post({ content, admin_interface }) {
    const router = useRouter()

    useEffect(() => {
        if (!router.isFallback) {
            // Replace URL if slug is not correct
            if (content.slug !== router.query.slug.join("/")) {
                router.replace("/blog/p/" + content.slug)
            }
        }
    }, [content, router]);

    if (router.isFallback) {
        return <Loading />
    }

    return <>
        <PostContent content={content} admin_interface={admin_interface} admin_components={
            <Link className="big-button" href={`/admin/post/${content.id}`}><FontAwesomeIcon icon={faEdit} /> Edit</Link>
        } />
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
    } catch (err) {  // Not found
        console.error(err)
        return {
            notFound: true
        }
    }
}
