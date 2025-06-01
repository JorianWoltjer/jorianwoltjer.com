const form = document.getElementById("form");

// Auto-Release
const hiddenCheckbox = document.getElementById("hidden");
const autoReleaseSection = document.getElementById("auto-release");
hiddenCheckbox.addEventListener("change", function () {
  if (hiddenCheckbox.checked) {
    autoReleaseSection.classList.remove("hidden");
  } else {
    autoReleaseSection.classList.add("hidden");
  }
});

function relativeTime(timestamp) {
  const now = new Date();
  let duration = Math.abs(now - timestamp);
  const seconds = Math.floor(duration / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  let value, unit;
  if (days > 0) {
    value = days;
    unit = "day";
  } else if (hours > 0) {
    value = hours;
    unit = "hour";
  } else if (minutes > 0) {
    value = minutes;
    unit = "minute";
  } else {
    value = seconds;
    unit = "second";
  }
  let plural = value !== 1 ? "s" : "";
  if (now > timestamp) {
    return `${value} ${unit}${plural} ago`;
  } else {
    return `in ${Math.abs(value)} ${unit}${plural}`;
  }
}

const autoRelease = document.getElementById("autorelease");
const autoReleaseIn = document.getElementById("autorelease-in");
autoRelease.addEventListener("input", function () {
  autoReleaseIn.textContent = relativeTime(new Date(autoRelease.value));
});
autoRelease.dispatchEvent(new Event("input"));

// Monaco Editor iframe communication
async function getMonacoEditorValue() {
  return new Promise((resolve) => {
    document.editor.postMessage({
      type: "get-monaco-editor-value"
    }, location.origin);
    window.addEventListener("message", (e) => {
      if (e.origin !== location.origin && e.source !== document.editor) return;
      if (e.data.type === "monaco-editor-value") {
        resolve(e.data.value);
      }
    }, { once: true });
  });
}

const editorFrame = document.querySelector("iframe[name=editor]");
editorFrame.addEventListener("load", () => {
  editorFrame.contentWindow.postMessage({
    type: "set-monaco-editor-value",
    value: editorFrame.dataset.value || ''
  }, location.origin);
});

// Update preview
function cancelEvent(e) {
  e.preventDefault();
}
const postPreview = document.getElementById("post-preview");
function trackValue(e) {
  if (e.isTrusted) {
    window.addEventListener("beforeunload", cancelEvent);
  }

  if (e.target.name === "title") {
    postPreview.querySelector("h3 a").textContent = e.target.value;
  } else if (e.target.name === "description") {
    postPreview.querySelector("p").textContent = e.target.value;
  } else if (e.target.name === "img") {
    postPreview.querySelector("img").src = e.target.value;
  } else if (e.target.name === "points") {
    const points = Number(e.target.value);
    postPreview.querySelector("#preview-points").textContent = points > 0 ? `+${e.target.value} points` : "";
  } else if (e.target.name === "hidden") {
    postPreview.querySelector("#preview-views").innerHTML = e.target.checked ? `<b>Hidden</b>` : `0 views`;
  }
}

document.querySelectorAll("input, textarea, select").forEach((el) => {
  el.addEventListener("input", trackValue);
  el.dispatchEvent(new Event("input"));
});
form.title.addEventListener("input", (e) => {
  form.slug.value = slugify(e.target.value);
});

// Tag input
const tagInput = document.getElementById("tag-add");
const tagsElem = document.querySelector(".tags-input .tags");
const datalistElem = document.getElementById("all-tags");
const all_tags = Array.from(datalistElem.querySelectorAll("option")).map(el => ({
  name: el.value,
  id: el.dataset.id,
  color: el.dataset.color
}));
let tags = Array.from(tagsElem.querySelectorAll("span")).map(el => ({
  name: el.textContent,
  id: el.dataset.id,
  color: el.dataset.color
}));

function handleNewTag(new_tag) {
  if (new_tag && !tags.find(tag => tag.id == new_tag.id)) {
    tags.push(new_tag);
    const tagSpan = document.createElement("span");
    tagSpan.textContent = new_tag.name;
    tagSpan.classList = `tag tag-${new_tag.color}`;
    tagSpan.dataset.id = new_tag.id;
    tagsElem.appendChild(tagSpan);
    tagSpan.addEventListener("click", function () {
      tags = tags.filter(tag => tag.id !== new_tag.id);
      tagSpan.remove();
    });
  }
  datalistElem.innerHTML = "";
  all_tags.filter(tag => !tags.includes(tag)).forEach(tag => {
    const option = document.createElement("option");
    option.value = tag.name;
    datalistElem.appendChild(option);
  });
  postPreview.querySelector(".tags").replaceWith(
    tagsElem.cloneNode(true)
  );
}

tagInput.addEventListener("change", (e) => {
  const new_tag = all_tags.find(tag => tag.name == e.target.value);
  handleNewTag(new_tag);
  e.target.value = "";
});
tagInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter" || e.key === ",") {
    const new_tag = all_tags.find(tag => tag.name.toLowerCase().startsWith(e.target.value.toLowerCase()));
    e.preventDefault();
    handleNewTag(new_tag);
    e.target.value = "";
  }
  if (e.key === "Backspace" && e.target.value === "") {
    const lastTag = tags.pop();
    if (lastTag) {
      tagsElem.querySelector(`span[data-id="${lastTag.id}"]`).remove();
    }
    handleNewTag();
  }
});
handleNewTag();

// Submit form
let previewWindow;
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  form.querySelectorAll("button").forEach(btn => btn.disabled = true);
  try {
    const post = {
      folder: Number(form.folder.value),
      slug: form.slug.value,
      title: form.title.value,
      description: form.description.value,
      img: form.img.value,
      points: Number(form.points.value),
      featured: form.featured.checked,
      hidden: form.hidden.checked,
      auto_release: form.autoreleasecheck.checked ? new Date(form.autorelease.value) : null,
      markdown: await getMonacoEditorValue(),
      tags: Array.from(form.querySelectorAll(".tags-input .tags span")).map(el => Number(el.dataset.id)),
    };
    if (e.submitter.name === "preview") {
      if (!previewWindow || previewWindow.closed) {
        previewWindow = window.open("", "preview");
      }
      const form = document.createElement("form");
      form.method = "POST";
      form.action = "/blog/admin/preview";
      form.target = "preview";  // Open result into the preview window
      form.style.display = "none";
      const input = document.createElement("input");
      input.type = "hidden";
      input.name = "json";
      input.value = JSON.stringify(post);
      form.appendChild(input);
      document.body.appendChild(form);
      form.submit();
      form.remove();
    } else if (e.submitter.name === "save") {
      if (!confirm("Are you sure you want to save?")) {
        return;
      }
      fetch(form.action, {
        method: form.getAttribute("method"),
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(post)
      })
        .then(async response => {
          if (response.ok) {
            const { url } = await response.json();
            if (/^\/[a-z]/.test(url)) {
              window.removeEventListener("beforeunload", cancelEvent);
              location.href = url;
            }
          } else {
            console.error(await response.text());
            alert("Failed to save post. Check DevTools/logging for details.");
          }
        })
    }
  } finally {
    form.querySelectorAll("button").forEach(btn => btn.disabled = false);
  }
});
