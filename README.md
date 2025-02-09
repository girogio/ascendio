# Flight Simulation Throttle Quadrant

This project is aimed at creating a throttle quadrant for flight simulation.
Based on an Arduino Nano, and motorized slider potentiometers (like the one
found in A/V mixers) we shall emulate the different detents and mechanical
resistance of the various levers found in a real aircraft.

I plan to support the throttle behaviour of the following aircrafts:

- Airbus A320-200
- Boeing 737

## Hardware

The hardware is based on the following components:

| Quantity | Component                                                                                                      | Description                                                                                                   |
| -------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| 1        | Teensy 3.2                                                                                                     | Microcontroller board                                                                                         |
| 4        | [Motorized Slider Potentiometer](https://tech.alpsalpine.com/e/products/detail/RSA0N11M9A0K/)                  | Alps RSA0N11M9A0K                                                                                             |
| 2        | [Lever Switch](https://www.fabian.com.mt/en/products/webshop/19924/switch-toggle-miniature-spdt-on-off-on.htm) | Engine start switches                                                                                         |
| 1        | [3 position rotary switch](https://www.fabian.com.mt/en/products/webshop/9117/switch-rotary-3p-4w-bbm.htm)                                                                                       | Engine MODE selector switch |
| 4        | [TA7219SG](https://www.alldatasheet.com/datasheet-pdf/pdf/215391/TOSHIBA/TA7291SG.html)                                                                      | Dual full H-bridge motor driver for potentiometers                                                            |

> [!NOTE]
> The project is still in the design phase, and the components listed above are subject to change.


## Software

The software for this projcet is split into two parts:

- The firmware running on the Teensy 3.2
- The software running on the host computer to interface with the Arduino Nano and the flight simulator

## Dependencies

- [clang](https://rust-lang.github.io/rust-bindgen/requirements.html)
- [SimConnect SDK]()

## License

This project is licensed under the GNU General Public License v3.0. See the
[LICENSE](LICENSE) file for more information.
