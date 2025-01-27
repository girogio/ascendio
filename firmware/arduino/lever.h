#ifndef LEVER_H
#define LEVER_H

#include "pins.h"

class Lever
{

private:
    int RV_PIN;
    int MOT_DAT0_PIN;
    int MOT_DAT1_PIN;
    int MOT_SPD_PIN;

    int readings[READ_CNT];
    int total = 0;
    int average = 0;
    int readIndex = 0;

public:
    Lever(int rv_pin, int mot_dat0_pin, int mot_dat1_pin, int mot_spd_pin)
    {
        RV_PIN = rv_pin;
        MOT_DAT0_PIN = mot_dat0_pin;
        MOT_DAT1_PIN = mot_dat1_pin;
        MOT_SPD_PIN = mot_spd_pin;
    }

    void setup()
    {
        pinMode(RV_PIN, INPUT);
        pinMode(MOT_DAT0_PIN, OUTPUT);
        pinMode(MOT_DAT1_PIN, OUTPUT);
        pinMode(MOT_SPD_PIN, OUTPUT);

        for (int i = 0; i < READ_CNT; i++)
            readings[i] = 0;
    }

    int readValue()
    {
        total -= readings[readIndex];

        readings[readIndex] = analogRead(RV_PIN);

        total += readings[readIndex];

        ++readIndex %= READ_CNT; // inc and wrap around :)

        average = total / READ_CNT;

        return average;
    }
};

#endif
