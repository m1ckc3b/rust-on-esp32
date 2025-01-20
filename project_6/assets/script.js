document.addEventListener("DOMContentLoaded", function() {
  const colorPicker = document.getElementById('colorPicker');
  let hexInput
  
  const colorPreview = document.getElementById('colorPreview');
  const ctx = colorPicker.getContext('2d');
  const size = 300;
  colorPicker.width = size;
  colorPicker.height = size;
  const gradient = ctx.createConicGradient(0, size / 2, size / 2);
  gradient.addColorStop(0, '#fff490');
  gradient.addColorStop(1 / 12, '#f9e380');
  gradient.addColorStop(2 / 12, '#fecc87');
  gradient.addColorStop(3 / 12, '#ffa894');
  gradient.addColorStop(4 / 12, '#f98089');
  gradient.addColorStop(5 / 12, '#f9b7c5');
  gradient.addColorStop(6 / 12, '#c193b8');
  gradient.addColorStop(7 / 12, '#a8b7e0');
  gradient.addColorStop(8 / 12, '#88d2f9');
  gradient.addColorStop(9 / 12, '#81e8e5');
  gradient.addColorStop(10 / 12, '#b7e5a5');
  gradient.addColorStop(11 / 12, '#eaefac');
  gradient.addColorStop(1, '#fff490');
  
  const radialGradient = ctx.createRadialGradient(size / 2, size / 2, 0, size / 2, size / 2, size / 2);
  radialGradient.addColorStop(0, 'rgba(255,255,255,1)');
  radialGradient.addColorStop(1, 'rgba(255,255,255,0)');
  
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, size, size);
  
  ctx.fillStyle = radialGradient;
  ctx.fillRect(0, 0, size, size);
  let cooldown = false;

  function updateColorPreview(color) {
    colorPreview.style.backgroundColor = color;
  }

  colorPicker.addEventListener('mousemove', function(event) {
    if (!cooldown) {
      const rect = colorPicker.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;
      const pixel = ctx.getImageData(x, y, 1, 1).data;
      const [r, g, b] = pixel;
      const hex = rgbToHex(r, g, b);

      updateColorPreview(hex);
    }
  });

  colorPicker.addEventListener('click', function(event) {
    if (!cooldown) {
      const rect = colorPicker.getBoundingClientRect();
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;
      const pixel = ctx.getImageData(x, y, 1, 1).data;
      const [r, g, b] = pixel;
      const hex = rgbToHex(r, g, b);
      
      hexInput = hex;
      updateColorPreview(hex);
      cooldown = true;
      setTimeout(() => {
        cooldown = false;
      }, 3000);
    }
  });

  function componentToHex(c) {
    const hex = c.toString(16);
    return hex.length == 1 ? "0" + hex : hex;
  }

  function rgbToHex(r, g, b) {
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
  }
});

function setLedColor() {
  const rgbString = colorPreview.style.backgroundColor
  console.log(colorPreview.style.backgroundColor);
  
  const match = rgbString.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);

if (match) {
  // Les trois valeurs RGB sont capturées dans les groupes
  const red = parseInt(match[1], 10);
  const green = parseInt(match[2], 10);
  const blue = parseInt(match[3], 10);

  console.log("Red:", red, "Green:", green, "Blue:", blue);
} else {
  console.log("La chaîne n'est pas au format attendu.");
}
}