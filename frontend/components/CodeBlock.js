import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useState } from "react";
import hljs from 'highlight.js';
import { Tooltip } from "react-tooltip";

export default function CodeBlock({ lang, code }) {
    const highlighted = hljs.highlight(lang, code).value;

    const [showTooltip, setShowTooltip] = useState(false);
    const id = "copy-tooltip-" + Math.random().toString(36).substring(7);

    return <div className="code-block">
        <p>
            {lang}
            <a className="copy" data-tooltip-id={id} onClick={() => {
                navigator.clipboard.writeText(code);
                setShowTooltip(true);
                setTimeout(() => setShowTooltip(false), 2000);
            }}><FontAwesomeIcon icon={faCopy} /></a>
            <Tooltip suppressHydrationWarning delayShow={100} className={`copy-tooltip ${showTooltip ? '' : 'hide'}`} id={id} place="top" effect="solid" isOpen={true}>Copied!</Tooltip>
        </p>
        <pre><code className={`hljs language-${lang}`} dangerouslySetInnerHTML={{ __html: highlighted }}></code></pre>
    </div>
}