#!/usr/bin/env python3

"""
    Expects the script location, ttyd url, /dev/path, baud rate and webdriver path as parameters
    in that order
    Goes to ttyd page, starts minicom session

    NB: minicom will need to have Hardware Flow Control set to No for this to work

"""
import sys
import time

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

args = sys.argv
url = args[1]
tty_path = args[2]
baud_rate = args[3]
driver_path = args[4]

driver = webdriver.Chrome(driver_path)
driver.get(url)

text_area = driver.find_element(By.CLASS_NAME, "xterm-helper-textarea")

minicom_cmd = "minicom -b " + baud_rate + " -D " + tty_path
text_area.send_keys(minicom_cmd)
text_area.send_keys(Keys.ENTER)
driver.quit()
