#pragma once

#include <Arduino.h>

#include <Button.h>

class Switch
{
private:
    bool state;
    int pin;

public:
    Switch(int pin)
    {
        this->pin = pin;
        pinMode(pin, INPUT_PULLUP);
        state = digitalRead(pin);
    }

    bool getState()
    {
        return state;
    }

    void setState(bool state)
    {
        this->state = state;
    }

    void onPress()
    {
        state = true;
    }
};
