const pandasImage = document.querySelector("[alt='Pandas logo']")
const mediaDark = window.matchMedia('(prefers-color-scheme: dark)');
const rustLogo = document.querySelector("#path3")

function changeImages(condition) {
  if (condition) {

    pandasImage.src =
      "https://pandas.pydata.org/static/img/pandas_white.svg"
    rustLogo.style.fill = "whitesmoke"
  } else {

    pandasImage.src =
      "https://pandas.pydata.org/static/img/pandas.svg"
    rustLogo.style.fill = "black"
  }
}

/** Check if user is in dark mode and select the according image depending on it. */
function setImagesDarkMode() {
  changeImages(mediaDark.matches)
}
// Check if it is Safari, see https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList
if (mediaDark.addEventListener)
  mediaDark.addEventListener('change', event => {
    changeImages(event.matches)
});
else {
  mediaDark.addListener(event => {
    changeImages(event.matches)
});
  // WebP it is not supported by Safari yet.
  document.querySelector(".img-container").style.backgroundImage = "url(\"../images/background-safari.png\")"
}
window.addEventListener("load", setImagesDarkMode, { once: true })

const progressBarContainer = document.querySelectorAll('.progress-bar-container > *')
const firstAnimated = [];

const skills = [/* Python */ "100%", /* Rust */ "60%", /* Vue */ "90%", /* Pandas */ "63%"]

function animateProgressBar() {
  for (const [i, progressBarElement] of progressBarContainer.entries()) {
    if (isOnScreen(progressBarElement)) {

      const { className: progressBarClass } = progressBarElement
      if (!firstAnimated.includes(progressBarClass)) {

        progressBarElement
          .animate([{ width: "0" }, { width: skills[i] }], {
            duration: 2500 / (i + 2),
            direction: "normal",
            fill: "forwards",
          });
        firstAnimated.push(progressBarClass);

      } else {

        if (firstAnimated.length === progressBarContainer.length)
          window.removeEventListener("scroll", animateProgressBar);

      }
    }
  }
};

function isOnScreen(element) {
  const docViewTop = window.scrollY;
  const docViewBottom = docViewTop + window.innerHeight;

  const elemTop = element.offsetTop;
  const elemBottom = elemTop + element.style.height;

  return elemBottom <= docViewBottom && elemTop >= docViewTop;
}

window.addEventListener("scroll", animateProgressBar);

/**
 * @TODO
 * @param {Event} event 
 */
function contact(event) {
  event.preventDefault()
  const { 0: nameElement, 1: emailElement, 2: messageElement } = event.target
  const [{ value: name }, { value: email }, { value: message }] =
    [nameElement, emailElement, messageElement];
  console.log(name, email, message);
  return false;
}

