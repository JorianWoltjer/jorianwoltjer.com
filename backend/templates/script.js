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

function handleMobileToc(e) {
  toc.open = !e.matches;
}
mediaQuery.addEventListener("change", handleMobileToc);
handleMobileToc(mediaQuery);

// Set current year in footer
document.getElementById("year").textContent = new Date().getFullYear();
