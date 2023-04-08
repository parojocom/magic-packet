# Magic Packet
A Rust implementation of Wake on LAN Magic Packet sending.

# Background
Configuring a machine to allow Wake on LAN enables the machine to be woken up by the local area network. This also requires that a "Magic Packet" is sent to the machine to perform the actual wake-up.

The format of this message is as follows:
- 48 bits set to 1 (`FFFFFFFFFFFF`)
- The MAC address of the destination machine 16 times consecutively (MacMacMacMacMacMacMacMacMacMacMacMacMacMacMacMac)

Example of MAC `12:44:56:C8:12:A8`:

`FFFFFFFFFFFF124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8124456C812A8`

# Usage
Magic Packet
An example usage of Magic Packet is to include it in bash script that also performs ssh.
```
#!/bin/bash

echo "Sending Magic Packet"
magic_packet_sender "12:44:56:C8:12:A1"

echo "Giving machine time to wake up"
sleep 1

ssh host_to_connect_to
```