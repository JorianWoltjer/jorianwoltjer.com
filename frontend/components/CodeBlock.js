import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import hljs from 'highlight.js';
import { useState } from "react";
import { Tooltip } from "react-tooltip";

export default function CodeBlock({ lang, code }) {
  let highlighted;
  try {
    highlighted = (lang === undefined) ? hljs.highlightAuto(code) : hljs.highlight(code, { language: lang, ignoreIllegals: true });
  } catch (e) {
    highlighted = hljs.highlight(code, { language: "plaintext" });
  }

  const [showTooltip, setShowTooltip] = useState(false);
  const id = "copy-tooltip-" + Math.random().toString(36).substring(7);

  return <div className="code-block">
    <p className="position-relative">
      {lang || <>&nbsp;</>}
      <a suppressHydrationWarning className="copy" data-tooltip-id={id} onClick={() => {
        navigator.clipboard.writeText(code);
        setShowTooltip(true);
        setTimeout(() => setShowTooltip(false), 2000);
      }}><FontAwesomeIcon icon={faCopy} /></a>
      <Tooltip suppressHydrationWarning delayShow={100} className={`copy-tooltip ${showTooltip ? '' : 'hide'}`} id={id} place="top" effect="solid" isOpen={true}>Copied!</Tooltip>
    </p>
    <pre><code className={`hljs language-${lang}`} dangerouslySetInnerHTML={{ __html: highlighted.value }}></code></pre>
  </div>
}