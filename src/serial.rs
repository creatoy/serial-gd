use godot::prelude::*;
use serialport::SerialPort;
use serialport::SerialPortType;

#[derive(GodotClass)]
#[class(base=Object)]
struct Serial {
    port: Option<Box<dyn SerialPort>>,

    #[base]
    base: Base<Object>,
}

#[godot_api]
impl ObjectVirtual for Serial {
    fn init(base: Base<Object>) -> Self {
        Self {
            port: None,
            base
        }
    }
}

#[godot_api]
impl Serial {
    /// Returns an array of all serial ports on system
    ///
    /// It is not guaranteed that these ports exist or are available even if they're
    /// returned by this function.
    #[func]
    fn list_ports() -> Array<Dictionary> {
        if let Ok(infos) = serialport::available_ports() {
            infos.into_iter().map(
                |info| {
                    let mut dict = Dictionary::new();
                    dict.insert("name", info.port_name);
                    if let SerialPortType::UsbPort(usb) = info.port_type {
                        dict.insert("type", "usb");
                        dict.insert("vid", usb.vid);
                        dict.insert("pid", usb.pid);
                        dict.insert("sn", usb.serial_number.unwrap_or("".to_string()));
                        dict.insert("manufacture", usb.manufacturer.unwrap_or("".to_string()));
                        dict.insert("product", usb.product.unwrap_or("".to_string()));
                    }
                    dict
                }
            ).collect()
        } else {
            godot_error!("Failed to list serial ports");
            Array::new()
        }
    }

    /// Open a serial port with the specified name and baud rate
    #[func]
    fn open(&mut self, name: GodotString, baud_rate: u32) -> bool {
        match serialport::new(name.to_string(), baud_rate).open() {
            Ok(port) => {
                self.port = Some(port);
                true
            },
            Err(e) => {
                godot_error!("Failed to open serial port: {}", e);
                false
            }
        }
    }

    /// Sets the baud rate.
    #[func]
    fn set_baud_rate(&mut self, baud_rate: u32) -> bool {
        match self.port {
            Some(ref mut port) => {
                match port.set_baud_rate(baud_rate) {
                    Ok(_) => true,
                    Err(e) => {
                        godot_error!("Failed to set baud rate: {}", e);
                        false
                    }
                }
            },
            None => {
                godot_error!("Serial port not open.");
                false
            }
        }
    }

    /// Write data to the serial port.
    #[func]
    fn write(&mut self, data: PackedByteArray) -> i32 {
        if let Some(port) = &mut self.port {
            match port.write(&data.to_vec()) {
                Ok(n) => n as i32,
                Err(e) => {
                    godot_error!("Failed to write to serial port: {}", e);
                    -1
                }
            }
        } else {
            godot_error!("Serial port not open");
            -1
        }
    }

    /// Read data from the serial port.
    #[func]
    fn read(&mut self) -> PackedByteArray {
        if let Some(port) = &mut self.port {
            let mut buf = vec![0u8; port.bytes_to_read().unwrap_or(0) as usize];
            match port.read(&mut buf) {
                Ok(_) => buf.as_slice().into(),
                Err(e) => {
                    godot_error!("Failed to write to serial port: {}", e);
                    PackedByteArray::new()
                }
            }
        } else {
            godot_error!("Serial port not open");
            PackedByteArray::new()
        }
    }

    /// Read exact number of bytes from the serial port.
    #[func]
    fn read_exact(&mut self, size: i32) -> PackedByteArray {
        if let Some(port) = &mut self.port {
            let mut buf = vec![0u8; size as usize];
            match port.read_exact(&mut buf) {
                Ok(_) => buf.as_slice().into(),
                Err(e) => {
                    godot_error!("Failed to write to serial port: {}", e);
                    PackedByteArray::new()
                }
            }
        } else {
            godot_error!("Serial port not open");
            PackedByteArray::new()
        }
    }

    /// Sets the state of the RTS (Request To Send) control signal.
    ///
    /// Setting a value of `true` asserts the RTS control signal. `false` clears the signal.
    #[func]
    fn set_rts(&mut self, level: bool) {
        if let Some(port) = &mut self.port {
            if let Err(e) = port.write_request_to_send(level) {
                godot_error!("Failed to set RTS: {}", e);
            }
        } else {
            godot_error!("Serial port not open");
        }
    }

    /// Writes to the Data Terminal Ready pin
    ///
    /// Setting a value of `true` asserts the DTR control signal. `false` clears the signal.
    #[func]
    fn set_dtr(&mut self, level: bool) {
        if let Some(port) = &mut self.port {
            if let Err(e) = port.write_data_terminal_ready(level) {
                godot_error!("Failed to set DTR: {}", e);
            }
        } else {
            godot_error!("Serial port not open");
        }
    }

    /// Reads the state of the CTS (Clear To Send) control signal.
    ///
    /// This function returns a boolean that indicates whether the CTS control signal is asserted.
    #[func]
    fn get_cts(&mut self) -> bool {
        if let Some(port) = &mut self.port {
            match port.read_clear_to_send() {
                Ok(level) => level,
                Err(e) => {
                    godot_error!("Failed to get CTS: {}", e);
                    false
                }
            }
        } else {
            godot_error!("Serial port not open");
            false
        }
    }

    /// Reads the state of the Data Set Ready control signal.
    ///
    /// This function returns a boolean that indicates whether the DSR control signal is asserted.
    #[func]
    fn get_dsr(&mut self) -> bool {
        if let Some(port) = &mut self.port {
            match port.read_data_set_ready() {
                Ok(level) => level,
                Err(e) => {
                    godot_error!("Failed to get DSR: {}", e);
                    false
                }
            }
        } else {
            godot_error!("Serial port not open");
            false
        }
    }

    /// Reads the state of the Ring Indicator control signal.
    /// 
    /// This function returns a boolean that indicates whether the RI control signal is asserted.
    #[func]
    fn get_ri(&mut self) -> bool {
        if let Some(port) = &mut self.port {
            match port.read_ring_indicator() {
                Ok(level) => level,
                Err(e) => {
                    godot_error!("Failed to get RI: {}", e);
                    false
                }
            }
        } else {
            godot_error!("Serial port not open");
            false
        }
    }

    /// Reads the state of the Carrier Detect control signal.
    /// 
    /// This function returns a boolean that indicates whether the CD control signal is asserted.
    #[func]
    fn get_cd(&mut self) -> bool {
        if let Some(port) = &mut self.port {
            match port.read_carrier_detect() {
                Ok(level) => level,
                Err(e) => {
                    godot_error!("Failed to get RI: {}", e);
                    false
                }
            }
        } else {
            godot_error!("Serial port not open");
            false
        }
    }

    /// Gets the number of bytes available to be read from the input buffer.
    #[func]
    fn available(&self) -> i32 {
        if let Some(port) = &self.port {
            match port.bytes_to_read() {
                Ok(bytes) => bytes as i32,
                Err(e) => {
                    godot_error!("Failed to get bytes read: {}", e);
                    0
                }
            }
        } else {
            godot_error!("Serial port not open");
            0
        }
    }

    #[func]
    fn remains(&self) -> i32 {
        if let Some(port) = &self.port {
            match port.bytes_to_write() {
                Ok(bytes) => bytes as i32,
                Err(e) => {
                    godot_error!("Failed to get bytes remains to write: {}", e);
                    0
                }
            }
        } else {
            godot_error!("Serial port not open");
            0
        }
    }
}
