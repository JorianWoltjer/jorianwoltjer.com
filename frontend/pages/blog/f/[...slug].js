import { Breadcrumbs, FolderItem, Loading, Metadata, PostItem } from "@/components";
import { BACKEND, SLUG_REGEX } from "@/config";
import { faEdit, faFolderPlus, faPlus } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Link from 'next/link';
import { useRouter } from 'next/router';
import { useEffect } from "react";

function splitSentence(sentence) {
    const first = /^.*?(\w[!?.] |$)/.exec(sentence)[0];
    const rest = sentence.slice(first.length);
    return [first, rest];
}

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

    const [descriptionFirst, descriptionRest] = splitSentence(content.description);

    return <>
        <Metadata title={"Folder: " + content.title} description={content.description} img={content.img} />
        <Breadcrumbs slug={content.slug} title={content.title} />
        <hr />
        <p className="lead">{descriptionFirst}<span className="desktop-only">{descriptionRest}</span></p>
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
