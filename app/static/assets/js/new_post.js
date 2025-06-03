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
const cardPreview = document.getElementById("card-preview");
function updatePreview() {
  const post = {
    title: form.title.value,
    description: form.description.value,
    img: form.img.value,
    points: form.points.value,
    hidden: form.hidden.checked,
    tags,
  }

  cardPreview.replaceChildren(t`
    <div class="card horizontal">
      <a class="image" href="#"><img src="/img/blog/${post.img}" /></a>
      <div class="info">
        <div class="body">
          <div class="tags">
            ${post.tags.map((tag) => t`
              <span class="tag tag-${tag.color}">${tag.name}</span>
            `)}
          </div>
          <span>${post.points > 0 ? `+${post.points} points` : ''}</span>
          <h3><a href="#">${post.title}</a></h3>
          <p>${post.description}</p>
        </div>
        <div class="footer text-darker">
          0 seconds ago -
          <span class="darken">
            <i class="fa-regular fa-eye"></i>
            ${post.hidden ? t`<b>Hidden</b>` : '0 views'}
          </span>
        </div>
      </div>
    </div>
  `);
}
function cancelEvent(e) {
  e.preventDefault();
}
form.querySelectorAll("input, textarea, select").forEach((el) => {
  el.addEventListener("input", () => {
    window.addEventListener("beforeunload", cancelEvent);
  });
});
form.querySelectorAll("input[name=title], textarea[name=description], input[name=img], input[name=points], input[name=hidden]").forEach((el) => {
  el.addEventListener("input", updatePreview);
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

function updateTags() {
  console.log("Updating tags", tags);
  tagsElem.replaceChildren(
    ...tags.map(tag => t`
      <span class="tag tag-${tag.color}" data-id="${tag.id}">${tag.name}</span>
    `)
  );
  document.querySelectorAll(".tags-input .tag:not(.tag-add)").forEach(tagSpan => {
    tagSpan.addEventListener("click", function () {
      tags = tags.filter(tag => tag.id !== this.dataset.id);
      updateTags();
    });
  });
  updateTagCollections();
  updatePreview();
}
function updateTagCollections() {
  datalistElem.replaceChildren(
    ...all_tags.filter(tag => !tags.includes(tag)).map(tag => {
      const option = document.createElement("option");
      option.value = tag.name;
      return option;
    })
  );
}

tagInput.addEventListener("change", (e) => {
  const new_tag = all_tags.filter(tag => !tags.includes(tag)).find(tag => tag.name == e.target.value);
  if (new_tag) {
    tags.push(new_tag);
    updateTags();
  }
  e.target.value = "";
});
tagInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter" || e.key === ",") {
    const new_tag = all_tags.filter(tag => !tags.includes(tag)).find(tag => tag.name.toLowerCase().startsWith(e.target.value.toLowerCase()));
    e.preventDefault();
    if (new_tag) {
      tags.push(new_tag);
      updateTags();
    }
    e.target.value = "";
  }
  if (e.key === "Backspace" && e.target.value === "") {
    tags.pop();
    updateTags();
  }
});
updateTags();

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
      tags: tags.map(tag => Number(tag.id)),
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
