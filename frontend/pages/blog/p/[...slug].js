import { Loading, Metadata, PostContent } from "@/components";
import { BACKEND, BACKEND_API, SLUG_REGEX } from "@/config";
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
    <Metadata title={"Post: " + content.title} description={content.description} img={`/img/blog/${content.img}`} />
    <Head>
      <link rel="alternate" type="application/rss+xml" href="https://jorianwoltjer.com/blog/rss.xml" title="Blog | Jorian Woltjer" />
    </Head>
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
      headers: {
        "X-Internal": process.env.INTERNAL_TOKEN
      },
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
