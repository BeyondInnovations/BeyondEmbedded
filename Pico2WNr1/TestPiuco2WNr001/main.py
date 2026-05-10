import wifi
import time
import network
import machine
import ubinascii

from machine import Pin
from utime import sleep

pin = Pin("LED", Pin.OUT)
print("Booting...")

wifi.start_ap()
status = wifi.wait_for_client()
print("Connected stations:", status)

print("LED starts flashing...")
while True:
    try:
        pin.toggle()
        sleep(2) # sleep 1sec
        print("Led toggled !!")
    except KeyboardInterrupt:
        break
pin.off()
print("Finished.")