import timeAgo from '@/utils/timeAgo'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faEye } from '@fortawesome/free-regular-svg-icons'
import Link from 'next/link'

export default function PostItem({ slug, title, description, img, points, views, timestamp }) {
    const href = slug ? `/blog/p/${slug}` : "#"
    timestamp = timestamp || Date.now()

    return <div className="card card-horizontal">
        <div className="row no-gutters">
            <div className="col-sm-3">
                <Link href={href}>
                    {/* eslint-disable @next/next/no-img-element */}
                    <img src={`/img/blog/${img || '../placeholder.png'}`} className="card-img-top h-100" alt="Post thumbnail" />
                </Link>
            </div>
            <div className="col-sm-9">
                <div className="card-body">
                    <p className="card-text tags">
                        {points ? `+${points}` : ''}
                    </p>
                    <h3 className="card-title">
                        <Link href={href}><code>{title}</code></Link>
                    </h3>
                    <p className="card-text">{description}</p>
                </div>
                <div className="card-footer text-muted">
                    {timeAgo(timestamp)} - <span className="darken"><FontAwesomeIcon icon={faEye} /> {views} views</span>
                </div>
            </div>
        </div>
    </div>
}