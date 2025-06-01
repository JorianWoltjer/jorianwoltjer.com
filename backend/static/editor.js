require.config({ paths: { 'vs': new URL('/monaco-editor/min/vs', location.origin) } });

require(['vs/editor/editor.main'], function () {
  monaco.editor.create(document.getElementById("monaco-editor"), {
    value: '',
    language: 'markdown',
    theme: 'vs-dark'
  });
});

window.addEventListener("message", (e) => {
  if (e.origin !== location.origin) return;

  if (e.data.type === "get-monaco-editor-value") {
    const editor = monaco.editor.getModels()[0];
    e.source.postMessage({
      type: "monaco-editor-value",
      value: editor.getValue()
    }, location.origin);
  } else if (e.data.type === "set-monaco-editor-value") {
    const editor = monaco.editor.getModels()[0];
    editor.setValue(e.data.value);
  }
});
