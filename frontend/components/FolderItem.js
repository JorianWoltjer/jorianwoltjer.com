import { RelativeTime } from '@/components'
import { faFolderClosed } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import Image from 'next/image'
import Link from 'next/link'

export default function FolderItem({ title, description, img, slug, timestamp }) {
  const href = slug ? `/blog/f/${slug}` : "#"

  return <div className="card card-horizontal">
    <div className="row no-gutters">
      <div className="col-sm-3">
        <Link href={href}>
          <div className="card-img-top h-100">
            <Image fill src={`http://nginx/img/blog/${img || '../placeholder.png'}`} alt="Folder thumbnail" />
          </div>
        </Link>
      </div>
      <div className="col-sm-9">
        <div className="card-body">
          <h3 className="card-title">
            <Link href={href}>
              <FontAwesomeIcon icon={faFolderClosed} className="text-icon" />
              {title}
            </Link>
          </h3>
          <p className="card-text">{description}</p>
        </div>
        <div className="card-footer text-muted"><RelativeTime timestamp={timestamp} /></div>
      </div>
    </div>
  </div>
}