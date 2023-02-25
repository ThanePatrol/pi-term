#!/usr/bin/env python3
"""
    Expects the script location, ip of the node, port number
    and baud_rate as parameters in that order
    Flow is:
    1. Create Tmux container in detached mode
    3. Start ttyd on specified port


    NB: minicom will need to have Hardware Flow Control set to No for this to work
"""

import subprocess
import sys

args = sys.argv
node_ip = str(args[1])
node_id = node_ip.replace(".", "")
node_name = node_id
port = str(args[2])

# change it to kill the server associated with the node, not the entire server
#kill_tmux = "tmux kill-server"
#tmuxsubprocess.run(kill_tmux, shell=True)

create_tmux_command = 'tmux new -s process-spawner -d'
subprocess.run(create_tmux_command, shell=True)

# send keys to tmux session to open ttyd
open_ttyd_cmd = 'tmux send-keys -t process-spawner "ttyd -p ' + port + ' tmux new -A -s ' + node_name + ' bash" C-m'
subprocess.run(open_ttyd_cmd, shell=True)

# minicom_start = 'minicom -b ' + baud_rate + ' -D /dev/ttyUSB0'
# subprocess.run(minicom_start, shell=True)
