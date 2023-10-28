# RF Calc Utility

This is a command-line utility that provides various RF (radio frequency) calculations, such as:

- Transmitter Power mW<->dBm Conversion
- Free Space Path Loss Calculation
- RF Link Range Calculation
- Times Further Calculation

## Installation

- Install Rust and Cargo.
- Clone this repository and navigate to its directory.
- Build and install the program with `cargo install --path .`.
- `rf_calc` should be accessible if you have the `cargo` `bin` in `$PATH`.

## Commands and Their Usage

### Power Conversion (`power_conversion`)

Converts transmitter power between mW and dBm.

```
rf_calc power_conversion [VALUE]
```

- `[VALUE]`: The value to convert, followed by its unit (mW or dBm).

#### Formula

Conversion between mW and dBm:

```
P(dBm) = 10 * log10(P(mW))
P(mW) = 10^(P(dBm)/10)
```

### Free Space Path Loss (`path_loss`)

Calculates the free space path loss given frequency and distance.

```
rf_calc path_loss [FREQUENCY] [DISTANCE]
```

- `[FREQUENCY]`: Frequency in MHz.
- `[DISTANCE]`: Distance in meters.

#### Formula

Free Space Path Loss (FSPL) is given by:

```
FSPL(dB) = 20*log10(d) + 20*log10(f) + 20*log10(c/(4*pi))
```

Where:
- \( d \) is the distance.
- \( f \) is the frequency.
- \( c \) is the speed of light.

### RF Link Range (`link_range`)

Calculates the RF link range given transmitter power, receiver sensitivity, and frequency.

```
rf_calc link_range [TRANSMITTER_POWER] [RECEIVER_SENSITIVITY] [FREQUENCY]
```

- `[TRANSMITTER_POWER]`: Transmitter power in dBm.
- `[RECEIVER_SENSITIVITY]`: Receiver sensitivity in dBm.
- `[FREQUENCY]`: Frequency in MHz.

#### Formula

The formula to calculate link range is derived from the Friis transmission equation:

```
d = (lambda/(4*pi)) * 10^((Pr - Pt)/20)
```

Where:
- \( d \) is the distance (or range).
- \( lambda \) is the wavelength.
- \( Pr \) is the receiver power.
- \( Pt \) is the transmitter power.

### Times Further (`times_further`)

Calculates how many times further a new distance is compared to a current distance.

```
rf_calc times_further [CURRENT_DISTANCE] [NEW_DISTANCE]
```

- `[CURRENT_DISTANCE]`: Current distance in meters.
- `[NEW_DISTANCE]`: New distance in meters.

#### Formula

Times Further is given by:

```
Times Further = New Distance / Current Distance
```

## License

This project is open-source and available under the MIT License.
