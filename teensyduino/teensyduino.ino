#include <Wire.h>

// Make sure these are correct
#define DATA   17
#define CLK    16
#define LATCH  15
#define ENABLE 14

#define MUX_ADDR_1    0x70
#define MUX_ADDR_2    0x77

#define LTR303_ADDR                  0x29
#define LTR303_ALS_CONTR_REG         0x80
#define LTR303_ALS_MEAS_RATE_REG     0x85
#define LTR303_ALS_DATA_CH1_REG      0x88

#define ALS_GAIN  96.0
#define ALS_INT    1.0

uint8_t current_channel = 0;
uint16_t sensor_data = 0;

// Initialize a 64-bit array to keep track of the state of each bit
uint8_t bitArray[8] = {0, 0, 0, 0, 0, 0, 0, 0};

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

    set_channel(MUX_ADDR_1, current_channel);
    initialize_sensor();
    set_channel(MUX_ADDR_1, 0);

    delay(250);
    
    set_channel(MUX_ADDR_2, current_channel);
    initialize_sensor();
    set_channel(MUX_ADDR_2, 0);
    
    delay(250);
  }

  // Setup LEDs
  pinMode(LATCH, OUTPUT);
  pinMode(CLK, OUTPUT);
  pinMode(DATA, OUTPUT);
  pinMode(ENABLE, OUTPUT);
}

void loop(){

  for(uint8_t i = 0; i < 8; i ++) {
    current_channel = pow(2, i);

    // FROM MUX 1
    set_channel(MUX_ADDR_1, current_channel);
    
    sensor_data = (uint16_t) read_sensor();
    
    uint8_t low_byte = sensor_data & 0xFF;
    uint8_t high_byte = (sensor_data >> 8) & 0xFF;

    Serial.write(i);
    Serial.write(low_byte);
    Serial.write(high_byte);
    
    set_channel(MUX_ADDR_1, 0);
   
    set_channel(MUX_ADDR_2, current_channel);
    
    sensor_data = (uint16_t) read_sensor();
    
    low_byte = sensor_data & 0xFF;
    high_byte = (sensor_data >> 8) & 0xFF;

    Serial.write(0b10000000 | i);
    Serial.write(low_byte);
    Serial.write(high_byte);
    
    set_channel(MUX_ADDR_2, 0);
  }
  
  while (Serial.available() > 0) {
    uint8_t cmd = Serial.read();
    
    if(cmd == 0b11111111) {
      // Prepare LEDs
      turn_leds_off();
      cmd = Serial.read();
      analogWrite(ENABLE, cmd);
    } else {
      int state = (cmd & 0b10000000) ? HIGH : LOW;
      int bitIndex = cmd & 0b01111111;
      
      setBit(bitIndex, state);
    }
  }

  delay(100);
}

void turn_leds_off() {
  digitalWrite(LATCH, LOW);

  for(int i = 0; i < 8; i ++) {
    bitArray[i] = 0.0;
  }
  
  shiftOut(DATA, CLK, MSBFIRST, 0);
  shiftOut(DATA, CLK, MSBFIRST, 0);

  shiftOut(DATA, CLK, MSBFIRST, 0);
  shiftOut(DATA, CLK, MSBFIRST, 0);

  shiftOut(DATA, CLK, MSBFIRST, 0);
  shiftOut(DATA, CLK, MSBFIRST, 0);

  shiftOut(DATA, CLK, MSBFIRST, 0);
  shiftOut(DATA, CLK, MSBFIRST, 0);
  
  digitalWrite(LATCH, HIGH);
}

void setBit(int bitIndex, int state) {
  if (bitIndex < 0 || bitIndex >= 64) {
    return; // bit index out of bounds
  }

  // Determine which byte and bit within that byte the target bit resides
  int byteIndex = bitIndex / 8;
  int bitPosition = bitIndex % 8;

  // Set or clear the specific bit
  if (state == HIGH) {
    bitSet(bitArray[byteIndex], bitPosition);
  } else {
    bitClear(bitArray[byteIndex], bitPosition);
  }

  // Update the shift register outputs
  updateShiftRegisters();
}

void updateShiftRegisters() {
  digitalWrite(LATCH, LOW);

  // Push each byte to the shift registers, starting from the last one
  for (int i = 0; i < 8; i++) {
    shiftOut(DATA, CLK, MSBFIRST, bitArray[i]);
  }

  digitalWrite(LATCH, HIGH); // Latch the data
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

float read_sensor() {
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

  float ch0 = (float) reading_0;
  float ch1 = (float) reading_1;
 
  float ratio = ch1/(ch0+ch1);
  if (ratio < 0.45) {
    return (1.7743*ch0 + 1.1059*ch1)/ALS_GAIN/ALS_INT;
  } else if (ratio < 0.64 && ratio >= 0.45) {
    return (4.2785*ch0 - 1.9548*ch1)/ALS_GAIN/ALS_INT;
  } else if (ratio < 0.85 && ratio >= 0.64) {
    return (0.5926*ch0 + 0.1185*ch1)/ALS_GAIN/ALS_INT;
  } else {
    return 0.0;
  }
}
