// Table of Contents
const article = document.querySelector('article');
const tocOl = document.getElementById('toc');
const tocWrapper = document.querySelector('details.toc');

let lastLi = null;

article.querySelectorAll('h2, h3').forEach(header => {
  const level = parseInt(header.tagName[1]);
  const id = header.id;
  const title = header.textContent.replace(/^\d+\.\s*/, '');

  const li = document.createElement('li');
  const a = document.createElement('a');
  a.href = `#${id}`;
  a.textContent = title;
  li.appendChild(a);

  if (level === 2) {
    tocOl.appendChild(li);
    lastLi = li;
  } else if (level === 3 && lastLi) {
    let sublist = lastLi.querySelector('ul');
    if (!sublist) {
      sublist = document.createElement('ul');
      lastLi.appendChild(sublist);
    }
    sublist.appendChild(li);
  }
});
if (tocOl.querySelectorAll("li").length > 0) {
  tocWrapper.classList.remove('hidden');
}

// Make headers clickable
article.querySelectorAll('h2, h3, h4, h5, h6').forEach(header => {
  if (!header.id) return;
  header.replaceChildren(t`<a href="#${header.id}">${header.childNodes}</a>`);
});

// Code blocks
article.querySelectorAll('pre:has(code)').forEach(block => {
  const language = block.querySelector("code").dataset.lang || '';
  block.replaceWith(t`
    <div class="code-block">
      <div class="code-block-header">
        <span>${language}</span>
        <i class="fa-solid fa-copy copy"></i>
      </div>
      ${block}
    </div>
  `);
});
// Copy code blocks
article.querySelectorAll('.code-block .copy').forEach(button => {
  button.addEventListener('click', () => {
    const code = button.closest('.code-block').querySelector('code');
    navigator.clipboard.writeText(code.textContent).then(() => {
      button.classList.add('fa-check');
      setTimeout(() => {
        button.classList.remove('fa-check');
      }, 1000);
    });
  });
});

// Image alternative text
article.querySelectorAll('img, video').forEach(el => {
  const alt = el.getAttribute("alt");
  if (alt) {
    const p = document.createElement('p');
    p.className = 'alt';
    p.textContent = alt;
    el.insertAdjacentElement('afterend', p);
  }
});
// Click on image to enlarge
const enlargedImage = document.getElementById('enlarged-image');
article.querySelectorAll('img').forEach(img => {
  img.addEventListener('click', (e) => {
    const img = document.createElement('img');
    img.src = e.target.src;  // Copy src instead of srcset to get full size
    img.alt = e.target.alt || '';
    enlargedImage.replaceChildren(img);
    enlargedImage.classList.add('visible');
    document.body.style.overflow = "hidden";
    function removeEnlargedImage() {
      enlargedImage.classList.remove('visible');
      enlargedImage.replaceChildren();
      document.body.style.overflow = "";
    }
    document.addEventListener('click', removeEnlargedImage, { once: true });
    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        removeEnlargedImage();
      }
    });
    e.stopPropagation();
  });
});

// Add to view counter
function addView() {
  setTimeout(() => {
    navigator.sendBeacon(`/blog/add_view/${article.dataset.id}`);
  }, 5000);
}
if (document.prerendering) {
  document.addEventListener("prerenderingchange", addView, {
    once: true,
  });
} else {
  addView();
}
