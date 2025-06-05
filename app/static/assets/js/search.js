const POSTS_PER_PAGE = 5;

function replaceHighlights(input, slug) {
  const result = document.createDocumentFragment();
  input.split(/{~(.*?)~}/g).forEach((part, index) => {
    if (index % 2 === 1) {
      let elem;
      if (slug) {
        const href = `/blog/p/${slug}#:~:text=${encodeURIComponent(part)}`;
        elem = document.createElement("a");
        elem.href = href;
      } else {
        elem = document.createElement("span");
      }
      elem.className = "search-highlight";
      elem.textContent = part;
      result.appendChild(elem);
    } else {
      result.appendChild(document.createTextNode(part));
    }
  });
  return result;
}

const icon = document.querySelector(".search-bar i");
const input = document.querySelector(".search-bar input");
const resultsEl = document.getElementById("search-results");
const noMorePosts = document.getElementById("no-more-posts");

var q = new URLSearchParams(window.location.search).get("q") || "";
var page = 1;
var ws;

function resultsToHTML(results) {
  return results.map((post) => t`
    <div class="card horizontal">
      <a class="image" href="/blog/p/${post.slug}"><img src="/img/blog/${post.img}" /></a>
      <div class="info">
        <div class="body">
          <div>
            <div class="tags">
              ${post.tags.map((tag) => t`
                <span class="tag tag-${tag.color}">${tag.name}</span>
              `)}
            </div>
            <span>${post.points > 0 ? `+${post.points} points` : ''}</span>
          </div>
          <h3><a href="/blog/p/${post.slug}">${post.title}</a></h3>
          <p>${post.description}</p>
        </div>
        <div class="footer text-darker">
          ${relativeTime(post.timestamp)} -
          <span class="darken">
            <i class="fa-regular fa-eye"></i>
            ${post.views} views
          </span>
        </div>
      </div>
    </div>`
  );
}

function updateResults(query) {
  if (query) {
    ws.send({
      search: {
        query
      }
    });
  } else {
    ws.send({
      allPosts: {
        page
      }
    })
  }
}

const createSocket = () => {
  icon.classList = "fa-solid fa-rotate";
  const protocol = location.protocol === "https:" ? "wss" : "ws";
  ws = new WebSocket(`${protocol}://${location.host}/blog/search_ws`);

  ws.send = new Proxy(ws.send, {
    apply: (target, thisArg, args) => {
      icon.classList = "fa-solid fa-rotate";
      args[0] = JSON.stringify(args[0]);
      return Reflect.apply(target, thisArg, args);
    }
  });

  ws.onopen = () => {
    updateResults(q);
  };

  ws.onmessage = (e) => {
    const data = JSON.parse(e.data);
    if ("searchResults" in data) {
      const results = data.searchResults.map((post) => {
        // Replace description with highlighted content
        if (post.markdown.includes("{~")) {
          post.description = "… " + post.markdown.replaceAll("...", "…") + " …";
        }
        post.title = replaceHighlights(post.title);
        post.description = replaceHighlights(post.description, post.slug);
        return post;
      });
      resultsEl.replaceChildren(...resultsToHTML(results));
    } else if ("allPosts" in data) {
      const { page: receivedPage, posts } = data.allPosts;
      if (receivedPage === 1) {
        resultsEl.replaceChildren();
      }
      resultsEl.append(...resultsToHTML(posts));
      if (posts.length < POSTS_PER_PAGE) {
        noMorePosts.classList.remove("hidden");
      } else {
        noMorePosts.classList.add("hidden");
      }
    }
    icon.classList = "fa-solid fa-check";
  };

  ws.onclose = (e) => {
    console.error("Socket closed unexpectedly:", e.reason);
    ws = null;
    icon.classList = "fa-solid fa-xmark";
    setTimeout(createSocket, 2000);
  };
  ws.onerror = (e) => {
    console.error("Socket error:", e);
    ws.close();
  };
};
createSocket();

input.value = q;
input.addEventListener("input", (e) => {
  q = e.target.value.trim();
  if (ws) {
    history.replaceState({}, "", `?q=${encodeURIComponent(q)}`);
    updateResults(q);
  }
});

const observer = new IntersectionObserver((e) => {
  e.forEach((entry) => {
    // If scroll all the way down, and we are done with loading, and there are more posts: load more.
    if (entry.isIntersecting && icon.classList.contains("fa-check") && !q && noMorePosts.classList.contains("hidden")) {
      page++;
      updateResults(q);
    }
  });
}, {
  root: document.querySelector("#scrollArea"),
  rootMargin: "0px",
  threshold: 1.0,
});
observer.observe(document.getElementById("infinite-scroll-end"));
