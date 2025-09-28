use bms_sm::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{thread, time};
use tailcall::tailcall;

use crate::models::ffb_data::{ComputeData, FrameTelemetryString, MozaFFBData};

mod models;

const TICK_SLEEP_TIME: Duration = time::Duration::from_millis(10);
const WAITING_SIM_AND_TELEMETRY_SLEEP_TIME: Duration = time::Duration::from_millis(300);

fn main() {
    let socket_port = 1234;
    let socket_host = "127.0.0.1";
    let socket: TcpListener =
        connect_to_telemetry(socket_host, socket_port).expect("Can't create socket!");

    let client = socket.accept();

    println!("Waiting for BMS");
    let flight_data = wait_for_flight_data();
    let intellivibe_data = wait_for_intellivibe_data();

    println!("Starting main loop");
    if let Ok((mut stream, addr)) = client {
        println!("new client: {addr:?}");
        main_loop(&mut stream, flight_data, intellivibe_data);
    } else {
        println!("Socket accept failure");
    }
    println!("Shutting down since BMS is closed");
}

#[tailcall]
fn wait_for_flight_data() -> MemoryFile<'static, FlightData> {
    thread::sleep(WAITING_SIM_AND_TELEMETRY_SLEEP_TIME);
    let maybe_fligth_data = FlightData::new();
    match maybe_fligth_data {
        Ok(data) => data,
        Err(_) => wait_for_flight_data(),
    }
}

#[tailcall]
fn wait_for_intellivibe_data() -> MemoryFile<'static, IntellivibeData> {
    thread::sleep(WAITING_SIM_AND_TELEMETRY_SLEEP_TIME);
    let maybe_intellivibe_data = IntellivibeData::new();
    match maybe_intellivibe_data {
        Ok(data) => data,
        Err(_) => wait_for_intellivibe_data(),
    }
}

fn socket_send(socket: &mut TcpStream, data: String) {
    let custom_flight_data = data.as_bytes();

    let result = socket.write(custom_flight_data);
    match result {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn main_loop(
    stream: &mut TcpStream,
    flight_data_file: MemoryFile<'static, FlightData>,
    intellivibe_data_file: MemoryFile<'static, IntellivibeData>,
) {
    let mut moza_data = MozaFFBData::default();
    loop {
        thread::sleep(TICK_SLEEP_TIME);

        let intellivibe_data_file = intellivibe_data_file.read();
        let flight_data_file = flight_data_file.read();
        if intellivibe_data_file.exit_game {
            break;
        } else {
            if intellivibe_data_file.paused
                || intellivibe_data_file.ejecting
                || intellivibe_data_file.end_flight
            {
                continue;
            }

            moza_data.compute_ffb_data(flight_data_file, intellivibe_data_file);
            // moza_data.debug_log();
            socket_send(stream, moza_data.telemetry_string());
        }
    }
}

fn connect_to_telemetry(socket_host: &str, socket_port: i32) -> std::io::Result<TcpListener> {
    TcpListener::bind(format!("{socket_host}:{socket_port}"))
}
