const form = document.getElementById("form");

// Update preview
const cardPreview = document.getElementById("card-preview");
function updatePreview() {
  const folder = {
    title: form.title.value,
    description: form.description.value,
    img: form.img.value,
  }

  cardPreview.replaceChildren(t`
    <div class="card horizontal">
      <a class="image" href="#"><img src="/img/blog/${folder.img}" /></a>
      <div class="info">
        <div class="body">
          <h3>
            <a href="#"><i class="fa-solid fa-folder-closed"></i> ${folder.title}</a>
          </h3>
          <p>${folder.description}</p>
        </div>
        <div class="footer text-darker">
          0 seconds ago
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
form.querySelectorAll("input[name=title], textarea[name=description], input[name=img]").forEach((el) => {
  el.addEventListener("input", updatePreview);
});
updatePreview();
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
