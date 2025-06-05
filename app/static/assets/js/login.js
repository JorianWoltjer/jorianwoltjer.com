const alertElem = document.getElementById("alert");
const button = document.querySelector("#login-form button[type=submit]");

document.getElementById("login-form").addEventListener("submit", (e) => {
  e.preventDefault();
  alertElem.textContent = "";
  button.disabled = true;
  button.textContent = "Submitting...";
  const { password } = e.target;

  fetch(e.target.action, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ password: password.value.trim() }),
  }).then(async (res) => {
    if (res.ok) {
      const back = new URLSearchParams(window.location.search).get("back");
      if (back && /^\/[a-z]/.test(back)) {
        // If there is a back URL, redirect to it
        document.location.href = back;
      } else {
        try {
          await navigation.back().finished;
        } catch (e) {
          console.error(e);
          document.location.href = "/blog";
        }
      }
    } else {
      // alertElem.innerHTML = `<div class="alert error">Invalid password</div>`;
      alertElem.textContent = "Invalid password";
    }
    button.disabled = false;
    button.textContent = "Submit";
  });
});