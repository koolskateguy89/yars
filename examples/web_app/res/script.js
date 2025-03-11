console.log("hello");

const TOO_LOW = "Too low!";
const TOO_HIGH = "Too high!";
const GUESSED = "You guessed it!";

window.onload = (event) => {
  console.log("page is fully loaded");

  document.getElementById("myForm").addEventListener("submit", (event) => {
    event.preventDefault();

    const guess = document.getElementById("guessInput").value;

    fetch(`/guess`, {
      method: "POST",
      body: JSON.stringify({
        guess: Number(guess),
      })
    })
    .then(response => response.json())
    .then(({ message }) => {
      // TODO: show it on page
      console.log(message);
    })
    .catch(error => console.error(error));
  });
};
