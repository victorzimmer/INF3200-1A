import os
import re
import uuid
import sys
import json
import time
import random
import urllib.request
import numpy as np

def test_throughput(nodes, num_runs=3):
    key_value_to_test = [(uuid.uuid4(), uuid.uuid4()) for _ in range(1000)]
    
    put_times = []
    get_times = []

    for _ in range(num_runs):
        # Measure PUT time
        put_start_time = time.time()
        for key, value in key_value_to_test:
            req = urllib.request.Request(
                url=f"http://{random.choice(nodes)}/storage/{key}", 
                data=bytes(str(value).encode("utf-8")), 
                method="PUT"
            )
            req.add_header("Content-type", "text/plain")
            urllib.request.urlopen(req)
        put_end_time = time.time()
        put_times.append(put_end_time - put_start_time)

        # Measure GET time
        success_counter = 0
        failure_counter = 0
        get_start_time = time.time()
        for key, value in key_value_to_test:
            response = urllib.request.urlopen(f"http://{random.choice(nodes)}/storage/{key}").read()
            if response.decode("utf-8") == str(value):
                success_counter += 1
            else:
                failure_counter += 1
        get_end_time = time.time()
        get_times.append(get_end_time - get_start_time)

    # Calculate mean and standard deviation
    put_avg = np.mean(put_times)
    put_std = np.std(put_times)
    get_avg = np.mean(get_times)
    get_std = np.std(get_times)

    return {
        "put_avg": put_avg,
        "put_std": put_std,
        "get_avg": get_avg,
        "get_std": get_std,
        "successes": success_counter,
        "failures": failure_counter
    }

def shutdown_nodes(nodes):
    for node in nodes:
        print(f"Shutting down node: {node}")
        response = urllib.request.urlopen(f"http://{node}/shutdown").read()



if __name__ == "__main__":
    if len(sys.argv) != 2: print(f"Usage: python3 {sys.argv[0]} <combinations of node count and finger table size to test list(list(node_count, finger_table_size))>\nExample: python3 {sys.argv[0]} '[[4,0], [8,0], [8,2], [8,4], [16,0], [16,2], [16,4], [16,8]]'") ; sys.exit(1)

    try:
        tests_to_run = json.loads(sys.argv[1])
    except json.JSONDecodeError:
        print("Invalid JSON provided.")


    test_results = []

    for test in tests_to_run:
        node_count = test[0]
        finger_table_size = test[1]

        run_script_output = os.popen(f"sh ../src/run.sh {node_count} {finger_table_size}").read()
        run_script_json_list_match = re.search("\\[\\\".*\\\"\\]", run_script_output)
        run_script_json_list = run_script_json_list_match.group()

        # print(f"Debug, output from run script: {run_script_json_list} EOS")
        print(f"Running test with {node_count} nodes and finger table size {finger_table_size}")
        deployed_nodes = json.loads(run_script_json_list)
        test_results.append({"test": test, "result": test_throughput(deployed_nodes)})

        shutdown_nodes(deployed_nodes)

    print("Tests done!")
    print(test_results)
