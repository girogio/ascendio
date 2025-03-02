#pragma once

#include "ArduPID.h"

const int NUM_DETENTS = 4;
const int READ_CNT = 10;

class Lever
{

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
    const int DETENT_POSITIONS[NUM_DETENTS] = {0, 700, 900, 1023};
    const int DEADBAND = 10;

    // PID Variables
    double set_point, input, output;
    const double Kp = .2, Ki = 0.005, Kd = 5;

public:
    ArduPID pid;
    bool master_switch_on;

    Lever(int master_sw_pin, int rv_pin, int mot_dat0_pin, int mot_dat1_pin)
    {
        RV_PIN = rv_pin;
        MASTER_SW_PIN = master_sw_pin;
        MOT_DAT0_PIN = mot_dat0_pin;
        MOT_DAT1_PIN = mot_dat1_pin;
    }

    void setup();
    int getRaw();
    int readValue();
    int getNearestDetent(int position);
    void controlMotor(int speed);
    void stopMotor();
    void handleDetents();
};
