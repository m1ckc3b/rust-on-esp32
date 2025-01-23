let redIsOn = false
let greenIsOn = false


function toggleGreen() {
  fetch('/green').then(res => {
    if (res.ok) {
      const input_red = document.querySelector("#input_green").checked
      const led = document.querySelector(".led")

      if (input_red) {
        led.classList.remove('off')
        led.classList.add('on')
      } else {
        led.classList.remove('on')
        led.classList.add('off')
      }
    } else {
        console.error("Erreur avec la requête")
      }
  })
}