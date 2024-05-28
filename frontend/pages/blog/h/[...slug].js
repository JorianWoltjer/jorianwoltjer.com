import { Metadata, PostContent } from "@/components";
import { BACKEND, SLUG_REGEX } from "@/config";
import { faEdit } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useRouter } from 'next/router';
import { useEffect } from 'react';
import Link from 'next/link';
import Head from "next/head";

export default function HiddenPost({ content, admin_interface, signature }) {
  const router = useRouter()

  useEffect(() => {
    // Redirect to regular post if no longer hidden
    if (!content.hidden) {
      router.replace("/blog/p/" + content.slug)
    }
  }, [content, router]);

  return <>
    <Metadata title={"Hidden: " + content.title} description={content.description} img={`/img/blog/${content.img}`} />
    <Head>
      <link rel="alternate" type="application/rss+xml" href="https://jorianwoltjer.com/blog/rss.xml" title="Blog | Jorian Woltjer" />
    </Head>
    <PostContent content={content} admin_interface={admin_interface} admin_components={
      <Link className="big-button" href={`/admin/post/${content.id}?s=${signature}`}><FontAwesomeIcon icon={faEdit} /> Edit</Link>
    } />
  </>
}

export async function getServerSideProps({ params, query }) {
  try {
    const slug = params.slug.join("/")
    if (!SLUG_REGEX.test(slug)) {  // Sanity check
      throw new Error("Invalid slug: " + slug)
    }
    const { s: signature } = query;
    if (!/^[a-f0-9]{64}$/.test(signature)) {  // Sanity check
      throw new Error("Invalid signature: " + signature)
    }
    const res = await fetch(BACKEND + `/blog/hidden/${slug}?signature=${signature}`)
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
        content,
        signature
      }
    }
  } catch (err) {  // Not found
    console.error(err)
    return {
      notFound: true
    }
  }
}
