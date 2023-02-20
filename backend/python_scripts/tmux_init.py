#!/usr/bin/env python3
"""
    Expects the script location, ip of the node, port number
    and baud_rate as parameters in that order
    Flow is:
    1. Create Tmux container
    2. Attach Tmux container
    3. Start ttyd on specified port
    4. start minicom connection to device with baud_rate


    NB: minicom will need to have Hardware Flow Control set to No for this to work
"""

import subprocess
import sys
import time

args = sys.argv
node_ip = str(args[1])
node_name = 'node-' + node_ip
port = str(args[2])
baud_rate = str(args[3])

create_tmux_command = 'tmux new -s ' + node_name + " -d"
subprocess.run(create_tmux_command, shell=True)

# todo update syntax to "ttyd -p 3002 tmux new -A -s ttyd" to start a reattachable ttyd session

# send keys to tmux session to open ttyd
open_ttyd_cmd = 'tmux send-keys -t ' + node_name + ' "ttyd -p ' + port + ' bash"' + ' C-m'
open_ttyd_cmd2 = "ttyd tmux new -A -s ttyd vim"
subprocess.run(open_ttyd_cmd2, shell=True)

# minicom_start = 'minicom -b ' + baud_rate + ' -D /dev/ttyUSB0'
# subprocess.run(minicom_start, shell=True)
