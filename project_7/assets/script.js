let redIsOn = false
let greenIsOn = false


function toggleRed() {
  fetch('/red').then(res => {
    if (res.ok) {
      const input_red = document.querySelector("#input_red").checked
      console.log(`Toggle Red: ${input_red}`);
    } else {
      console.error("Erreur avec la requête")
    }
  })
  
}

function toggleGreen() {
  fetch('/green').then(res => {
    if (res.ok) {
      const input_red = document.querySelector("#input_green").checked
      console.log(`Toggle Red: ${input_green}`);
    } else {
      console.error("Erreur avec la requête")
    }
  })
}