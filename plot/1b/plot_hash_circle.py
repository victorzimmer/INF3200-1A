import matplotlib.pyplot as plt
import numpy as np
import hashlib
import os

def generate_hash(node_id):
    """Generate a hash value for a given node_id."""
    return int(hashlib.md5(node_id.encode()).hexdigest(), 16) % (2**32)

def plot_node_distribution(node_counts, ring_size=2**32):
    # Create the figure and subplots for hashed and generated IDs
    fig, axes = plt.subplots(2, len(node_counts), figsize=(len(node_counts) * 3, 8), subplot_kw=dict(projection='polar'))
    fig.suptitle("Comparison of Hashed and Generated Node IDs", fontsize=16, fontweight='bold', y=1.03)

    for i, num_nodes in enumerate(node_counts):
        node_ids = [f"node_{n}" for n in range(num_nodes)]

        hashed_node_positions = [generate_hash(node_id) for node_id in node_ids]
        even_node_positions = np.linspace(0, ring_size, num_nodes, endpoint=False, dtype=int)

        # Plot the node distribution for both hashed and Generated node IDs
        plot_nodes(axes[0, i], hashed_node_positions, ring_size, f"Hashed IDs (n={num_nodes})", point_size=20)
        plot_nodes(axes[1, i], even_node_positions, ring_size, f"Generated IDs (n={num_nodes})", point_size=20)

    # Add a legend explaining the red dots (representing nodes)
    red_dot = plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='red', markersize=8, label='Nodes')
    fig.legend(handles=[red_dot], loc='upper left', fontsize=12, bbox_to_anchor=(0.5, 0.97))

    # Save and display the plot 
    if not os.path.exists('plot'):
        os.makedirs('plot')
    
    plt.savefig('plot/chord_node_distribution.pdf', dpi=300, bbox_inches='tight', format='pdf')
    # plt.show()

def plot_nodes(ax, node_positions, ring_size, title, point_size):
    """Helper function to plot the node distribution on a circular ring."""
    # Plot the hash ring (circular boundary)
    ax.plot(np.linspace(0, 2*np.pi, 1000), np.ones(1000), color='gray', linewidth=1, linestyle='--')  # Make the circle thinner
    
    # Plot node positions
    for node_pos in node_positions:
        angle = (node_pos / ring_size) * (2 * np.pi)
        ax.scatter([angle], [1.1], color='red', s=point_size, zorder=5)

    # Customize the plot
    ax.set_yticks([])
    ax.set_xticks([])
    ax.set_title(title, fontsize=12)


# Choose the number of nodes to visualize and call the function
node_counts = [1, 4, 8, 16, 100]  
plot_node_distribution(node_counts)
