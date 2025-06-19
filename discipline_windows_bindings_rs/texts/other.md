prevent windows from bening resetted, run, from an elevated cmd:
reagentc.exe /disable

netsh interface show interface
netsh interface set interface "Interface Name" disable
shutdown /p
shutdown /a