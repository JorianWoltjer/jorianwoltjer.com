function html_strip(html) {
    return html.replace(/<[^>]*>?/g, '').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

export default function TableOfContents({ html }) {
    const toc = [...html.matchAll(/<h2 id="(.*?)"><a.*?>(?:\d+\.\s*)?(.*?)<\/a><\/h2>/g)];

    if (toc.length === 0) {
        return <></>;
    }

    return <div className="table-of-contents desktop-only">
        <h4>Contents</h4>
        <ol>
            {toc.map((match, index) => {
                const [_, id, title] = match;
                return <li key={index}>
                    <a href={`#${id}`}>{html_strip(title)}</a>
                </li>
            })}
        </ol>
    </div>
}