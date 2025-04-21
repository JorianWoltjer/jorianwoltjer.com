import { Loading, Metadata, PostContent, TransitionAnimator } from "@/components";
import { BACKEND, BACKEND_API, SLUG_REGEX, HOST } from "@/config";
import { getRenderedPost } from "@/utils/api";
import { faEdit } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useRouter } from 'next/router';
import { useEffect } from "react";
import Link from 'next/link';
import Head from "next/head";

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
    <Metadata title={content.title} description={content.description} img={`/img/blog/${content.img}`} />
    <Head>
      <link rel="alternate" type="application/rss+xml" href={`${HOST}/blog/rss.xml`} title="Blog | Jorian Woltjer" />
    </Head>
    <TransitionAnimator>
      <PostContent content={content} admin_interface={admin_interface} admin_components={
        <Link className="big-button" href={`/admin/post/${content.id}`}><FontAwesomeIcon icon={faEdit} /> Edit</Link>
      } />
    </TransitionAnimator>
  </>
}

export async function getStaticPaths() {
  let posts = await fetch(BACKEND + "/blog/posts").then(res => res.json());

  return {
    paths: posts.map(post => post.Post).filter(Boolean).map(post => ({
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
    let content;
    try {
      content = await getRenderedPost(slug);
    } catch (err) {
      console.error(err);
      const res = await fetch(BACKEND + "/blog/post/" + slug)
      if (!res.ok) {
        // Try to find folder with the same name, and redirect to it
        const res2 = await fetch(BACKEND + "/blog/folder/" + slug)
        if (res2.ok) {
          return {
            redirect: {
              destination: "/blog/f/" + slug,
              permanent: true
            }
          }
        }
      }
      throw err;
    }

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
