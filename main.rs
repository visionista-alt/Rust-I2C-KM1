use std::io::{self, Write};
use std::thread;
use std::time::{self, Duration, Instant};

mod motor;

/// This is a sample code to rotate KM1S for 3000ms by connecting RaspberryPi and Keigan Motor KM1S series via I2C.
/// 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // モータ初期化設定
    let mut main_motor = motor::Motor::new(0x20)?;
    let _ = main_motor.set_interface();
    let _ = main_motor.set_curvetype();
    let _ = main_motor.presetposition();
    let _ = main_motor.saveallregisters();
    let _ = main_motor.set_maxtorque(0.1);
    let _ = main_motor.enable();

    main_motor.speed(20.0);
    println!("Successfully initialized motor"); 
    main_motor.runforward();
    thread::sleep(Duration::from_millis(3000));
    main_motor.free();
        
    Ok(())
}