Run this to block internet access:

sudo systemctl stop NetworkManager

Run this to allow internet access:

sudo systemctl start NetworkManager

Run this to shutdown the device:

shutdown -h now

You could run this to sync system time:

ntpdate -s time.nist.gov