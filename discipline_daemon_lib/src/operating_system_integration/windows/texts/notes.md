You could run this to block internet access:

netsh interface set interface Wi-Fi disable
netsh interface set interface Ethernet disable


You could run this to allow internet access:

netsh interface set interface Wi-Fi enable
netsh interface set interface Ethernet enable

You could run this to shutdown the device:

shutdown /p

You could run this to change a user's password:

net user username newPassword

You could run this to sync system time

w32tm /resync