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
    cardPreview.querySelector("h3 a").textContent = e.target.value;
  } else if (e.target.name === "description") {
    cardPreview.querySelector("p").textContent = e.target.value;
  } else if (e.target.name === "img") {
    cardPreview.querySelector("img").src = e.target.value;
  } else if (e.target.name === "url") {
    const domain = new URL(e.target.value).hostname;
    cardPreview.querySelector("#domain").textContent = domain;
  }
}
document.querySelectorAll("input, textarea, select").forEach((el) => {
  el.addEventListener("input", trackValue);
  el.dispatchEvent(new Event("input"));
});

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
