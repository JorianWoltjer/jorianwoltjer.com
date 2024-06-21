import { CategoryFolder, Metadata, RelativeTime, Tags } from "@/components";
import { BACKEND } from "@/config";
import { faEye } from "@fortawesome/free-regular-svg-icons";
import { faEyeSlash, faMagnifyingGlass, faSquareRss } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { motion } from "framer-motion";
import Image from "next/image";
import Link from "next/link";
import Head from "next/head";

const variants = {
  visible: { opacity: 1, x: 0 },
  hidden: (direction) => ({
    opacity: direction === 0 ? 1 : 0, x: 20 * direction,
  })
}

function getDirection() {
  if (typeof window === 'undefined') return 1

  let before = new URL(navigation.entries().at(-1).url)
  if (before.href === location.href) {
    before = new URL(navigation.entries().at(-2).url)
  }
  console.log(before, location)

  const [beforeSlashes, afterSlashes] = [before, location].map(url => url.pathname.split('/').length - 1)
  console.log(before.pathname, location.pathname, beforeSlashes, afterSlashes)

  const diff = afterSlashes - beforeSlashes
  return diff === 0 ? 0 : diff / Math.abs(diff)
}

export default function Blog({ root_folders, featured_posts, admin_interface }) {
  return (
    <>
      <Metadata title="Blog" description="A blog with cybersecurity-related articles. Writeups of challenges in Capture The Flag (CTF) events, stories about hacking and guides with code examples and detailed explanations." />
      <Head>
        <link rel="alternate" type="application/rss+xml" href="https://jorianwoltjer.com/blog/rss.xml" title="Blog | Jorian Woltjer" />
      </Head>
      <h1 className="my-4">Blog</h1>
      <motion.main
        custom={getDirection()}
        initial="hidden"
        animate="visible"
        variants={variants}
        transition={{ ease: "easeOut", duration: 0.2 }}
      >
        <div className="mb-3">
          {root_folders.map(folder => {
            if (folder.parent === null) {
              return <CategoryFolder key={folder.id} {...folder} />
            }
          })}
        </div>
        <hr />
        <Link className='big-button big-button-wide' href='/blog/search'><FontAwesomeIcon icon={faMagnifyingGlass} />Search</Link>
        {/* eslint-disable-next-line @next/next/no-html-link-for-pages */}
        <a className='big-button big-button-icon' href='/blog/rss.xml' title="RSS Feed"><FontAwesomeIcon icon={faSquareRss} /></a>
        {admin_interface && <Link href="/admin/hidden" className="big-button"><FontAwesomeIcon icon={faEyeSlash} />Hidden posts</Link>}

        <h3 className="my-4">Recent posts</h3>
        <div className="row row-cols-1 row-cols-md-2 g-4">
          {featured_posts.map(post => <div key={post.id} className="col">
            <div className="card h-100">
              <a href={`/blog/p/${post.slug}`}><div className="card-img-top"><Image fill src={`http://nginx/img/blog/${post.img || '../placeholder.png'}`} alt="Thumbnail" /></div></a>
              <div className="card-body">
                <Tags tags={post.tags} />
                <h4 className="card-title">
                  <a href={`/blog/p/${post.slug}`}>{post.title}</a>
                </h4>
                <p className="card-text">{post.description}</p>
              </div>
              <div className="card-footer text-muted">
                <RelativeTime timestamp={post.timestamp} /> - <span className="darken"><FontAwesomeIcon icon={faEye} /> {post.views} views</span>
              </div>
            </div>
          </div>)}
        </div>
      </motion.main>
    </>
  )
}

export async function getStaticProps() {
  const res = await fetch(BACKEND + "/blog/folders")
  const root_folders = await res.json()

  const res_featured = await fetch(BACKEND + "/blog/featured")
  const featured_posts = await res_featured.json()

  return {
    props: {
      root_folders,
      featured_posts
    }
  }
}
