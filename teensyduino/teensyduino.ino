#include <Wire.h>

#define MUX_ADDR_1    0x70
#define MUX_ADDR_2    0x72

#define LTR303_ADDR                  0x29
#define LTR303_ALS_CONTR_REG         0x80
#define LTR303_ALS_MEAS_RATE_REG     0x85
#define LTR303_ALS_DATA_CH1_REG      0x88

uint8_t power = 0;
uint8_t current_channel = 0;

void setup() {
  // Turn LED connected to pin 13 on Teensy to show the board is connected.
  pinMode(13, OUTPUT);
  digitalWrite(13, HIGH);

  // Setup Communication
  Serial.begin(115200);
  Wire.begin();
  
  // Initialize all sensors.
  for(int i = 0; i < 8; i ++) {
    current_channel = pow(2, i);

    Serial.print("Setting up: 0x");
    Serial.print(current_channel, HEX);
    Serial.print(" from ");
    Serial.println(MUX_ADDR_1, HEX);

    set_channel(MUX_ADDR_1, current_channel);
    initialize_sensor();
    set_channel(MUX_ADDR_1, 0);

    Serial.print("Setting up: 0x");
    Serial.print(current_channel, HEX);
    Serial.print(" from ");
    Serial.println(MUX_ADDR_2, HEX);

    set_channel(MUX_ADDR_2, current_channel);
    initialize_sensor();
    set_channel(MUX_ADDR_2, 0);
    
    delay(1000);
  }

  current_channel = 0;
}

void loop(){
  power = (power + 1) % 8;
  current_channel = pow(2, power);
  
  Serial.print("Reading: 0x");
  Serial.print(current_channel, HEX);
  Serial.print(" from ");
  Serial.println(MUX_ADDR_1, HEX);

  set_channel(MUX_ADDR_1, current_channel);
  read_sensor();
  set_channel(MUX_ADDR_1, 0);

  Serial.print("Reading: 0x");
  Serial.print(current_channel, HEX);
  Serial.print(" from ");
  Serial.println(MUX_ADDR_2, HEX);

  set_channel(MUX_ADDR_2, current_channel);
  read_sensor();
  set_channel(MUX_ADDR_2, 0);

  // Wait for a second
  delay(1000);
}


void set_channel(uint8_t addr, uint8_t channel) {
  Wire.beginTransmission(addr);
  Wire.write(channel);
  Wire.endTransmission();
}

void initialize_sensor() {
  // This will set the gain of the sensor to x96
  Wire.beginTransmission(LTR303_ADDR);
  Wire.write(LTR303_ALS_CONTR_REG);
  Wire.write(0x1D);
  Wire.endTransmission();

  // This will set the integration rate to 50ms and measurement rate to 100ms
  Wire.beginTransmission(LTR303_ADDR);
  Wire.write(LTR303_ALS_MEAS_RATE_REG);
  Wire.write(0x09);
  Wire.endTransmission();
}

void read_sensor() {
  // Set to register to data
  Wire.beginTransmission(LTR303_ADDR);
  Wire.write(LTR303_ALS_DATA_CH1_REG);
  Wire.endTransmission();

  // Read 4 bytes and wait until you have 4 bytes to read
  Wire.requestFrom(LTR303_ADDR, 4);
  while(Wire.available() != 4);

  // Read and print channel 1 and 0
  uint16_t reading_1 = Wire.read();
  reading_1 |= Wire.read() << 8;

  uint16_t reading_0 = Wire.read();
  reading_0 |= Wire.read() << 8;
 
  Serial.print("Reading 1 (IR):           ");
  Serial.println(reading_1);

  Serial.print("Reading 0 (Visible + IR): ");
  Serial.println(reading_0);
}
