# Client speed test
import requests
import json

import time
# Get current epoch time
def tnow() -> int:
    return round(time.time()*1000)

# Difference between now and the time passed
def tdiff(epoch: int) -> int:
    return (tnow() - epoch)


URL = "http://192.168.1.20:8090"
#URL = "http://localhost:8080"
AUTH_BEARER = dict()
USERNAME = "weg"
PASSWORD = "weg123"

def pipeline_measure(meausures: list, body):
    url = URL + "/pipeline/to/frequency"
    
    # send request
    init = tnow()
    response = requests.post(url, json=body)
    diff = tdiff(init)
    measures.append(diff)

    # convert response
    
    if response.status_code == 200:
        result = response.json()
        print("Test: ", len(measures), len(result))
    else:
        print("- Error:")

if __name__ == "__main__":
    measures = []
    
    body = dict()
    with open("./tests/pipeline_request.json", "r") as f:
        body = json.load(f)
        print("Size:", len(body["signals"][0]["values"]))
        body = {
            "signal": body["signals"][0]["values"][0:131072],
            "inverse": False
        }
    for i in range(50):
        pipeline_measure(measures, body)
    
    # get average, max and min from measures
    average = sum(measures)/len(measures)
    maximum = max(measures)
    minimum = min(measures)
    print("\nResults:")
    print(f"Average: {average} ms")
    print(f"Maximum: {maximum} ms")
    print(f"Minimum: {minimum} ms")

