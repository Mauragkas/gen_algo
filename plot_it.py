import csv
import matplotlib.pyplot as plt
import numpy as np

with open('initial_population.csv', 'r') as f:
    reader = csv.reader(f)
    initial_data = list(reader)

initial_data = [int(i[1]) for i in initial_data]

initial_frequency = {}
for item in initial_data:
    if item in initial_frequency:
        initial_frequency[item] += 1
    else:
        initial_frequency[item] = 1

with open('final_population.csv', 'r') as f:
    reader = csv.reader(f)
    final_data = list(reader)

final_data = [int(i[1]) for i in final_data]

final_frequency = {}
for item in final_data:
    if item in final_frequency:
        final_frequency[item] += 1
    else:
        final_frequency[item] = 1

labels = sorted(set(initial_frequency.keys()).union(set(final_frequency.keys())))
initial_counts = [initial_frequency.get(label, 0) for label in labels]
final_counts = [final_frequency.get(label, 0) for label in labels]

x = np.arange(len(labels))  
width = 0.35  

fig, ax = plt.subplots()
bars1 = ax.bar(x - width/2, initial_counts, width, label='Initial Population', color='blue')
bars2 = ax.bar(x + width/2, final_counts, width, label='Final Population', color='orange')

ax.set_xlabel('Population')
ax.set_ylabel('Frequency')
ax.set_title('Population Frequency Comparison')
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend()

plt.show()
