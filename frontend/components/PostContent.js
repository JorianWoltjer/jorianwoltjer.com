import { Breadcrumbs } from '@/components'
import timeAgo from '@/utils/timeAgo';
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
        <p class="tags">
            {content.points ? `+${content.points}` : ''}
        </p>
        <div class="text-muted">
            {timeAgo(content.timestamp)} - <span class="darken"><FontAwesomeIcon icon={faEye} /> {content.views} views</span>
        </div>
        {admin_interface && admin_components}
        <div className='post-content' dangerouslySetInnerHTML={{ __html: content.html }} />
    </>
}
