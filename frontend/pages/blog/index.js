import { CategoryFolder, Metadata, RelativeTime, Tags, TransitionAnimator } from "@/components";
import { BACKEND, CDN, HOST } from "@/config";
import { faEye } from "@fortawesome/free-regular-svg-icons";
import { faEyeSlash, faMagnifyingGlass, faSquareRss, faLink, faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Image from "next/image";
import Link from "next/link";
import Head from "next/head";

export default function Blog({ root_folders, featured_posts, admin_interface }) {
  return (
    <>
      <Metadata title="Blog" description="A blog with cybersecurity-related articles. Writeups of challenges in Capture The Flag (CTF) events, stories about hacking and guides with code examples and detailed explanations." />
      <Head>
        <link rel="alternate" type="application/rss+xml" href={`${HOST}/blog/rss.xml`} title="Blog | Jorian Woltjer" />
      </Head>
      <h1 className="my-4">Blog</h1>
      <TransitionAnimator>
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

        <h3 className="my-4">Featured posts</h3>
        <div className="row row-cols-1 row-cols-md-2 g-4">
          {featured_posts.map(post => {
            if (post.Post) {
              const p = post.Post;
              return <div key={"post" + p.id} className="col">
                <div className="card h-100">
                  <Link href={`/blog/p/${p.slug}`}><div className="card-img-top"><Image fill src={`${CDN}/img/blog/${p.img || '../placeholder.png'}`} alt="Thumbnail" /></div></Link>
                  <div className="card-body">
                    <Tags tags={p.tags} />
                    <h4 className="card-title">
                      <Link href={`/blog/p/${p.slug}`}>{p.title}</Link>
                    </h4>
                    <p className="card-text">{p.description}</p>
                  </div>
                  <div className="card-footer text-muted">
                    <RelativeTime timestamp={p.timestamp} /> - <span className="darken"><FontAwesomeIcon icon={faEye} /> {p.views} views</span>
                  </div>
                </div>
              </div>
            } else {
              const p = post.Link;
              const domain = new URL(p.url).hostname.replace(/^www\./, '');
              return <div key={"link" + p.id} className="col">
                <div className="card h-100">
                  <Link href={p.url} target="_blank"><div className="card-img-top"><Image fill src={`${CDN}/img/blog/${p.img || '../placeholder.png'}`} alt="Thumbnail" /></div></Link>
                  <div className="card-body">
                    <Tags tags={[{ name: <><FontAwesomeIcon icon={faArrowUpRightFromSquare} /> External</>, color: "gray" }]} />
                    <h4 className="card-title">
                      <Link href={p.url} target="_blank">{p.title}</Link>
                    </h4>
                    <p className="card-text">{p.description}</p>
                  </div>
                  <div className="card-footer text-muted">
                    <RelativeTime timestamp={p.timestamp} /> - <FontAwesomeIcon icon={faLink} /> {domain}
                  </div>
                </div>
              </div>
            }
          })}
        </div>
      </TransitionAnimator>
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
