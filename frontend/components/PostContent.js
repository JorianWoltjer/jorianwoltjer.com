import Breadcrumbs from '@/components/Breadcrumbs'
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css'
import { useEffect } from 'react';

export default function PostContent({ content, admin_interface, admin_components, hljsRef }) {
    useEffect(() => {
        if (hljsRef !== undefined) {
            hljsRef.current = hljs;
        }
        hljs.highlightAll();
    }, [hljsRef]);

    return <>
        <Breadcrumbs slug={content.slug} title={content.title} />
        {admin_interface && admin_components}
        <div className='post-content' dangerouslySetInnerHTML={{ __html: content.html }} />
    </>
}