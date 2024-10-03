const amazonLogo = document.getElementById("amazon-logo");
const mediaDark = window.matchMedia('(prefers-color-scheme: dark)');
const hCaptchaDiv = document.querySelector("div.h-captcha");


function changeImages(darkMode) {
  //hCaptchaDiv.classes  
  amazonLogo.src = darkMode ? "../images/amazon-dark.png" : "../images/Amazon_icon.svg";
}

/** Check if user is in dark mode and select the according image depending on it. */
function setImagesDarkMode() {
  changeImages(mediaDark.matches);
}
mediaDark.addEventListener('change', event => {
  changeImages(event.matches);
});
window.addEventListener("load", setImagesDarkMode, { once: true });

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

