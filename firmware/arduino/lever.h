#ifndef LEVER_H
#define LEVER_H

#include "pins.h"
#include "ArduPID.h"

const int NUM_DETENTS = 4;

class Lever {

private:
  int MASTER_SW_PIN;
  int RV_PIN;
  int MOT_DAT0_PIN;
  int MOT_DAT1_PIN;

  int readings[READ_CNT];
  int total = 0;
  int average = 0;
  int readIndex = 0;

  // Detent variables
  const int DETENT_POSITIONS[NUM_DETENTS] = { 0, 700, 900, 1023 };
  const int DEADBAND = 10;

  // PID Variables
  double set_point, input, output;
  double Kp = .2, Ki = 0.005, Kd = 5;

public:
  ArduPID pid;
  bool master_switch_on;


  Lever(int master_sw_pin, int rv_pin, int mot_dat0_pin, int mot_dat1_pin) {
    RV_PIN = rv_pin;
    MASTER_SW_PIN = master_sw_pin;
    MOT_DAT0_PIN = mot_dat0_pin;
    MOT_DAT1_PIN = mot_dat1_pin;
  }

  void setup() {
    pinMode(RV_PIN, INPUT);
    pinMode(MASTER_SW_PIN, INPUT);
    pinMode(MOT_DAT0_PIN, OUTPUT);
    pinMode(MOT_DAT1_PIN, OUTPUT);

    pid.begin(&input, &output, &set_point, Kp, Ki, Kd);
    pid.setOutputLimits(-255, 255);
    pid.setWindUpLimits(-255, 255);  // Adjust as needed
    pid.setBias(255 / 2);
    pid.start();

    stopMotor();

    input = getRaw();
    master_switch_on = digitalRead(MASTER_SW_PIN);

    for (int i = 0; i < READ_CNT; i++)
      readings[i] = 0;
  }

  int getRaw() {
    return analogRead(RV_PIN);
  }

  int readValue() {
    total -= readings[readIndex];

    readings[readIndex] = analogRead(RV_PIN);

    total += readings[readIndex];

    ++readIndex %= READ_CNT;  // inc and wrap around :)

    average = total / READ_CNT;

    return average;
  }

  int getNearestDetent(int position) {
    int nearestDetent = 0;
    int minDistance = abs(position - DETENT_POSITIONS[0]);

    for (int i = 1; i < NUM_DETENTS; i++) {
      int distance = abs(position - DETENT_POSITIONS[i]);
      if (distance < minDistance) {
        minDistance = distance;
        nearestDetent = i;
      }
    }

    return nearestDetent;
  }

  void controlMotor(int speed) {
    if (speed > 0) {
      analogWrite(MOT_DAT0_PIN, 0);
      analogWrite(MOT_DAT1_PIN, speed);
    } else {
      analogWrite(MOT_DAT0_PIN, -speed);
      analogWrite(MOT_DAT1_PIN, 0);
    }
  }

  void stopMotor() {
    analogWrite(MOT_DAT0_PIN, 0);
    analogWrite(MOT_DAT1_PIN, 0);
  }

  void handleDetents() {
    input = getRaw();

    int nearestDetent = getNearestDetent(input);

    set_point = DETENT_POSITIONS[nearestDetent];

    pid.compute();

    // Apply deadband
    if (abs(input - set_point) < DEADBAND || abs(input - set_point) + DEADBAND > 100) {
      output = 0;
    }

    controlMotor(output);
  }
};

#endif
