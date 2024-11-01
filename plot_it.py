#!/usr/bin/env python3
import csv
import matplotlib.pyplot as plt
import numpy as np

# Load and process population data
def load_population_data(filename):
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        data = [int(row[1]) for row in reader]
    frequency = {}
    for item in data:
        frequency[item] = frequency.get(item, 0) + 1
    return data, frequency

initial_data, initial_frequency = load_population_data('initial_population.csv')
final_data, final_frequency = load_population_data('final_population.csv')

# Find the range of values across both datasets
min_value, max_value = min(min(initial_data), min(final_data)), max(max(initial_data), max(final_data))
labels = list(range(min_value, max_value + 1))

# Prepare data for plotting
initial_counts = [initial_frequency.get(label, 0) for label in labels]
final_counts = [final_frequency.get(label, 0) for label in labels]

# Plot
x = np.arange(len(labels))
width = 0.35
fig, ax = plt.subplots(figsize=(12, 6))
bars1 = ax.bar(x - width/2, initial_counts, width, label='Initial Population', color='royalblue')
bars2 = ax.bar(x + width/2, final_counts, width, label='Final Population', color='coral')

# Add text annotations for non-zero bars
for bars, color in [(bars1, 'royalblue'), (bars2, 'coral')]:
    for bar in bars:
        if bar.get_height() > 0:
            ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height(),
                    f'{int(bar.get_height())}', ha='center', va='bottom',
                    fontsize=10, color=color)

# Set labels and title
ax.set_xlabel('Fitness')
ax.set_ylabel('Population')
ax.set_title('Initial and Final Population Fitness')
ax.set_xticks(x)
ax.set_xticklabels(map(str, labels))

# Rotate x-axis labels if there are many values
if len(labels) > 10:
    plt.xticks(rotation=45)

ax.legend()
plt.tight_layout()
plt.show()
