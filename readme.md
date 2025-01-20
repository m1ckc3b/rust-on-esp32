# Rust on ESP32

I purchased a basic ESP32 starter kit created by Lafvin with the plan to build each of the 12 projects with Rust instead of Arduino as the documentation calls for.

Bare-programing with Arduino is easier than Rust because of the abstraction layers offers by the sdk but Rust gives us more power to handle the bare-metal with high-level programing style.

Here is details about each project:

[Project_0](./project_0/) - Hello World + Blinky

The first simple project when one begins in bare-metal programing.


[Project_1](./project_1/) - Blinky

Simple project with a pushbutton and an LED.

[Project_2](./project_2/) - Analog Inputs (ADC)

Reading an analog voltage value varying between 0V and 3.3V. The voltage measured is then assigned to a value between 0 (0V) and 4095 (3.3V) because the value has 12-bit resolution.


[Project_3](./project_3/) - PWM Analog Output

Using the PWM protocol to increase/decrease the LED brightness with a resolution of 12 bits and a frequency of 4 Khz.


[Project_4](./project_4/) - PIR Motion Sensor

When motion is detected the buzzer will sound an alarm during 500 milliseconds.


[Project_5](./project_5/) - Switch Web Server

Creating a standalone web server that controls (outputs) two Leds. The web server is mobile responsive and can be accessed with any device that as a browser on the local network.

<!-- [Project_6](./project_6/):
[Project_7](./project_7/):
[Project_8](./project_8/):
[Project_9](./project_9/):
[Project_10](./project_10/):
[Project_11](./project_11/):
[Project_12](./project_12/): -->



