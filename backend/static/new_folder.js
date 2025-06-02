const form = document.getElementById("form");

// Update preview
function cancelEvent(e) {
  e.preventDefault();
}
const cardPreview = document.getElementById("card-preview");
function trackValue(e) {
  if (e.isTrusted) {
    window.addEventListener("beforeunload", cancelEvent);
  }

  if (e.target.name === "title") {
    cardPreview.querySelector("h3 .preview-title").textContent = e.target.value;
  } else if (e.target.name === "description") {
    cardPreview.querySelector("p").textContent = e.target.value;
  } else if (e.target.name === "img") {
    cardPreview.querySelector("img").src = e.target.value;
  }
}
document.querySelectorAll("input, textarea, select").forEach((el) => {
  el.addEventListener("input", trackValue);
  el.dispatchEvent(new Event("input"));
});
form.title.addEventListener("input", (e) => {
  form.slug.value = slugify(e.target.value);
});

// Submit form
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  form.querySelectorAll("button").forEach(btn => btn.disabled = true);
  try {
    const folder = {
      parent: Number(form.folder.value) || null,
      title: form.title.value,
      slug: form.slug.value,
      description: form.description.value,
      img: form.img.value,
    }
    if (!confirm("Are you sure you want to save?")) {
      return;
    }
    fetch(form.action, {
      method: form.getAttribute("method"),
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(folder)
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
          alert("Failed to save folder. Check DevTools/logging for details.");
        }
      })
  } finally {
    form.querySelectorAll("button").forEach(btn => btn.disabled = false);
  }
});
