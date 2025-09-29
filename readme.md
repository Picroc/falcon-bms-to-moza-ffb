# Falcon BMS to Moza FFB mapper

A tool to support force feedback in Falcon BMS for Moza joysticks via translation of telemetry into format Moza's
software understands.

Since Moza only listens for telemetry when a supported game is running, the tool pretends to be DCS, hence the name of
executable.

## Instructions

Download from the [latest release](https://github.com/Picroc/falcon-bms-to-moza-ffb/releases) the app executable.

Then just start the app together with Moza Cockpit and BMS. You can select any profile, DCS Viper works okay.

## What's supported

Being a FBW aircraft, it's hard to judge if some telemetry values are ignored, incorrect or work as designed, so it's
approximated with the F-16 feel in DCS. Feel free to raise PRs with values tuning.

- Afterburner rattle
- Gear movement
- Bombs deployment effect
- Chaff/Flare deployment effect
- Speedbrake movement
- Gear touchdown bump effect

## Credits

This is based on [Bartosso's repo for FFBeast](https://github.com/Bartosso/bms-to-ffbeast)