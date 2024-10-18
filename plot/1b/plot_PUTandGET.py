import matplotlib.pyplot as plt
import numpy as np

def save_plot(plt, filename):
    plt.savefig(f'plot/{filename}', dpi=300, bbox_inches='tight', format='pdf')

# Data for nodes without finger table
no_finger_results = [
    {'nodes': 1, 'put_avg': 0.7524369557698568, 'put_std': 0.038715487557135025, 'get_avg': 0.6256277561187744, 'get_std': 0.00815975426550072},
    {'nodes': 2, 'put_avg': 1.7496532599131267, 'put_std': 0.01514530215204883, 'get_avg': 1.6585612297058105, 'get_std': 0.029257478264029836},
    {'nodes': 4, 'put_avg': 2.9722142219543457, 'put_std': 0.06310756044801792, 'get_avg': 2.806232531865438, 'get_std': 0.0636876141629717},
    {'nodes': 8, 'put_avg': 4.841251770655314, 'put_std': 0.11447038638841359, 'get_avg': 4.549646059672038, 'get_std': 0.0826532287334393},
    {'nodes': 16, 'put_avg': 12.935950676600138, 'put_std': 7.663110839341131, 'get_avg': 6.878310600916545, 'get_std': 0.09154756668078251}
]

# Extract data for plotting
nodes = [entry['nodes'] for entry in no_finger_results]
put_times = [entry['put_avg'] for entry in no_finger_results]
put_stds = [entry['put_std'] for entry in no_finger_results]
get_times = [entry['get_avg'] for entry in no_finger_results]
get_stds = [entry['get_std'] for entry in no_finger_results]

# Plot PUT and GET times for different node counts
plt.figure(figsize=(10, 6))
plt.errorbar(nodes, put_times, yerr=put_stds, marker='o', color='orange', label='PUT', capsize=5)
plt.errorbar(nodes, get_times, yerr=get_stds, marker='o', color='blue', label='GET', capsize=5)
plt.title('PUT and GET Operation Times for Different Node Counts (Finger-table size 0)')
plt.xlabel('Number of nodes in network')
plt.ylabel('Time (seconds)')
plt.xscale('log', base=2)
plt.xticks(nodes, labels=nodes)
plt.grid(True)
plt.legend()

save_plot(plt, 'put_get_times_vs_nodes_without_finger_table.pdf')

# Data for 16 nodes with different finger table sizes
finger_results = [
    {'finger': 0, 'put_avg': 9.727128823598227, 'put_std': 0.24937378864760984, 'get_avg': 8.968641916910807, 'get_std': 0.038643329111794224},
    {'finger': 2, 'put_avg': 6.36955205599467, 'put_std': 0.06951057331176141, 'get_avg': 5.978910128275554, 'get_std': 0.10088526026373933},
    {'finger': 4, 'put_avg': 4.421700795491536, 'put_std': 0.047840331763432345, 'get_avg': 4.0844349066416425, 'get_std': 0.025710102112591657},
    {'finger': 6, 'put_avg': 4.369396527608235, 'put_std': 0.03211150621159705, 'get_avg': 4.111806710561116, 'get_std': 0.03154388183680087},
    {'finger': 8, 'put_avg': 4.009881019592285, 'put_std': 0.0331916351896988, 'get_avg': 3.7238598664601645, 'get_std': 0.04343130067673035}
]

# Extract data for plotting for 16 nodes
finger_sizes = [entry['finger'] for entry in finger_results]
put_times_16 = [entry['put_avg'] for entry in finger_results]
put_stds_16 = [entry['put_std'] for entry in finger_results]
get_times_16 = [entry['get_avg'] for entry in finger_results]
get_stds_16 = [entry['get_std'] for entry in finger_results]

# Plot PUT and GET times for 16 nodes with different finger table sizes
plt.figure(figsize=(10, 6))
plt.errorbar(finger_sizes, put_times_16, yerr=put_stds_16, marker='o', color='orange', label='PUT', capsize=5)
plt.errorbar(finger_sizes, get_times_16, yerr=get_stds_16, marker='o', color='blue', label='GET', capsize=5)
plt.title('PUT and GET Operation Times for 16 Nodes with Different Finger-table Sizes')
plt.xlabel('Finger table size')
plt.ylabel('Time (seconds)')
plt.xticks(finger_sizes)
plt.grid(True)
plt.legend()

save_plot(plt, 'put_get_times_vs_finger_table_size_16_nodes.pdf')