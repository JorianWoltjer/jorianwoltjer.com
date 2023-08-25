import { BACKEND, SLUG_REGEX } from "@/config";
import Breadcrumbs from "@/components/Breadcrumbs";
import { useRouter } from 'next/router'
import Link from 'next/link';

export default function Post({ content, admin_interface }) {
    const router = useRouter()

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        {admin_interface && <Link href={`/admin/post/${content.id}`}>Edit</Link>}
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
