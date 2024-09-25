import matplotlib.pyplot as plt
import numpy as np
import os

# Sample data
nodes = [1, 2, 4, 8, 16]
put_times = [2.2, 5.3, 7.9, 9.1, 15.8]
get_times = [1.7, 4.8, 5.5, 8.0, 12.1]

# Calculate error bars
put_errors = [0.3, 0.5, 0.8, 2.1, 1.5]
get_errors = [0.2, 0.4, 1.8, 1.9, 1.0]

fig, ax = plt.subplots(figsize=(12, 7))

# Plot data with error bars, correct lables and totle
ax.errorbar(nodes, put_times, yerr=put_errors, fmt='o-', color='#FF7F0E', capsize=5, label='PUT', linewidth=2, markersize=8)
ax.errorbar(nodes, get_times, yerr=get_errors, fmt='o-', color='#1F77B4', capsize=5, label='GET', linewidth=2, markersize=8)

ax.set_xlabel('Number of nodes in network', fontsize=14)
ax.set_ylabel('Time (seconds)', fontsize=14)
ax.set_title('Time to PUT and GET 100 different values in DHT (n=10)', fontsize=16, fontweight='bold')

# Set x-axis to logarithmic scale to only show the values we are interested in (1, 2, 4, 8, 16)
ax.set_xscale('log', base=2)
ax.set_xticks(nodes)
ax.set_xticklabels(nodes)


# Make some customizations to the plot
ax.grid(True, which="both", ls="-", alpha=0.2)
ax.legend(fontsize=12, loc='upper left')
ax.set_ylim(0, max(max(put_times), max(get_times)) * 1.2)
plt.tight_layout()

# save and display the plot in the plot directory
if not os.path.exists('plot'):
    os.makedirs('plot')

plt.savefig('plot/dht_performance.png', dpi=300, bbox_inches='tight')

plt.show()