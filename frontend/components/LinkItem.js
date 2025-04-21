import { RelativeTime, Tags } from '@/components'
import { CDN } from '@/config'
import { faLink, faArrowUpRightFromSquare } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import Image from 'next/image'
import Link from 'next/link'

export default function LinkItem({ admin_interface, id, url, title, description, img, timestamp }) {
  let domain;
  try {
    domain = new URL(url).hostname.replace(/^www\./, '');
  } catch (e) { }

  const handleClick = (e) => {
    // If admin, ask to edit or visit
    e.preventDefault();
    const edit = confirm("Do you want to EDIT this link? Cancel to open instead");
    if (edit) {
      document.location.href = `/admin/link/${id}`;
    } else {
      window.open(url, "_blank");
    }
  }

  return <div className="card card-horizontal">
    <div className="row no-gutters">
      <div className="col-sm-3">
        <Link href={url} target="_blank" onClick={admin_interface ? handleClick : undefined}>
          <div className="card-img-top h-100">
            <Image fill src={`${CDN}/img/blog/${img || '../placeholder.png'}`} className="card-img-top h-100" alt="Post thumbnail" />
          </div>
        </Link>
      </div>
      <div className="col-sm-9">
        <div className="card-body">
          <Tags tags={[{ name: <><FontAwesomeIcon icon={faArrowUpRightFromSquare} /> External</>, color: "gray" }]} />
          <h3 className="card-title">
            <Link href={url} target="_blank" onClick={admin_interface ? handleClick : undefined}><code>{title}</code></Link>
          </h3>
          <p className="card-text">{description}</p>
        </div>
        <div className="card-footer text-muted">
          <RelativeTime timestamp={timestamp} /> - <FontAwesomeIcon icon={faLink} /> {domain}
        </div>
      </div>
    </div>
  </div >
}