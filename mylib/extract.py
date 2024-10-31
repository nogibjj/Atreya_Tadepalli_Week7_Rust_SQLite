import requests
import os
"""
NYC Taxi trips dataset, arranged by day
"""


def extract(url="https://raw.githubusercontent.com/fivethirtyeight/uber-tlc-foil-response/refs/heads/master/Uber-Jan-Feb-FOIL.csv", 
            file_path="data/Uber-Jan-Feb-FOIL.csv",
            directory="data",
):
    """"Extract a url to a file path"""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, 'wb') as f:
            f.write(r.content)
    return file_path

