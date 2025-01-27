#include "pins.h"
#include "lever.h"


enum Commands
{
    INIT = 0x4C,
    ACK = 0xCD,
    START = 0x01
};

Lever ENG1(RV_ENG1_PIN, DAT0_ENG1_PIN, DAT1_ENG1_PIN, M_SPD_ENG1_PIN);
Lever ENG2(RV_ENG2_PIN, DAT0_ENG2_PIN, DAT1_ENG2_PIN, M_SPD_ENG2_PIN);
Lever FLP(RV_FLP_PIN, DAT0_FLP_PIN, DAT1_FLP_PIN, M_SPD_FLP_PIN);
Lever BRK(RV_BRK_PIN, DAT0_BRK_PIN, DAT1_BRK_PIN, M_SPD_BRK_PIN);

bool handshakeDone = false;

void setup()
{
    Serial.begin(9600);

    ENG1.setup();
    ENG2.setup();
    FLP.setup();
    BRK.setup();
}

void loop()
{
    if (!handshakeDone)
    {
        if (Serial.available() > 0)
        {
            byte incomingByte = Serial.read();
            if (incomingByte == Commands::INIT)
            {
                handshakeDone = true;
                Serial.write(Commands::ACK);
            }
        }
    }
    else
    {
        int eng1_val = ENG1.readValue();

        byte low = eng1_val & 0xFF;
        byte high = (eng1_val >> 8) & 0xFF;

        Serial.write(low);
        Serial.write(high);

        delay(1);
    }
}
