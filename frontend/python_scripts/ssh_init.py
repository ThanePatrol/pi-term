#!/usr/bin/env python3
"""
    Expects the script location, ttyd url and node username, node_ip and web driver path
    as parameters, in that order
    Goes to ttyd page, starts ssh session
"""
import sys
import time

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

args = sys.argv
url = args[1]
user = args[2]
node_ip = args[3]
driver_path = args[4]

driver = webdriver.Chrome(driver_path)
driver.get(url)

text_area = driver.find_element(By.CLASS_NAME, "xterm-helper-textarea")

ssh_cmd = "ssh " + user + "@" + node_ip
text_area.send_keys(ssh_cmd)
text_area.send_keys(Keys.ENTER)
time.sleep(0.5)
text_area = driver.find_element(By.CLASS_NAME, "xterm-helper-textarea")
text_area.send_keys("pi")
text_area.send_keys(Keys.ENTER)

driver.quit()
