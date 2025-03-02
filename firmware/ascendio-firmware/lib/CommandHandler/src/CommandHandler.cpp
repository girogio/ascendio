#include <CommandHandler.h>

CommandHandler::CommandHandler(int baudRate)
{
    this->baudRate = baudRate;
    handshakeDone = false;
}

void CommandHandler::setup()
{
    ;
}

void CommandHandler::handle()
{
    if (Serial.available() > 0)
    {
        uint8_t command_id = Serial.read();

        CommandType command = static_cast<CommandType>(command_id);

        switch (command)
        {
        case CommandType::INIT:
            Serial.write(CommandType::INIT_ACK);
            break;

        case CommandType::HEALTH:
            Serial.write(CommandType::HEALTH_ACK);
            break;

        default:
            Serial.write(CommandType::UNKNOWN);
        }
    }
}
