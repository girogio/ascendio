#include "pins.h"
#include "lever.h"
#include "commands.h"

const int BAUD_RATE = 115200;

CommandHandler commandHandler(BAUD_RATE);
Lever ENG1(ENG1_MASTER_SW_PIN, RV_ENG1_PIN, DAT0_ENG1_PIN, DAT1_ENG1_PIN);
Lever ENG2(ENG2_MASTER_SW_PIN, RV_ENG2_PIN, DAT0_ENG2_PIN, DAT1_ENG2_PIN);
// Lever ENG2(RV_ENG2_PIN, DAT0_ENG2_PIN, DAT1_ENG2_PIN, M_SPD_ENG2_PIN);
// Lever FLP(RV_FLP_PIN, DAT0_FLP_PIN, DAT1_FLP_PIN, M_SPD_FLP_PIN);
// Lever BRK(RV_BRK_PIN, DAT0_BRK_PIN, DAT1_BRK_PIN, M_SPD_BRK_PIN);



int prevX;
int prevY;

void setup() {
  Serial.begin(BAUD_RATE);

  Joystick.useManualSend(true);
  
  prevX = ENG1.getRaw(); 
  prevY = ENG2.getRaw();

  analogReference(EXTERNAL);

  commandHandler.setup();

  ENG1.setup();
  ENG2.setup();
  // FLP.setup();
  // BRK.setup();
}

void loop() {

  commandHandler.handle();

  int rawX = ENG1.getRaw();
  int newX = rawX >> 2;  

  int rawY = ENG2.getRaw();
  int newY = rawY >> 2;
  
  Joystick.X(newX);
  Joystick.Y(newY);

  if (abs(newX - prevX) >= 2 || abs(newY - prevY) >= 2) {
    prevX = newX;
    prevY = newY;
    Joystick.send_now();
  }

  ENG1.handleDetents();
  ENG2.handleDetents();

  delay(10);
}
