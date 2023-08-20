import { useRouter } from 'next/router'
import { BACKEND } from "@/config";

export default function Post({ content }) {
    const router = useRouter()

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <h1>Post - {content.title}</h1>
        <pre><code>{content.markdown}</code></pre>
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
        const res = await fetch(BACKEND + "/blog/post/" + params.slug.join("/"))
        const content = await res.json()

        return {
            props: {
                content
            }
        }
    } catch (err) {
        return {
            notFound: true
        }
    }
}
