#!/bin/bash

echo "Deploying on node $nodename:$port"

BINARY_FILE="inf2300-a1-bin-x86_64-unknown-linux-gnu"
BINARY_FILE_URL="https://github.com/SeraMadeleine/INF3200-1B/releases/download/v0.4.0/inf2300-a1-bin-x86_64-unknown-linux-gnu"

if [ -f $BINARY_FILE ]; then
   echo "Binary already present."
else
   echo "Downloading binary file."
   wget -q $BINARY_FILE_URL
   chmod 554 $BINARY_FILE
   echo "Downloaded binary file."
fi


echo "Launching on $nodename:$port"
ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=$port A1_HOSTNAME=$nodename A1_PORT=$port nohup ./$BINARY_FILE &> /dev/null < /dev/null &


echo "Exiting node $nodename"
exit
