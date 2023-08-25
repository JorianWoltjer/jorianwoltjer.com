import { BACKEND, SLUG_REGEX } from "@/config";
import Breadcrumbs from "@/components/Breadcrumbs";
import { useRouter } from 'next/router'
import Link from 'next/link';

export default function Folder({ content, admin_interface }) {
    const router = useRouter()

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        {admin_interface && <>
            <Link href={`/admin/post?folder=${content.id}`}>New Post</Link>
            <Link href={`/admin/folder?folder=${content.id}`}>New Folder</Link>
            <Link href={`/admin/folder/${content.id}`}>Edit</Link>
        </>}
        <ul>
            {content.folders.map(folder => (
                <li key={folder.slug}>
                    <Link href={`/blog/f/${folder.slug}`}>Folder - {folder.title}</Link>
                </li>
            ))}
            {content.posts.map(post => (
                <li key={post.slug}>
                    <Link href={`/blog/p/${post.slug}`}>Post - {post.title}</Link>
                </li>
            ))}
        </ul>
    </>
}

export async function getStaticPaths() {
    let posts = await fetch(BACKEND + "/blog/folders").then(res => res.json());

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
        const res = await fetch(BACKEND + "/blog/folder/" + slug)
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
