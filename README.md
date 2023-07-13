## A serial port extension for godot

* Written by rust with gdext

> How to use it: Download the release archive and decompress it to your godot project root directory.


Example:
```
func _ready():
    # List all connected serial devices.
    var devices = Serial.list_ports()
    print(devices)

    var serial = Serial.new()
    # Open by vender id and product id
    serial.open(path, 115200)

    # Then you can read data from serial device.
    var recv = serial.read()
    # And write report data to serial device
    serial.write(data_to_send)
```
