#!/bin/bash

regex_positive_integer="^[0-9]+$"  # regex for positive integer

# See if an argument is provided
if [ -z "$1" ]; then  # -z checks if the variable is empty
    echo "No argument provided, please provide the number of servers to start: $0 <number_of_servers>"
    exit 1
fi

# See if the argument is an integer
if ! [[ "$1" =~ $regex_positive_integer ]]; then
    echo "The argument provided is not an integer, please provide an integer value: $0 <number_of_servers>"
    exit 1
fi


# Download run-node.sh if not already present
if [ -f "run-node.sh" ]; then
   echo "run-node.sh already present."
else
   echo "Downloading run-node.sh..."
   wget -q "https://raw.githubusercontent.com/SeraMadeleine/INF3200-1B/refs/heads/master/src/run-node.sh"
   echo "Downloaded run-node.sh."
fi






# Number of requested nodes is initialized as requested by user
requested_node_count=$1

echo "Starting $requested_node_count instances..."


# Get available nodes from cluster
echo "Checking available nodes..."
available_nodes=$(/share/ifi/available-nodes.sh)

# echo "Available nodes: $available_nodes"  # DEBUG PRINT

# Print information about available nodes
available_node_count=$(echo "$available_nodes" | wc -w)
echo "Found $available_node_count available nodes!"

# Loop through the number of servers to start and assign nodes to servers
node_list=$(echo "$available_nodes" | shuf -n "$1")
# echo "Node list: $node_list"  # DEBUG PRINT


if [ $requested_node_count -gt $available_node_count ]
then
    echo "Warning: Requested number of instances is higher than available nodes, some instances will be deployed to the same node."
fi


remaining_node_count=$requested_node_count

deployed_services=()

deployed_nodes_count=0

previous_node=0
previous_port=0

while [ $remaining_node_count -gt 0 ]
do
    for node in $node_list; do
        if [ $remaining_node_count -gt 0 ]
        then
            port=$(shuf -i 49152-65535 -n 1)
            (echo "nodename=$node port=$port"; cat run-node.sh) | ssh $node /bin/bash
            # TODO: add precessor and successor, might look something like this curl http://$node:$port/setup/precessor -d SOME_DATA

            # TODO: calculate finger table

            echo "Started server on node: $node:$port"
            deployed_services+=("$node:$port")
            remaining_node_count=$((remaining_node_count-1))

            if [ deployed_nodes_count -eq 0 ]
            then
                echo "Initializing network on first node."
                curl -v -X "PUT" -H "Content-Type: application/json" --data "{\"network_id\": \"chord-network\"}" "http://$node:$port/network/initialize"
            else
                echo "Joining node to previous node."
                curl -v -X "PUT" -H "Content-Type: application/json" --data "{\"hostname\": \"$previous_node\", \"port\":$previous_port}" "http://$node:$port/network/join"
            fi
        fi
    done
done



# Initialize the JSON string
json_result="["

# Remove the trailing comma (only if the array isn't empty) and close the JSON array
if [ ${#deployed_services[@]} -gt 0 ]; then
    # Loop through the deployed_services array and append each service to json_result
    for service in "${deployed_services[@]}"; do
        json_result+="\"$service\","
    done

    # Strip last comma
    json_result="${json_result%,}]"
else
    # Handle empty list
    json_result+="]"
fi

# Print the JSON string
echo "$json_result"
