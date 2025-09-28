use bms_sm::{FlightData, IntellivibeData};
use std::cmp::min;

const AIRCRAFT_NAME: &str = "F-16C_50";
const ON_GROUND_GEAR: f32 = 0.01011961698532;

pub trait ComputeData {
    fn compute_ffb_data(
        &mut self,
        flight_data_file: &FlightData,
        intellivibe_data_file: &IntellivibeData,
    );
}

pub trait FrameTelemetryString {
    fn telemetry_string(&self) -> String;
}

#[derive(Default)]
pub struct MozaFFBData {
    aircraft_name: String,
    engine_rpm_left: f32,
    engine_rpm_right: f32,
    left_gear: f32,
    nose_gear: f32,
    right_gear: f32,
    acceleration_x: f32,
    acceleration_y: f32,
    acceleration_z: f32,
    wind_x: f32,
    wind_y: f32,
    wind_z: f32,
    vector_velocity_x: f32,
    vector_velocity_y: f32,
    vector_velocity_z: f32,
    tas: f32,
    ias: f32,
    vertical_velocity_speed: f32,
    aoa: f32,
    heading: f32,
    pitch: f32,
    bank: f32,
    aos: f32,
    euler_vx: f32,
    euler_vy: f32,
    euler_vz: f32,
    canopy_pos: f32,
    flap_pos: f32,
    gear_value: f32,
    speedbrake_value: f32,
    afterburner_1: f32,
    afterburner_2: f32, // Moza expects this to be 1 for F-16
    weapon: String,
    flare: f32,
    chaff: f32,
    cannon_shells: u32,
    mach: f32,
    altitude_sea_level: f32,
    led_console: f32,
    led_instruments_result: f32,
    light_apu_ready: f32,
    light_gear_warning: f32,
    light_gear_indicator: f32,

    // LOCAL PERSIST DATA
    last_aa_missile_count: u8,
    last_ag_missile_count: u8,
    last_bomb_count: u8,
}

impl ComputeData for MozaFFBData {
    fn compute_ffb_data(
        &mut self,
        flight_data_file: &FlightData,
        intellivibe_data_file: &IntellivibeData,
    ) {
        let aircraft_name = AIRCRAFT_NAME.to_string();

        let engine_rpm_left = flight_data_file.rpm;
        let engine_rpm_right = 0.0;

        let left_gear = flight_data_file.left_gear_pos;
        let nose_gear = flight_data_file.nose_gear_pos;
        let right_gear = flight_data_file.right_gear_pos;

        let acceleration_x = 0.0;
        let acceleration_y = 0.0;
        let acceleration_z = 0.0;

        let wind_x = 0.0;
        let wind_y = 0.0;
        let wind_z = 0.0;

        let vector_velocity_x = flight_data_file.x_dot;
        let vector_velocity_y = flight_data_file.y_dot;
        let vector_velocity_z = flight_data_file.z_dot;

        let tas = flight_data_file.vt;
        let ias = flight_data_file.kias;
        let vertical_velocity_speed = flight_data_file.z_dot;

        let aoa = flight_data_file.alpha;
        let heading = flight_data_file.current_heading;
        let pitch = flight_data_file.pitch;
        let bank = flight_data_file.roll;
        let aos = 0.0;

        let euler_vx = 0.0;
        let euler_vy = 0.0;
        let euler_vz = 0.0;

        let canopy_pos = 0.0;
        let flap_pos = 0.0;

        let gear_value = if intellivibe_data_file.on_ground && flight_data_file.gear_pos > 0.0 {
            flight_data_file.gear_pos - ON_GROUND_GEAR
        } else {
            flight_data_file.gear_pos
        };
        let speedbrake_value = flight_data_file.speed_brake;

        let afterburner_2 = if flight_data_file.rpm > 95.7 {
            let result = (97.0 - flight_data_file.rpm) / 1.5;
            if result > 1.0 {
                1.0
            } else {
                result
            }
        } else {
            0.0
        };
        let afterburner_1 = 0.0;

        let aa_weapon = "AIM-120C-4.4.7.106*1";
        let ag_weapon = "Mk-82-4.5.9.31*1";
        let default_weapon = "f-16c_hts_pod-4.15.44.808*1";

        let mut weapons = vec![default_weapon];

        if self.last_aa_missile_count < intellivibe_data_file.aa_missile_fired {
            self.last_aa_missile_count = intellivibe_data_file.aa_missile_fired;
        } else {
            weapons.push(aa_weapon);
        }

        if self.last_bomb_count < intellivibe_data_file.bomb_dropped {
            self.last_bomb_count = intellivibe_data_file.bomb_dropped;
        } else {
            weapons.push(ag_weapon);
        }

        let weapon = weapons.join("~");
        let flare = flight_data_file.flare_count;
        let chaff = flight_data_file.chaff_count;

        let cannon_shells: u32 = 520 - intellivibe_data_file.bullets_fired as u32;

        let mach = flight_data_file.mach;

        let altitude_sea_level = 245.0;

        let led_instruments_result = 0.0;
        let light_apu_ready = 0.0;
        let light_gear_warning = 0.0;
        let light_gear_indicator = 0.0;

        self.aircraft_name = aircraft_name;
        self.engine_rpm_left = engine_rpm_left;
        self.engine_rpm_right = engine_rpm_right;
        self.left_gear = left_gear;
        self.nose_gear = nose_gear;
        self.right_gear = right_gear;
        self.acceleration_x = acceleration_x;
        self.acceleration_y = acceleration_y;
        self.acceleration_z = acceleration_z;
        self.wind_x = wind_x;
        self.wind_y = wind_y;
        self.wind_z = wind_z;
        self.vector_velocity_x = vector_velocity_x;
        self.vector_velocity_y = vector_velocity_y;
        self.vector_velocity_z = vector_velocity_z;
        self.tas = tas;
        self.ias = ias;
        self.vertical_velocity_speed = vertical_velocity_speed;
        self.aoa = aoa;
        self.heading = heading;
        self.pitch = pitch;
        self.bank = bank;
        self.aos = aos;
        self.euler_vx = euler_vx;
        self.euler_vy = euler_vy;
        self.euler_vz = euler_vz;
        self.canopy_pos = canopy_pos;
        self.flap_pos = flap_pos;
        self.gear_value = gear_value;
        self.speedbrake_value = speedbrake_value;
        self.afterburner_1 = afterburner_1;
        self.afterburner_2 = afterburner_2;
        self.weapon = weapon;
        self.flare = flare;
        self.chaff = chaff;
        self.cannon_shells = cannon_shells;
        self.mach = mach;
        self.altitude_sea_level = altitude_sea_level;
        self.led_instruments_result = led_instruments_result;
        self.light_apu_ready = light_apu_ready;
        self.light_gear_warning = light_gear_warning;
        self.light_gear_indicator = light_gear_indicator;
    }
}

impl FrameTelemetryString for MozaFFBData {
    fn telemetry_string(&self) -> String {
        let result = format!(
            "aircraft_name,{};engine_rpm_left,{};engine_rpm_right,{};left_gear,{};nose_gear,{};right_gear,{};acc_x,{};acc_y,{};acc_z,{};wind_x,{};wind_y,{};wind_z,{};vector_velocity_x,{};vector_velocity_y,{};vector_velocity_z,{};tas,{};ias,{};vertical_velocity_speed,{};aoa,{};heading,{};pitch,{};bank,{};aos,{};euler_vx,{};euler_vy,{};euler_vz,{};helicopter_rotor_rpm,0;canopy_pos,{};flap_pos,{};gear_value,{};speedbrake_value,{};afterburner_1,{};afterburner_2,{};weapon,{};flare,{};chaff,{};cannon_shells,{};mach,{};h_above_sea_level,{};led_console,{};led_instruments_result,{};light_apu_ready,{};light_gear_warning,{};light_gear_indicator,{};",
            self.aircraft_name,
            self.engine_rpm_left,
            self.engine_rpm_right,
            self.left_gear,
            self.nose_gear,
            self.right_gear,
            self.acceleration_x,
            self.acceleration_y,
            self.acceleration_z,
            self.wind_x,
            self.wind_y,
            self.wind_z,
            self.vector_velocity_x,
            self.vector_velocity_y,
            self.vector_velocity_z,
            self.tas,
            self.ias,
            self.vertical_velocity_speed,
            self.aoa,
            self.heading,
            self.pitch,
            self.bank,
            self.aos,
            self.euler_vx,
            self.euler_vy,
            self.euler_vz,
            self.canopy_pos,
            self.flap_pos,
            self.gear_value,
            self.speedbrake_value,
            self.afterburner_1,
            self.afterburner_2,
            self.weapon,
            self.flare,
            self.chaff,
            self.cannon_shells,
            self.mach,
            self.altitude_sea_level,
            self.led_console,
            self.led_instruments_result,
            self.light_apu_ready,
            self.light_gear_warning,
            self.light_gear_indicator
        );

        result
    }
}

impl MozaFFBData {
    pub fn debug_log(&self) {}
}
