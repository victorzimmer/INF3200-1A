import os
import matplotlib.pyplot as plt

# Function to save plots
def save_plot(plt, filename):
    if not os.path.exists('plot'):
        os.makedirs('plot')
    plt.savefig('plot/' + filename, dpi=300, bbox_inches='tight', format='pdf')


# Data for combined plot of PUT and GET times (without finger table)
combined_results = [
    {'nodes': 1, 'finger': 0, 'putTimeTaken': 1.1913561820983887, 'getTimeTaken': 1.0813255310058594},
    {'nodes': 2, 'finger': 0, 'putTimeTaken': 1.996854305267334, 'getTimeTaken': 1.814741611480713},
    {'nodes': 4, 'finger': 0, 'putTimeTaken': 2.972432851791382, 'getTimeTaken': 2.7438085079193115},
    {'nodes': 8, 'finger': 0, 'putTimeTaken': 5.442094087600708, 'getTimeTaken': 4.794665336608887},
    {'nodes': 16, 'finger': 0, 'putTimeTaken': 9.121079444885254, 'getTimeTaken': 8.394300937652588}
]

# Extract data for plotting
nodes = [entry['nodes'] for entry in combined_results]
put_times = [entry['putTimeTaken'] for entry in combined_results]
get_times = [entry['getTimeTaken'] for entry in combined_results]

# Plot combined PUT and GET times for different node counts
plt.figure(figsize=(10, 6))
plt.plot(nodes, put_times, marker='o', color='blue', label='PUT Time')
plt.plot(nodes, get_times, marker='o', color='green', label='GET Time')
plt.title('PUT and GET Operation Times for Different Node Counts (Finger-table size 0)')
plt.xlabel('Number of Nodes')
plt.ylabel('Time Taken (seconds)')
plt.xticks(nodes)
plt.grid(True)
plt.legend()

save_plot(plt, 'put_get_times_vs_nodes_without_finger_table.pdf')
# plt.show()


# Data for 16 nodes with different finger table sizes
finger_results_16_nodes = [
    {'finger': 0, 'putTimeTaken': 9.41819977760315, 'getTimeTaken': 8.946151733398438},
    {'finger': 2, 'putTimeTaken': 5.827919244766235, 'getTimeTaken': 5.710876941680908},
    {'finger': 4, 'putTimeTaken': 4.324400186538696, 'getTimeTaken': 4.0490992069244385},
    {'finger': 6, 'putTimeTaken': 3.925969123840332, 'getTimeTaken': 3.7314555644989014},
    {'finger': 8, 'putTimeTaken': 3.92924427986145, 'getTimeTaken': 3.7061362266540527}
]

# Extract data for plotting for 16 nodes
finger_sizes = [entry['finger'] for entry in finger_results_16_nodes]
put_times_16 = [entry['putTimeTaken'] for entry in finger_results_16_nodes]
get_times_16 = [entry['getTimeTaken'] for entry in finger_results_16_nodes]

# Plot PUT and GET times for 16 nodes with different finger table sizes
plt.figure(figsize=(10, 6))
plt.plot(finger_sizes, put_times_16, marker='o', color='blue', label='PUT Time')
plt.plot(finger_sizes, get_times_16, marker='o', color='green', label='GET Time')
plt.title('PUT and GET Operation Times for 16 Nodes with Different Finger-table Sizes')
plt.xlabel('Finger-table Size')
plt.ylabel('Time Taken (seconds)')
plt.xticks(finger_sizes)
plt.grid(True)
plt.legend()

save_plot(plt, 'put_get_times_vs_finger_table_size_16_nodes.pdf')
# plt.show()
