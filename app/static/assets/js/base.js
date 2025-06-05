// View Transition
window.addEventListener("pagereveal", async (e) => {
  if (e.viewTransition) {
    const from = navigation.activation?.from?.url;
    const to = navigation.currentEntry?.url;

    let transition;
    console.log("View Transition:", from, "->", to);
    if (from && to) {
      let fromPath = new URL(from).pathname;
      let toPath = new URL(to).pathname;
      if (fromPath.startsWith("/blog") && toPath.startsWith("/blog")) {
        // Find direction of the navigation
        toPath = toPath.replace("/blog/f", "/blog/p");
        fromPath = fromPath.replace("/blog/f", "/blog/p");
        console.log(fromPath, "->", toPath);
        if (fromPath === toPath) {
        } else if (toPath.startsWith(fromPath)) {
          transition = "right";
        } else if (fromPath.startsWith(toPath)) {
          transition = "left";
        }
      }
    }

    console.log("Transition:", transition);
    if (!transition) {
      e.viewTransition.skipTransition();
    } else {
      const main = document.getElementsByTagName("main")[0];
      main.style.viewTransitionName = transition;
      await e.viewTransition.ready;
      main.style.viewTransitionName = "";
    }
  }
});

// Set current year in footer
document.getElementById("year").textContent = new Date().getFullYear();
// Set active link in navigation
const navOl = document.querySelector("header nav ol");
const navLinks = [...navOl.getElementsByTagName("a")];
navLinks.sort((a, b) => b.href.length - a.href.length);
const active = navLinks.find(link => link.origin === location.origin &&
  (link.pathname === "/" ? location.pathname === "/" : location.pathname.startsWith(link.pathname)));
if (active) active.classList.add("active");
// Logout button functionality
const logout = document.getElementById("logout");
if (logout) {
  logout.addEventListener("click", async (e) => {
    e.preventDefault();
    if (confirm("Are you sure you want to logout?")) {
      await fetch("/logout", {
        method: "POST",
        credentials: "same-origin"
      })

      location.reload();
    }
  });
}

// Sticky scroll header
let lastScrollTop = 0;
const deltaUp = 35;
const deltaDown = 10;
let didScroll = false;
const header = document.querySelector("header");
const navbarHeight = header.offsetHeight;

window.addEventListener("scroll", () => {
  didScroll = true;
});

requestAnimationFrame(function checkScroll() {
  if (didScroll) {
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;

    // If scrolled far down enough, hide the header, but show again when scrolling up
    const delta = header.classList.contains("nav-hidden") ? deltaUp : deltaDown;
    if (Math.abs(lastScrollTop - scrollTop) >= delta) {
      if (scrollTop > lastScrollTop && scrollTop > navbarHeight) {
        header.classList.add("nav-hidden");
      } else if (scrollTop + window.innerHeight < document.documentElement.scrollHeight) {
        header.classList.remove("nav-hidden");
      }
      lastScrollTop = scrollTop;
    }
    didScroll = false;
  }
  requestAnimationFrame(checkScroll);
});

// Mobile layout changes
const mediaQuery = window.matchMedia("(max-width: 768px)");
const toc = document.querySelector("details.toc");
if (toc) {
  function handleMobileToc(e) {
    toc.open = !e.matches;
  }
  mediaQuery.addEventListener("change", handleMobileToc);
  handleMobileToc(mediaQuery);
}
document.querySelectorAll(".folder-description").forEach(descriptionEl => {
  const description = descriptionEl.textContent.trim().split(".");
  const firstSentence = document.createTextNode(description.shift() + ".");
  const remainingText = document.createElement("span")
  remainingText.className = "description-extra";
  remainingText.textContent = description.join(".");
  descriptionEl.replaceChildren(firstSentence, remainingText);
});

// Utils
function slugify(title) {
  const regex = /(<.*?>)|(&.*?;)|[^\w]+/g;
  const slug = title
    .replace(regex, '-')
    .replace(/^-+|-+$/g, '')
    .toLowerCase();
  return slug;
}

function relativeTime(timestamp) {
  const now = new Date();
  const from = new Date(timestamp);
  let duration = Math.abs(now - from);
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
  if (now > from) {
    return `${value} ${unit}${plural} ago`;
  } else {
    return `in ${Math.abs(value)} ${unit}${plural}`;
  }
}

// Intercept link post clicks for admin interface
document.querySelectorAll("a[data-admin-link-id]").forEach(link => {
  const id = link.dataset.adminLinkId;
  link.addEventListener("click", async (e) => {
    e.preventDefault();
    if (confirm("Do you want to EDIT this link? Cancel to open instead")) {
      location.href = `/blog/admin/link/${id}`;
    } else {
      window.open(link.href, "_blank");
    }
  });
});

// Shortcut to open login (middle mouse button on logo)
document.querySelector("header nav .logo").addEventListener("auxclick", (e) => {
  if (e.button === 1) {
    e.preventDefault();
    location.href = "/login";
  }
});

// Client-side HTML templating
if (typeof trustedTypes === "undefined")
  trustedTypes = { createPolicy: (n, rules) => rules };

function escapeHTML(value) {
  if (value instanceof Element) {
    return value.outerHTML;
  } else if (value instanceof Text) {
    return escapeHTML(value.textContent);
  } else if (value instanceof DocumentFragment) {
    return Array.from(value.childNodes).map(escapeHTML).join('');
  } else if (Array.isArray(value)) {
    return value.map(escapeHTML).join('');
  } else if (value instanceof NodeList) {
    return escapeHTML(Array.from(value));
  } else {
    return String(value)
      .replace(/&/g, '&amp;')
      .replace(/"/g, '&quot;').replace(/'/g, '&#39;')
      .replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }
}
function t(strings, ...values) {
  const result = strings.reduce((result, str, i) => {
    let value = values[i - 1];
    return result + escapeHTML(value) + str;
  });
  const html = trustedTypes.createPolicy("trustedHTML", {
    createHTML: (input) => input,
  }).createHTML(result);

  const dom = new DOMParser().parseFromString(html, "text/html");
  const fragment = document.createDocumentFragment();
  while (dom.body.firstChild) {
    fragment.appendChild(dom.body.firstChild);
  }
  return fragment;
}
