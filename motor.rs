#![allow(dead_code)]

extern crate rppal;

use rppal::i2c::I2c;
use std::mem;
use std::error::Error;
use byteorder::ByteOrder;
use byteorder::BigEndian;

pub struct Motor {
    #[allow(unused)]
    address: u16,
    pub position: f32,
    pub velocity: f32,
    pub torque: f32,
    i2c: I2c,
}
impl Motor {
    pub fn new(addr: u16) -> Result<Self, Box<dyn Error>> {
        let mut mki2c = I2c::new()?;
        mki2c.set_slave_address(addr)?;
        Ok (
            Motor {
            address: addr,
            position: 0.0,
            velocity: 0.0,
            torque: 0.0,
            i2c: mki2c,
            }
        )
    }
    pub fn saveallregisters(&mut self) -> Result<(), Box<dyn Error>> {
        let saveallregisters: [u8; 5] = [0x41, 0, 0, 0, 0];
        let _ = self.write(&saveallregisters);
        Ok(())

    }
    pub fn max_speed(&mut self, input_velocity: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_velocity) };
        let max_speed: [u8; 9] = [0x06, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&max_speed);
        Ok(())
    }
    pub fn min_speed(&mut self, input_velocity: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_velocity) };
        let min_speed: [u8; 9] = [0x06, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&min_speed);
        Ok(())
    }
    pub fn set_curvetype(&mut self) -> Result<(), Box<dyn Error>> {
        let curvetype: [u8; 6] = [0x05, 0, 0, 1, 0, 0];
        let _ = self.write(&curvetype);
        Ok(())
    }
    pub fn acc(&mut self, input_acc: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_acc) };
        let acc: [u8; 9] = [0x07, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&acc);
        Ok(())
    }
    pub fn dec(&mut self, input_dec: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_dec) };
        let dec: [u8; 9] = [0x08, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&dec);
        Ok(())
    }
    pub fn set_maxtorque(&mut self, input_torque: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_torque) };
        let maxtorque: [u8; 9] = [0x0E, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&maxtorque);
        Ok(())
    }
    pub fn read_position(&mut self) -> Result<(), Box<dyn Error>> {
        let _ = self.i2c.write(&[0xB4, 0, 0, 0, 0 ]);
        let mut buf: [u8; 16] = [0; 16];
        let _ = self.i2c.read(&mut buf)?;
        let floats_bytes = &buf[2..14];
        self.position = BigEndian::read_f32(&floats_bytes[0..4]);
        self.velocity = BigEndian::read_f32(&floats_bytes[4..8]);
        self.torque = BigEndian::read_f32(&floats_bytes[8..12]);

        Ok(())
    }
    
    pub fn set_interface(&mut self) -> Result<(), Box<dyn Error>> {
        let interface: [u8; 6] = [0x2E, 0, 0, 0x90, 0, 0];
        let _ = self.write(&interface);
        Ok(())
    }
    pub fn disable(&mut self) -> Result<(), Box<dyn Error>> {
        let disable:[u8; 5] = [0x50, 0, 0, 0, 0];
        let _ = self.write(&disable);
        Ok(())

    }
    pub fn enable(&mut self) -> Result<(), Box<dyn Error>> {
        let enable: [u8; 5] = [0x51, 0, 0, 0, 0];
        let _ = self.write(&enable);
        Ok(())
    }
    pub fn speed(&mut self, input_speed: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe { mem::transmute(input_speed) };
        let speed: [u8; 9] = [0x58, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&speed);
        Ok(())

    }
    pub fn presetposition(&mut self) -> Result<(), Box<dyn Error>> {
        let presetposition: [u8; 9] = [0x5A, 0, 0, 0, 0, 0, 0, 0, 0];
        let _ = self.write(&presetposition);
        Ok(())
    }
    pub fn runforward(&mut self) -> Result<(), Box<dyn Error>> {
        let runforward: [u8; 5] = [0x60, 0, 0, 0, 0];
        let _ = self.write(&runforward);
        Ok(())
    }
    pub fn runreverse(&mut self) -> Result<(), Box<dyn Error>> {
        let runreverse: [u8; 5] = [0x61, 0, 0, 0, 0];
        let _ = self.write(&runreverse);
        Ok(())
    }
    pub fn runatvelocity(&mut self, input_velocity: f32) -> Result<(), Box<dyn Error>> {
        let bytes:[u8; 4] = unsafe{ mem::transmute(input_velocity)};
        let runatvelocity: [u8; 9] = [0x62, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&runatvelocity);
        Ok(())
    }
    pub fn moveto(&mut self, input_position: f32) -> Result<(), Box<dyn Error>> {
        let bytes: [u8; 4] = unsafe{ mem::transmute(input_position)};
        let moveto: [u8; 9] = [0x66, 0, 0, bytes[3], bytes[2], bytes[1], bytes[0], 0, 0];
        let _ = self.write(&moveto);
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        let stop: [u8; 5] = [0x6D, 0, 0, 0, 0];
        let _ = self.write(&stop);
        Ok(())
    }
    pub fn free(&mut self) -> Result<(), Box<dyn Error>> {
        let free: [u8; 5] = [0x6C, 0, 0, 0, 0];
        let _ = self.write(&free);
        Ok(())

    }
    fn write(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        let _ = self.i2c.write(&buf)?;
        Ok(())
    }
}