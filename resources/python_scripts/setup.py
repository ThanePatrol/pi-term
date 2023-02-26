#!/usr/bin/env python3
"""
    Runs the initial setup, creating ssh keys, etc
"""

import subprocess
import sys

subprocess.run("ssh-keygen", shell=True)

# need to run: sudo usermod -a -G dialout $USER
# to use minicom without sudo
