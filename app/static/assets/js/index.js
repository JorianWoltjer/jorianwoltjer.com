// Welcome animation
const welcome = document.getElementById("welcome");
const fakeCalc = document.querySelector(".fake-calc");
const nav = document.querySelector("header nav ol");
let timeout;
let i = 0;
// OS detection
function getOS() {
  // Mobile is considered Windows, otherwise detect desktop OS
  let userAgent = navigator.userAgent;
  userAgent += (navigator.userAgentData?.platform || navigator.platform);
  userAgent = userAgent.toLowerCase();
  if (userAgent.includes("android") || userAgent.includes("iphone") || userAgent.includes("ipad") || userAgent.includes("win")) {
    return "windows";
  } else if (userAgent.includes("mac")) {
    if (navigator.maxTouchPoints > 0) {
      return "windows";
    }
    return "macos";
  } else if (userAgent.includes("linux")) {
    return "linux";
  }
  return "windows";
}
const os = getOS();
fakeCalc.classList.add(os);
if (os !== "windows") {
  fakeCalc.querySelector("#calc").src = `/img/calc/${os}.png`;
}
fakeCalc.querySelector("#x").addEventListener("click", () => {
  fakeCalc.classList.add("exiting");
  setTimeout(() => {
    fakeCalc.classList.remove("exiting");
    fakeCalc.classList.add("hidden");
  }, 300);
});
fakeCalc.querySelector("#square").addEventListener("click", () => {
  if (os === "windows") {
    location = "ms-calculator://";
  } else if (os === "macos") {
    location = "itms-apps://itunes.apple.com/app/id1069511488";
  }
});
fakeCalc.querySelector(".grab-area").addEventListener("pointerdown", (e) => {
  e.preventDefault();
  if (e.button !== 0) return; // Only left click

  const calc = fakeCalc.querySelector("#calc");
  const offsetX = e.clientX - calc.getBoundingClientRect().left;
  const offsetY = e.clientY - calc.getBoundingClientRect().top;

  function move(e) {
    fakeCalc.style.left = `${e.clientX - offsetX}px`;
    fakeCalc.style.top = `${e.clientY - offsetY}px`;
  }
  function moveTouch(e) {
    e.preventDefault();
    e.stopPropagation();
    return move(e.targetTouches[0]);
  }
  function stop() {
    e.target.removeEventListener("touchmove", moveTouch);
    document.removeEventListener("pointermove", move);
    document.removeEventListener("pointerup", stop);
  }

  e.target.addEventListener("touchmove", moveTouch);
  document.addEventListener("pointermove", move);
  document.addEventListener("pointerup", stop);
});
welcome.addEventListener("click", async () => {
  // Either do the navbar animation, or show a fake calculator
  if (Math.random() < 0.5) {
    if (!fakeCalc.classList.contains("hidden")) {
      // If already visible, re-open
      fakeCalc.classList.add("hidden");
      fakeCalc.offsetWidth;
    }
    fakeCalc.classList.remove("hidden");
  } else {
    if (timeout) clearTimeout(timeout);
    nav.classList.remove("welcome-animation");
    nav.offsetWidth;
    nav.classList.add("welcome-animation");
    timeout = setTimeout(() => {
      nav.classList.remove("welcome-animation");
    }, 1000);
  }
});

// Particles.js in background
tsParticles.load({
  id: "tsparticles",
  options: {
    "particles": {
      "number": {
        "value": 80,
        "density": {
          "enable": true,
          "value_area": 800
        }
      },
      "color": {
        "value": "#bde4ff"
      },
      "shape": {
        "type": "circle"
      },
      "opacity": {
        "value": 0.5,
        "random": false,
        "anim": {
          "enable": false,
          "speed": 1,
          "opacity_min": 0.1,
          "sync": false
        }
      },
      "size": {
        "value": 3,
        "random": true,
        "anim": {
          "enable": false,
          "speed": 40,
          "size_min": 0.1,
          "sync": false
        }
      },
      "line_linked": {
        "enable": true,
        "distance": 150,
        "color": "#3498db",
        "opacity": 0.6,
        "width": 1
      },
      "move": {
        "enable": true,
        "speed": 1,
        "direction": "none",
        "random": false,
        "straight": false,
        "out_mode": "out",
        "bounce": false,
        "attract": {
          "enable": false,
          "rotateX": 600,
          "rotateY": 1200
        }
      }
    },
    "interactivity": {
      "detect_on": "window",
      "events": {
        "onhover": {
          "enable": true,
          "mode": "grab"
        },
        "onclick": {
          "enable": true,
          "mode": "push"
        },
        "resize": true
      },
      "modes": {
        "grab": {
          "distance": 100,
          "line_linked": {
            "opacity": 0.5
          }
        },
        "bubble": {
          "distance": 150,
          "size": 10,
          "duration": 2,
          "opacity": 0.5,
          "speed": 3
        },
        "repulse": {
          "distance": 75,
          "duration": 0.4
        },
        "push": {
          "particles_nb": 3
        },
        "remove": {
          "particles_nb": 2
        }
      }
    },
    "retina_detect": true
  }
})