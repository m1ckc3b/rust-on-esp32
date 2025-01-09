function preventAllEvents() {
  document.addEventListener("click", (event) => event.preventDefault());
  document.addEventListener("submit", (event) => event.preventDefault());
}

preventAllEvents();
