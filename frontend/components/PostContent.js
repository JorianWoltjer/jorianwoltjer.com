import { Breadcrumbs, CodeBlock, RelativeTime, TableOfContents, Tags } from '@/components';
import { faEye } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import 'highlight.js/styles/github-dark.css';
import parse, { domToReact } from 'html-react-parser';
import Image from 'next/image';
import Link from 'next/link';
import { useEffect, useState } from 'react';
import Zoom from 'react-medium-image-zoom';

export function render(html, mounted) {
    const headers = {
        h2: ({ id, children }) => <h2 id={id}>{children}</h2>,
        h3: ({ id, children }) => <h3 id={id}>{children}</h3>,
        h4: ({ id, children }) => <h4 id={id}>{children}</h4>,
        h5: ({ id, children }) => <h5 id={id}>{children}</h5>,
        h6: ({ id, children }) => <h6 id={id}>{children}</h6>,
    }

    return parse(html, {
        replace: (node) => {
            if (node.type === 'tag' && node.name === 'img') {
                // Image zoom
                const src = `http://nginx/img/blog/${node.attribs.src}`;
                // Stupid hydration error doesn't like <div> inside <p>, so have to delay to the client
                return <>{mounted && <Zoom><Image className='w-unset' fill src={src} alt={node.attribs.alt} /></Zoom>}</>

            } else if (node.type === 'tag' && node.name === 'pre') {
                // Code blocks
                if (node.children[0].name === 'code') {
                    const code = node.children[0].children[0].data;
                    const lang = node.children[0].attribs.class?.split("-")[1];
                    return <CodeBlock lang={lang} code={code} />
                }

            } else if (node.type === 'tag' && node.name === 'a') {
                // Convert YouTube links to iframes
                const match = /(?:https?:\/\/(?:www\.)?youtube\.com\/watch\?v=|https?:\/\/youtu\.be\/)([A-Za-z0-9_\-]+)/.exec(node.attribs.href);
                if (node.attribs.href === node.children[0].data && match) {
                    return <iframe width="560" height="315" src={`https://www.youtube-nocookie.com/embed/${match[1]}`} title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>
                }
                // Convert other links to `target="_blank"`
                return <a href={node.attribs.href} target="_blank">{domToReact(node.children)}</a>

            } else if (node.type === 'tag' && Object.keys(headers).includes(node.name)) {
                // Add anchor links to headings
                const id = node.attribs.id;
                return headers[node.name]({ id, children: <a className="header-link" href={`#${id}`}>{domToReact(node.children)}</a> })
            }
        }
    })
}

export default function PostContent({ content, admin_interface, admin_components }) {
    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, []);

    return <>
        <div className='d-flex flex-column h-100'>
            <Breadcrumbs slug={content.slug} title={content.title} />
            <br />
            <Tags tags={content.tags} points={content.points} />
            <div className="text-muted mb-2">
                {<RelativeTime timestamp={content.timestamp} />} - <span className="darken">
                    <FontAwesomeIcon icon={faEye} /> {content.hidden ? <b>Hidden</b> : `${content.views || 0} views`}</span>
            </div>
            {admin_interface && <div className="mb-4">{admin_components}</div>}
            <div className='flex-grow-1'>
                <TableOfContents html={content.html} />
                <div className='post-content'>{render(content.html, mounted)}</div>
            </div>
            <div className="pagination">
                <div className="pagination-center">
                    <p className="text-white-50">The end! If you have any questions feel free to ask me anywhere on my <Link href="/contact" target="_blank">Contacts</Link></p>
                </div>
            </div>
        </div>
    </>
}
