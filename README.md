# 3200-1B

## Assignment 1A for INF-3200

The HTTP server is implemented in Rust and replies to a single endpoint, `/helloworld`. The response is a combination of the configured Hostname and Port, provided by the environment variables `A1_HOSTNAME` and `A1_PORT`.

## Deployment

The server is ready for both containerized deployment and binary distribution for Linux (x86_64) and macOS (aarch64).

## Downloading

1. Log in to the cluster.
2. To download the script, run the following on the cluster:

   ```bash
   wget https://raw.githubusercontent.com/victorzimmer/INF3200-1A/master/src/run.sh

   chmod 552 run.sh
   ```

## Running

1. Log in to the cluster.
2. Run the following command:
   ```bash
   ./run.sh <number of nodes>
   ```
3. Test with th python script
   ```bash
   python3 testscript.py '<output from run.sh>'
   ```
4. Clean up by running
   ```bash
   /share/ifi/cleanup.sh
   ```

### Note:

Since the program picks ports at random, there is always a small chance that you may experience that the port number is already in use. If this happen, try again.
