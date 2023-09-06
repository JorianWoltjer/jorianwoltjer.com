import { CategoryFolder, Tags, RelativeTime } from "@/components";
import { BACKEND } from "@/config";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faEye } from "@fortawesome/free-regular-svg-icons";
import Image from "next/image";
import { faMagnifyingGlass, faSquareRss } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";

export default function Blog({ root_folders, featured_posts }) {
  return (
    <>
      <h1 className="my-4">Blog</h1>
      <div className="mb-4">
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

      <h3 className="my-4">Recent posts</h3>
      <div className="row row-cols-1 row-cols-md-2 g-4">
        {featured_posts.map(post => <div key={post.id} className="col">
          <div className="card h-100">
            <a href={`/blog/p/${post.slug}`}><div className="card-img-top"><Image fill src={`/img/blog/${post.img || '../placeholder.png'}`} alt="Thumbnail" /></div></a>
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
