import { Breadcrumbs, RelativeTime, Tags } from '@/components'
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css'
import { useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faEye } from '@fortawesome/free-regular-svg-icons';

export default function PostContent({ content, admin_interface, admin_components, hljsRef }) {
    useEffect(() => {  // Required for updating after loading
        if (hljsRef !== undefined) {
            hljsRef.current = hljs;
        }
        hljs.highlightAll();
    }, [hljsRef]);

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        <br />
        <Tags tags={content.tags} points={content.points} />
        <div className="text-muted">
            {<RelativeTime timestamp={content.timestamp} />} - <span className="darken"><FontAwesomeIcon icon={faEye} /> {content.views || 0} views</span>
        </div>
        {admin_interface && <div className="mb-4">{admin_components}</div>}
        <div className='post-content' dangerouslySetInnerHTML={{ __html: content.html }} />
    </>
}
