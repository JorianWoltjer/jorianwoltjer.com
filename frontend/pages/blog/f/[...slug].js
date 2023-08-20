import { useRouter } from 'next/router'
import { BACKEND } from "@/config";

export default function Post({ content }) {
    const router = useRouter()

    if (router.isFallback) {
        return <div>Loading...</div>
    }

    return <>
        <h1>Folder - {content.title}</h1>
        <ul>
            {content.folders.map(folder => (
                <li key={folder.slug}>
                    <a href={`/blog/f/${folder.slug}`}>Folder - {folder.title}</a>
                </li>
            ))}
            {content.posts.map(post => (
                <li key={post.slug}>
                    <a href={`/blog/p/${post.slug}`}>Post - {post.title}</a>
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
        const res = await fetch(BACKEND + "/blog/folder/" + params.slug.join("/"))
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
