let firstScrolled = false;

const fun = () => {
  if (isOnScreen(document.querySelector(".skill1"))) {
    if (!firstScrolled) {
      ["100%", "60%", "70%", "59%"].forEach((width, i) =>
        document
          .querySelector(".progress-bar-" + (i + 1))
          .animate([{ width: "0" }, { width }], {
            duration: 2500 / (i + 2),
            direction: "normal",
            fill: "forwards",
          })
      );
      firstScrolled = true;
    } else {
      window.removeEventListener("scroll", fun);
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
window.addEventListener("scroll", fun);
// window.removeEventListener(
//   "DOMNodeRemoved",
//   window.removeEventListener("scroll", fun)
// );
