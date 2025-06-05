const form = document.getElementById("form");

// Update preview
const cardPreview = document.getElementById("card-preview");
function updatePreview() {
  const link = {
    title: form.title.value,
    description: form.description.value,
    img: form.img.value,
    url: form.url.value,
  }
  let domain = "";
  try {
    domain = new URL(link.url).hostname;
  } catch (e) { }

  cardPreview.replaceChildren(t`
    <div class="card horizontal">
      <a class="image" href="#"><img src="/img/blog/${link.img}" /></a>
      <div class="info">
        <div class="body">
          <div class="tags">
            <span class="tag tag-gray">
              <i class="fa-solid fa-arrow-up-right-from-square"></i>
              External
            </span>
          </div>
          <h3><a href="#">${link.title}</a></h3>
          <p>${link.description}</p>
        </div>
        <div class="footer text-darker">
          0 seconds ago -
          <span class="darken">
            <i class="fa-solid fa-link"></i>
            ${domain}
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
form.querySelectorAll("input[name=title], textarea[name=description], input[name=img], input[name=url]").forEach((el) => {
  el.addEventListener("input", updatePreview);
});
updatePreview();

// Submit form
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  form.querySelectorAll("button").forEach(btn => btn.disabled = true);
  try {
    const link = {
      folder: Number(form.folder.value),
      url: form.url.value,
      title: form.title.value,
      description: form.description.value,
      img: form.img.value,
      featured: form.featured.checked,
    }
    if (!confirm("Are you sure you want to save?")) {
      return;
    }
    fetch(form.action, {
      method: form.getAttribute("method"),
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(link)
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
          alert("Failed to save link. Check DevTools/logging for details.");
        }
      })
  } finally {
    form.querySelectorAll("button").forEach(btn => btn.disabled = false);
  }
});
