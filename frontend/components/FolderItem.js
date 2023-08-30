import timeAgo from '@/utils/timeAgo'
import Link from 'next/link'
import Image from 'next/image'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faFolderClosed } from '@fortawesome/free-solid-svg-icons'

export default function FolderItem({ title, description, img, slug, timestamp }) {
    return <div className="card card-horizontal">
        <div className="row no-gutters">
            <div className="col-sm-3">
                <Link href={`/blog/f/${slug}`}>
                    {/* eslint-disable @next/next/no-img-element */}
                    <img src={`/img/blog/${img || '../placeholder.png'}`} className="card-img-top h-100" alt="Folder thumbnail" />
                </Link>
            </div>
            <div className="col-sm-9">
                <div className="card-body">
                    <h3 className="card-title">
                        <Link href={`/blog/f/${slug}`}>
                            <FontAwesomeIcon icon={faFolderClosed} className="text-icon" />
                            {title}
                        </Link>
                    </h3>
                    <p className="card-text">{description}</p>
                </div>
                <div className="card-footer text-muted">{timeAgo(timestamp)}</div>
            </div>
        </div>
    </div>
}