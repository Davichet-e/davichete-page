const pandasImage = document.querySelector("[alt='Pandas logo']");
const mediaDark = window.matchMedia('(prefers-color-scheme: dark)');
const hCaptchaDiv = document.querySelector("div.h-captcha");


function changeImages(darkMode) {
  //hCaptchaDiv.classes  
  pandasImage.src =
    `https://pandas.pydata.org/static/img/pandas${darkMode ? "_white" : ""}.svg`;
}

/** Check if user is in dark mode and select the according image depending on it. */
function setImagesDarkMode() {
  changeImages(mediaDark.matches);
}
// Check if it is Safari, see https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList
if (mediaDark.addEventListener) {
  mediaDark.addEventListener('change', event => {
    changeImages(event.matches);
  });
} else {
  mediaDark.addListener(event => {
    changeImages(event.matches);
  });
  // WebP it is not supported by Safari yet.
  document.querySelector(".img-container").style.backgroundImage = "url(\"../images/background-safari.png\")";
}
window.addEventListener("load", setImagesDarkMode, { once: true });

const progressBarContainer = document.querySelectorAll('.progress-bar-container > *');
const firstAnimated = [];

const skills = [/* Python */ "100%", /* Rust */ "60%", /* Vue */ "90%", /* Pandas */ "63%"];

function animateProgressBar() {
  for (const [i, progressBarElement] of progressBarContainer.entries()) {
    if (isOnScreen(progressBarElement)) {

      const { className: progressBarClass } = progressBarElement;
      if (!firstAnimated.includes(progressBarClass)) {
        progressBarElement
          .animate([{ width: "0" }, { width: skills[i] }], {
            duration: 2500 / (i + 2),
            direction: "normal",
            fill: "forwards",
          });
        firstAnimated.push(progressBarClass);

      } else {
        if (firstAnimated.length === progressBarContainer.length) {
          window.removeEventListener("scroll", animateProgressBar);
        }
      }
    }
  }
}

function isOnScreen(element) {
  const docViewTop = window.scrollY;
  const docViewBottom = docViewTop + window.innerHeight;

  const elemTop = element.offsetTop;
  const elemBottom = elemTop + element.style.height;

  return elemBottom <= docViewBottom && elemTop >= docViewTop;
}

window.addEventListener("scroll", animateProgressBar);
document.querySelector("#contact").addEventListener("reset", contact);

let errorActive = false;
let successActive = false;

/**
 * This function sends an email to me.
 * @param {Event} event 
 */
async function contact(event) {
  if (!event.target.reportValidity()) {
    event.preventDefault();
    return false;
  }
  const hCaptchaResponse = document.querySelector('[name^="h-captcha-response"');
  if (hCaptchaResponse.value === "") {
    setError("Please, fill in the CAPTCHA.");
    event.preventDefault();
    return false;
  }
  const success = document.querySelector(".success");
  let { 0: nameElement, 1: emailElement, 2: messageElement } = event.target;

  // This checks if an extension has added any input to the form. 
  // There should be 6, 4 from the inputs to fill in and the button to submit, and 2 from the h-captcha
  if (event.target.length !== 6) {
    console.warn("An extension is adding an element to the form.");
    // This checks if we are picking the correct inputs
    if (nameElement.nodeName.toUpperCase() !== "INPUT" ||
        nameElement.id !== "name") {
      nameElement = document.querySelector("#name");
    }
    if (nameElement.nodeName.toUpperCase() !== "INPUT" ||
        nameElement.id !== "email") {
      emailElement = document.querySelector("#email");
    }
    if (messageElement.nodeName.toUpperCase() !== "TEXTAREA" ||
        messageElement.id !== "message") {
      messageElement = document.querySelector("#message");
    }
  }
  const [{ value: name }, { value: email }, { value: message }] =
    [nameElement, emailElement, messageElement];
  const wrapSpin = document.querySelector(".wrap-spin");
  const button = document.querySelector("button");

  try { 
    wrapSpin.style.display = "flex";
    button.disabled = true;
    const response = await fetch("https://davichete.me/contact",
      {
        method: "POST",
        body: JSON.stringify({ name, email, message, "h-captcha-response": hCaptchaResponse.value }),
        headers: {
          "Content-type": "application/json; charset=UTF-8"
        }
      });
    wrapSpin.style.display = "none";
    button.disabled = false;

    if (response.status === 502) {
      setError("Service not available at the moment!");
      return false;
    }
    else if (response.status !== 201) {
      setError("Error sending the message!");
      return false;
    }
    success.style.display = "inline";
    successActive = true;
    if (errorActive) {
      document.querySelector(".error").style.display = "none";
      errorActive = false;
    }
    hcaptcha.reset();
  } catch (e) {
    setError("Error sending the message!");
  }
  return false;
}

function setError(text) {
  const error = document.querySelector(".error");
  error.innerText = text;
  error.style.display = "inline";
  errorActive = true;
  if (successActive) {
    document.querySelector(".success").style.display = "none";
    successActive = false;
  }
  hcaptcha.reset();
}

