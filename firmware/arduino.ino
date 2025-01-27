#define pot_pin A0
#define read_cnt 50

bool handshakeDone = false;

enum Commands
{
    INIT = 0x4C,
    ACK = 0xCD,
    START = 0x01
};

int readings[read_cnt]; // the readings from the analog input
int readIndex = 0;      // the index of the current reading
int total = 0;          // the running total
int average = 0;        // the average

void setup()
{
    Serial.begin(9600);

    for (int i = 0; i < read_cnt; i++)
    {
        readings[i] = 0;
    }
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

        total = total - readings[readIndex];

        // read from the sensor:
        readings[readIndex] = analogRead(pot_pin);

        // add the reading to the total:
        total = total + readings[readIndex];

        // advance to the next position in the array:
        readIndex++;

        // if we're at the end of the array...
        if (readIndex >= read_cnt)
        {
            // ...wrap around to the beginning:
            readIndex = 0;
        }

        // calculate the average:
        average = total / read_cnt;

        byte low = average & 0xFF;
        byte high = (average >> 8) & 0xFF;

        Serial.write(low);
        Serial.write(high);

        delay(1);
    }
}
