import os
import re
import sys
import json
import time
import random
import urllib.request


def test_throughput(nodes):
    return len(nodes)


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

        print(f"Debug, output from run script: {run_script_json_list} EOS")
        deployed_nodes = json.loads(run_script_json_list)
        test_results.append(test_throughput(deployed_nodes))

        shutdown_nodes(deployed_nodes)

    print("Tests done!")
    print(test_results)
