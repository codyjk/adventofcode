#!/usr/bin/env python3

elves = []

# read each line from a file
with open("input.txt") as f:
    lines = f.readlines()

    # iterate over each line
    elf = []
    for line in lines:
        line = line.strip()
        if line == "":
            elves.append(elf)
            elf = []
        else:
            # convert to number
            elf.append(int(line))

all_sums = sorted(sum(elf) for elf in elves)

print("Highest sum: {}".format(all_sums[-1]))
print("Top 3 sums: {}".format(all_sums[-3:]))
print("Sum of top 3 sums: {}".format(sum(all_sums[-3:])))
