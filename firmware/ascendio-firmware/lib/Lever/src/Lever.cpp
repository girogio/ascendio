#include <Lever.h>

void Lever::setup()
{
    pinMode(RV_PIN, INPUT);
    pinMode(MASTER_SW_PIN, INPUT);
    pinMode(MOT_DAT0_PIN, OUTPUT);
    pinMode(MOT_DAT1_PIN, OUTPUT);

    pid.begin(&input, &output, &set_point, Kp, Ki, Kd);
    pid.setOutputLimits(-255, 255);
    pid.setWindUpLimits(-255, 255); // Adjust as needed
    pid.setBias(255 / 2);
    pid.start();

    stopMotor();

    input = getRaw();
    master_switch_on = digitalRead(MASTER_SW_PIN);

    for (int i = 0; i < READ_CNT; i++)
        readings[i] = 0;
}

int Lever::getRaw()
{
    return analogRead(RV_PIN);
}

int Lever::readValue()

{
    total -= readings[readIndex];

    readings[readIndex] = analogRead(RV_PIN);

    total += readings[readIndex];

    ++readIndex %= READ_CNT; // inc and wrap around :)

    average = total / READ_CNT;

    return average;
}

int Lever::getNearestDetent(int position)
{
    int nearestDetent = 0;
    int minDistance = abs(position - DETENT_POSITIONS[0]);

    for (int i = 1; i < NUM_DETENTS; i++)
    {
        int distance = abs(position - DETENT_POSITIONS[i]);
        if (distance < minDistance)
        {
            minDistance = distance;
            nearestDetent = i;
        }
    }

    return nearestDetent;
}

void Lever::controlMotor(int speed)
{
    if (speed > 0)
    {
        analogWrite(MOT_DAT0_PIN, 0);
        analogWrite(MOT_DAT1_PIN, speed);
    }
    else
    {
        analogWrite(MOT_DAT0_PIN, -speed);
        analogWrite(MOT_DAT1_PIN, 0);
    }
}

void Lever::stopMotor()
{
    analogWrite(MOT_DAT0_PIN, 0);
    analogWrite(MOT_DAT1_PIN, 0);
}

void Lever::handleDetents()
{
    input = getRaw();

    int nearestDetent = getNearestDetent(input);

    set_point = DETENT_POSITIONS[nearestDetent];

    pid.compute();

    // Apply deadband
    if (abs(input - set_point) < DEADBAND || abs(input - set_point) + DEADBAND > 100)
    {
        output = 0;
    }

    controlMotor(output);
}
