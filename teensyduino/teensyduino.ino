#include <Wire.h>

// Sensors: https://www.mouser.com/datasheet/2/239/Lite-On_LTR-303ALS-01_DS_ver%201.1-1175269.pdf
// Mux:     https://www.ti.com/lit/ds/symlink/tca9548a.pdf?ts=1693960886543
// LED:     https://www.lumissil.com/assets/pdf/core/IS31FL3726A_DS.pdf

// TODOs
// Look at LED pins, find if they have to be pulled high or low to turn LED on 
// Figure out how the MUX works to communicate with the 16 different photodiodes
// Setup a command flow for the application to command the payload

#define LTR303_ADDRESS 0x29
#define LTR303_CONTROL_REG 0x80
#define LTR303_RATE_REG 0x85
#define LTR303_CH1_REG 0x88

void setup() {
  // Turn LED connected to pin 13 on Teensy
  pinMode(13, OUTPUT);
  digitalWrite(13, HIGH);


  // Begin Serial connection with baudrate 9600
  Serial.begin(9600);

  Wire.begin();
  setup_sensor(LTR303_ADDRESS);
}


void loop(){
  uint16_t reading_1, reading_0;
  read_channels(LTR303_ADDRESS, reading_1, reading_0);
 
  Serial.print("Reading 1 (IR):           ");
  Serial.println(reading_1);


  Serial.print("Reading 0 (Visible + IR): ");
  Serial.println(reading_0);
 
  // Wait for a second
  delay(1000);
}

void setup_sensor(uint8_t address) {
  // Set register address 0x80 to 0xD1
  // This will set the gain of the sensor to x96
  Wire.beginTransmission(address);
  Wire.write(LTR303_CONTROL_REG);
  Wire.write(0x1D);
  Wire.endTransmission();

  // Set register address 0x85 to 0x03
  // This will set the measurement rate to 200ms
  Wire.beginTransmission(address);
  Wire.write(LTR303_RATE_REG);
  Wire.write(0x03);
  Wire.endTransmission();
}

void read_channels(uint8_t address, uint16_t& channel_0, uint16_t& channel_1) {
  // Request to read channel 1
  Wire.beginTransmission(address);
  Wire.write(LTR303_CH1_REG);
  Wire.endTransmission();

  // Read 4 bytes and wait until you have 4 bytes to read
  Wire.requestFrom(LTR303_ADDRESS, 4);
  while(Wire.available() != 4);

  channel_1 = Wire.read();
  channel_1 |= Wire.read() << 8;

  channel_0 = Wire.read();
  channel_0 |= Wire.read() << 8;
}
