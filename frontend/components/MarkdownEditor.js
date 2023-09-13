import Editor, { loader } from '@monaco-editor/react';
import Loading from './Loading';

loader.config({
  paths: {
    vs: '/assets/monaco-editor/vs',
  },
});

export default function MarkdownEditor({ markdown, onChange }) {
  return <Editor height="30ch" theme='vs-dark' defaultLanguage="markdown" defaultValue={markdown}
    loading=<Loading /> onChange={onChange} wrapperProps={{ className: "editor" }} />
}
