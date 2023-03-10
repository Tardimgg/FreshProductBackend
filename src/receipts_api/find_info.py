from selenium.webdriver.support import expected_conditions as ec
from selenium.common import TimeoutException

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.select import Select
from selenium.webdriver.support.wait import WebDriverWait

import os

def format_text(text):
    for i in range(len(text)):
        if text[i].isalpha():
            return text[i:]
    return text


def get_receipt_info(fn, fd, fp, total_sum, date, time, receipt_type):

    # fn = "9960440301300647"
    # fd = "115332"
    # fp = "0657304403"
    # sum_amount = "23.80"
    date = "07.07.2022"
    # time = "19:27"
    # receipt_type = "Приход"
    ans = []

    try:

        options = webdriver.ChromeOptions()
        options.binary_location = os.environ.get("GOOGLE_CHROME_BIN")
        options.add_argument("--window-size=1280,720")
        options.add_argument('--headless')
        options.add_argument('--no-sandbox')
        options.add_argument("--disable-dev-shm-usage")


        # driver = webdriver.Chrome(chrome_options=options)
        driver = webdriver.Chrome(executable_path=os.environ.get("CHROMEDRIVER_PATH"), chrome_options=options)
        # driver = webdriver.Chrome(executable_path=os.environ.get("CHROMEDRIVER_PATH"), options=options)
        driver.get("https://proverkacheka.com/")

        elem = driver.find_element(By.ID, "b-checkform_fn")
        elem.send_keys(fn)

        elem = driver.find_element(By.ID, "b-checkform_fd")
        elem.send_keys(fd)

        elem = driver.find_element(By.ID, "b-checkform_fp")
        elem.send_keys(fp)

        elem = driver.find_element(By.ID, "b-checkform_s")
        elem.send_keys(total_sum)

        elem = driver.find_element(By.ID, "b-checkform_date")
        elem.send_keys(date)

        elem = driver.find_element(By.ID, "b-checkform_time")
        elem.send_keys(time)

        select = Select(driver.find_element(By.ID, "b-checkform_n"))
        select.select_by_visible_text(receipt_type)

        elem = driver.find_element(By.XPATH, "//*[@id='b-checkform_tab-props']/div/div/div/form/div[7]/div/button[1]")
        elem.click()

        elem_wait = WebDriverWait(driver, 10).until(ec.presence_of_element_located((By.CLASS_NAME, "b-check_item")))
        elem = driver.find_elements(By.CLASS_NAME, "b-check_item")
        for td in elem:
            v = td.find_element(By.XPATH, "td[2]")
            ans.append(format_text(v.text))
    except TimeoutException:
        ans = []

    return ans

