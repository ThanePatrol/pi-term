#!/usr/bin/env python3
"""
    Expects the id of the node, port number
    and baud_rate as parameters
    Flow is:
    1. Create Tmux container
    2. Attach Tmux container
    3. Start ttyd on specified port
    4. start minicom connection to device with baud_rate


    NB: minicom will need to have Hardware Flow Control set to No for this to work
"""

import subprocess
import sys

args = sys.argv
node_id = str(args[1])
node_name = 'node-' + node_id
port = str(args[2])
baud_rate = str(args[3])

# kills any already running tmux sessions
kill_tmux = 'tmux kill-server'
subprocess.run(kill_tmux, shell=True)

create_tmux_command = 'tmux new -s ' + node_name
subprocess.run(create_tmux_command, shell=True)

attach_tmux_command = 'tmux attach -t ' + node_name
subprocess.run(attach_tmux_command, shell=True)

open_ttyd_cmd = 'ttyd -p ' + port + ' bash'
subprocess.run(open_ttyd_cmd, shell=True)

minicom_start = 'minicom -b ' + baud_rate + ' -D /dev/ttyUSB0'
subprocess.run(minicom_start, shell=True)
