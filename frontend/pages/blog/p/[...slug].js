import { BACKEND, BACKEND_API, SLUG_REGEX } from "@/config";
import { Loading, Metadata, PostContent } from "@/components";
import { useEffect } from "react";
import { useRouter } from 'next/router'
import Link from 'next/link';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faEdit } from "@fortawesome/free-solid-svg-icons";

export default function Post({ content, admin_interface }) {
    const router = useRouter()

    useEffect(() => {
        if (!router.isFallback) {
            // Replace URL if slug is not correct
            if (content.slug !== router.query.slug.join("/")) {
                router.replace("/blog/p/" + content.slug)
            }

            setTimeout(() => {
                fetch(BACKEND_API + '/blog/add_view', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ id: content.id, signature: content.signature })
                });
            }, 5000);
        }
    }, [content, router]);

    if (router.isFallback) {
        return <Loading />
    }

    return <>
        <Metadata title={"Post: " + content.title} description={content.description} img={content.img} />
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
    const slug = params.slug.join("/")
    try {
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
        // Try to find folder with the same name, and redirect to it
        const res = await fetch(BACKEND + "/blog/folder/" + slug)
        if (res.status === 200) {
            return {
                redirect: {
                    destination: "/blog/f/" + slug,
                    permanent: true
                }
            }
        } else {
            return {
                notFound: true
            }
        }
    }
}
