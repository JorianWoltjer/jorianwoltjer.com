import { useEffect, useRef } from "react";

function html_strip(html) {
  return html.replace(/<[^>]*>?/g, '').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
}

export default function TableOfContents({ html }) {
  // Only h2 and h3 are shown, dot-prefixed numbers in title are removed
  const toc = [...html.matchAll(/<(h[2-3]) id="(.*?)">(?:\d+\.\s*)?(.*?)<\/\1>/g)];
  const ref = useRef(null);

  // Render tree with <ol> first and <ul> inside
  const renderToc = (toc) => {
    const items = [];
    let currentLevel = 2;
    let currentList = items;

    toc.forEach((match, index) => {
      const [_, tag, id, title] = match;
      const level = parseInt(tag[1]);

      while (currentLevel < level) {
        const lastItem = currentList[currentList.length - 1];
        lastItem.children = [];
        currentList = lastItem.children;
        currentLevel++;
      }

      while (currentLevel > level) {
        currentList = items;
        currentLevel--;
      }

      currentList.push({
        id,
        title: html_strip(title),
        children: []
      });
    });

    const renderList = (items) =>
      items.map((item, index) => (
        <li key={index}>
          <a href={`#${item.id}`}>{item.title}</a>
          <ul>
            {item.children.length > 0 && renderList(item.children)}
          </ul>
        </li>
      ));

    return <ol>{renderList(items)}</ol>;
  };

  useEffect(() => {
    const mediaQuery = window.matchMedia('(max-width: 768px)');
    const listener = () => {
      if (!ref.current) return;
      if (mediaQuery.matches) {
        ref.current.open = false;
      } else {
        ref.current.open = true;
      }
    };
    mediaQuery.addEventListener("change", listener);
    listener();
    return () => mediaQuery.removeEventListener("change", listener);
  }, []);

  if (toc.length === 0) {
    return <></>;
  } else {
    return (
      <details open className="table-of-contents" ref={ref}>
        <summary>Contents</summary>
        {renderToc(toc)}
      </details>
    );
  }
}