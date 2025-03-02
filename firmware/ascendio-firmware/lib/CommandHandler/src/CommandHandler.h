#pragma once

#include <Arduino.h>

enum CommandType : uint8_t
{
    UNKNOWN,
    INIT,
    INIT_ACK,
    HEALTH,
    HEALTH_ACK,
};

class CommandHandler
{

private:
    int baudRate;
    bool handshakeDone;

public:
    CommandHandler(int baudRate);

    void setup();
    void handle();
};
