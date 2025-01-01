# Project #2 - Analog inputs (ADC)

Reading an analog voltage value varying between 0V and 3.3V.
The voltage measured is then assigned to a value between 0 (0V) and 4095 (3.3V) because the value has 12-bit resolution.


![alt text](image.png)

The list of parts:
- ESP32 DEVKIT board
- Potentiometer
- Breadboard
- Jumper wires


```
# arduino code

const int potPin = 4;
int potValue = 0;

void setup() {
  Serial.begin(115200);
  delay(1000);
}

void loop() {
  potValue = analogRead(potPin);
  Serial.println(potValue);
  delay(500);
}
```