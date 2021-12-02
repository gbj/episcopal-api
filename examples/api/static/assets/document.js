// Light JS for Choice manipulation
const updateChoices = (el) => (ev) => {
  const chosenIndex = parseInt(el.value);
  // navigate within DOM to the choices
  Array.from(
    el.parentElement  // to the <nav>
      .parentElement  // to the <header>
      .parentElement  // to the <article>
      .querySelector(".choice")
      .children
  )
    .forEach((child, ii) => {
      if(ii === chosenIndex) {
        child.style.display = "block";
      } else {
        child.style.display = "none";
      }
    })
};

document.querySelectorAll(".choice-menu").forEach(el => {
  // update based on current value
  updateChoices(el)();
  // set listener to update for future values
  el.addEventListener("change", updateChoices(el))
});