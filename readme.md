# Rust on ESP32

I purchased a basic ESP32 starter kit created by Lafvin with the plan to build each of the 10 projects with Rust instead of Arduino as the documentation calls for. This kit provides an ESP32-WROOM-32 and I also bought an ESP32c3-DevKitm-1 to practice on 2 different microcontroller architecture (xtensa et RISC-V respectively).

Bare-programming with Arduino is easier than with Rust because of the abstraction layers offers by the sdk but Rust gives us more power to handle the bare-metal with high-level programing style even if it is more verbose.

For each project, I provide a breadboard schema, a list of components and the arduino code (perhaps I gona upload videos of the finish projects).

Here is the list of projects:

[Project_0](./project_0/) - Hello World + Blinky

The first simple project when one begins in bare-metal programming.

`target = ""`

[Project_1](./project_1/) - Blinky

Simple project with a pushbutton and an LED.

`target = ""`

[Project_2](./project_2/) - Analog Inputs (ADC)

Reading an analog voltage value varying between 0V and 3.3V. The voltage measured is then assigned to a value between 0 (0V) and 4095 (3.3V) because the value has 12-bit resolution.

`target = ""`


[Project_3](./project_3/) - PWM Analog Output

Using the PWM protocol to increase/decrease the LED brightness with a resolution of 12 bits and a frequency of 4 Khz.

`target = ""`


[Project_4](./project_4/) - PIR Motion Sensor

When motion is detected the buzzer will sound an alarm during 500 milliseconds.

`target = ""`


[Project_5](./project_5/) - Switch Web Server

Creating a standalone web server that controls (outputs) two Leds. The web server is mobile responsive and can be accessed with any device that as a browser on the local network.

`target = "xtensa-esp32-espidf"`

Project_6 - RGB LED Web Server

`target = "riscv32imc-esp-espidf"`

Project_7 - Relay Web Server

Project_8 - Output State Synchronization Web Server

Project_9 - DHT11 Web Server

Project_10 - OLED Display



