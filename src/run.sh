#!/bin/bash

regex_positiv_integer="^[0-9]+$"  # regex for positive integer

# See if an argument is provided
if [ -z "$1" ]; then  # -z checks if the variable is empty
    echo "No argument provided, please provide the number of servers to start: $0 <number_of_servers>"
    exit 1
fi 

# See if the argument is an integer 
if ! [[ "$1" =~ $regex_positiv_integer ]]; then
    echo "The argument provided is not an integer, please provide an integer value: $0 <number_of_servers>"
    exit 1
fi

# Get available nodes from cluster
availe_nodes=$(/share/ifi/available-nodes.sh)  # Note the syntax correction here

# node_count=$(echo "$availe_nodes" | wc -w)
# echo "Available nodes, count: $node_count"  # DEBUG PRINT
# # Check if there are enough nodes
# if [ "$1" -gt "${#availe_nodes[@]}" ]; then
#     echo "Not enough nodes available, please provide a smaller number of servers"
#     exit 1
# fi

# Loop through the number of servers to start and assign nodes to servers
node_list=$(echo "$availe_nodes" | shuf -n "$1")      # converterer til et array:  cut -d' ' -f1-"$1"
echo "Node list: $node_list"  # DEBUG PRINT

json_output=()

for node in $node_list; do  
    echo "Starting server on node: $node"  # DEBUG PRINT 
    port=$(shuf -i 49152-65535 -n 1)       # Get a random port number between 49152 and 65535 <- from the assignment 
    # Command to start the server with ssh 
    ## sjekke at podman er installert, last ned conteiner 
    ssh -f $node "echo $HOST; exit"
    json_output+=("\"$node:$port\"")
done

# Format the output as a JSON list
json_result="[$(IFS=,; echo "${json_output[*]}")]"
echo "$json_result"
