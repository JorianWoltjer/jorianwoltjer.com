import { BACKEND, SLUG_REGEX } from "@/config";
import { Breadcrumbs, FolderItem, PostItem, Loading } from "@/components";
import { useEffect } from "react";
import { useRouter } from 'next/router'
import Link from 'next/link';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faEdit, faFolderPlus, faPlus } from "@fortawesome/free-solid-svg-icons";

export default function Folder({ content, admin_interface }) {
    const router = useRouter()

    useEffect(() => {
        if (!router.isFallback) {
            // Replace URL if slug is not correct
            if (content.slug !== router.query.slug.join("/")) {
                router.replace("/blog/f/" + content.slug)
            }
        }
    }, [content, router]);

    if (router.isFallback) {
        return <Loading />
    }

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        <hr />
        <p className="lead">{content.description}</p>
        {admin_interface && <>
            <Link className="big-button" href={`/admin/post?parent=${content.id}`}><FontAwesomeIcon icon={faPlus} /> New Post</Link>
            <Link className="big-button" href={`/admin/folder?parent=${content.id}`}><FontAwesomeIcon icon={faFolderPlus} /> New Folder</Link>
            <Link className="big-button" href={`/admin/folder/${content.id}`}><FontAwesomeIcon icon={faEdit} /> Edit</Link>
        </>}
        {content.folders.map(folder => (
            <FolderItem key={folder.id} {...folder} />
        ))}
        {content.posts.map(post => (
            <PostItem key={post.id} {...post} />
        ))}
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
