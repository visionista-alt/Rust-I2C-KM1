use std::io::{self, Write};
use std::thread;
use std::time::{self, Duration, Instant};

mod motor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // モータ初期化設定
    let mut main_motor = motor::Motor::new(0x20)?;
    let _ = main_motor.set_interface();
    let _ = main_motor.set_curvetype();
    let _ = main_motor.acc(100000.0);
    let _ = main_motor.dec(100000.0);
    // let _ = main_motor.presetposition();
    // let _ = main_motor.saveallregisters();
    
    let _ = main_motor.set_maxtorque(0.1);
    let _ = left_motor.set_maxtorque(0.1);
    let _ = right_motor.set_maxtorque(0.1);

    let _ = main_motor.enable();
    let _ = left_motor.enable();
    let _ = right_motor.enable();

    main_motor.speed(20.0);

    println!("Successfully initialized motor");
    
    
        
    Ok(())
}