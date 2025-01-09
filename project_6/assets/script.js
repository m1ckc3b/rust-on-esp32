let redIsOn = false
let greenIsOn = false


function toggleRed() {
  fetch('/red').then(res => {
    if (res.ok) {
      redIsOn = !redIsOn
      document.getElementById("redState").textContent = redIsOn ? "ON" : "OFF"
    } else {
      console.error("Erreur avec la requête")
    }
  })
}

function toggleGreen() {
  fetch('/green').then(res => {
    if (res.ok) {
      greenIsOn = !greenIsOn
      document.getElementById("greenState").textContent = greenIsOn ? "ON" : "OFF"
    } else {
      console.error("Erreur avec la requête")
    }
  })
}