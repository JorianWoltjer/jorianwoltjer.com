import Editor from '@monaco-editor/react';
import Loading from './Loading';

export default function MarkdownEditor({ markdown, editorRef, onChange }) {
  // const handleMount = (editor, monaco) => {
  //   editorRef.current = editor;
  // } // onMount={handleMount}

  return <Editor height="30ch" theme='vs-dark' defaultLanguage="markdown" defaultValue={markdown}
    loading=<Loading /> onChange={onChange} wrapperProps={{ className: "editor" }} />
}
