import { BACKEND, SLUG_REGEX } from "@/config";
import { useRouter } from 'next/router'
import Link from 'next/link';

export default function Post({ content }) {
    const router = useRouter()

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <div className='breadcrumbs'>
            <Link href='/blog'>~</Link>
            {content.slug.split("/").slice(0, -1).map((slug, i) => {
                const path = content.slug.split("/").slice(0, i + 1).join("/")
                return <Link key={i} href={`/blog/f/${path}`}>{slug}</Link>
            })}
            <h1>{content.title}</h1>
        </div>
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
        const slug = params.slug.join("/")
        if (!SLUG_REGEX.test(slug)) {  // Sanity check
            throw new Error("Invalid slug: " + slug)
        }
        const res = await fetch(BACKEND + "/blog/post/" + slug)
        const content = await res.json()

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
