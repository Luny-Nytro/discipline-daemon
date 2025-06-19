# Discipline mangaes multiple devices
Discipline is typicaly configured to manage the device it is installed on. That is called the Host Device. However, Discipline can also be configured to manage other devices that connect to the internet through the host device. The host device's owner specifies how Discipline manages remote devices. 

For Discipline to manage remote devices, it needs a reliable way to identify those devices.

There are serveral ways to go about this: 
  - using IMEI
  - using the remote device's cpu identifier (cpuid).
  - manual authentication by the remote device
    - adminstrator creates a Discipline Profile for the remote device
    - for a remote device to connect to the internet through the Host Device, it needs to first authenticate itself.

## CPU Identifier
To access thte cpu identify on windows, you run
```
wmic
```
or
```
cpu get processorid
```