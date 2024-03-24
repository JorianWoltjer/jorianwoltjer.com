import { RelativeTime, Tags } from '@/components'
import { timeDifference } from '@/utils/strings'
import { faEye } from '@fortawesome/free-regular-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import Image from 'next/image'
import Link from 'next/link'

export default function PostItem({ slug, title, description, img, points, views, timestamp, hidden, autorelease, tags, signature }) {
    let href;
    if (slug) {
        if (signature) {  // Hidden
            href = `/blog/h/${slug}?s=${signature}`
        } else {  // Public
            href = `/blog/p/${slug}`
        }
    } else {  // Preview
        href = "#"
    }

    return <div className="card card-horizontal">
        <div className="row no-gutters">
            <div className="col-sm-3">
                <Link href={href}>
                    <div className="card-img-top h-100">
                        <Image fill src={`http://nginx/img/blog/${img || '../placeholder.png'}`} className="card-img-top h-100" alt="Post thumbnail" />
                    </div>
                </Link>
            </div>
            <div className="col-sm-9">
                <div className="card-body">
                    <Tags tags={tags} points={points} />
                    <h3 className="card-title">
                        <Link href={href}><code>{title}</code></Link>
                    </h3>
                    <p className="card-text">{description}</p>
                </div>
                <div className="card-footer text-muted">
                    <RelativeTime timestamp={timestamp} /> - <span className="darken" title={autorelease && `Auto-Release: ${timeDifference(autorelease)}`}>
                        <FontAwesomeIcon icon={faEye} /> {hidden ? <b>Hidden</b> : `${views || 0} views`}</span>
                </div>
            </div>
        </div>
    </div>
}