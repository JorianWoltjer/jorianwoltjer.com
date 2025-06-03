// TODO: click on image to enlarge it

// Table of Contents
const article = document.querySelector('article');
const tocElem = document.getElementById('toc');

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
    tocElem.appendChild(li);
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
article.querySelectorAll('img').forEach(img => {
  if (img.alt) {
    const p = document.createElement('p');
    p.className = 'img-alt';
    p.textContent = img.alt;
    img.insertAdjacentElement('afterend', p);
  }
});
