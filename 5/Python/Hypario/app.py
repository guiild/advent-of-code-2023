"""
Author: Fabien Duterte
Alias: Hypario
contact: fabienduterte@mailfence.com

Part 2 is based on https://www.youtube.com/watch?v=iqTopXV13LE
to understand better how the calculus of the intervals works
"""

import re

def parse_input(lines: list[str]) -> tuple[list[int], list[list[tuple[int, int, int]]]]:
    lines = list(filter(None, lines))
    seeds = [int(seed.group()) for seed in re.finditer("\d+", lines[0])]
    
    # extract seed start and length pairs
    seed_pairs = []
    for pairs in re.finditer("\d+\s\d+", lines[0]):
        pair = pairs.group().split(" ")
        seed_pairs.append((int(pair[0]), int(pair[1])))
    
    lines.pop(0)
    
    maps = []
    current_map = []
    for line in lines:
        if "map" in line:
            # this is a new map
            if len(current_map) > 0: maps.append(current_map)
            current_map = []
        else:
            dest_start, source_start, length = (int(number.group()) for number in re.finditer("\d+", line))
            current_map.append((dest_start, source_start, length))
    maps.append(current_map)
    
    return seeds, seed_pairs, maps

def apply_map(number: int, map_: tuple[int, int, int]) -> int:
    for dest_start, source_start, length in map_:
        if source_start <= number < source_start + length:
            return dest_start + number - source_start
    return number

def apply_map_range(range_: list[tuple[int, int]], map_: tuple[int, int, int]):
    A = []
    for (dest, source, size) in map_:
        source_end = source + size 
        NR_ = []
        while range_:
            # [start                                                                      end)
            #                       [source source_end]
            # [Before          ][INTER                  ][AFTER                     )
            start, end = range_.pop()
            before = (start, min(end, source))
            inter = max(start, source), min(source_end, end)
            after = max(source_end, start), end
            if before[1] > before[0]:
                NR_.append(before)
            if inter[1] > inter[0]:
                A.append((inter[0] - source + dest, inter[1] - source + dest))
            if after[1] > after[0]:
                NR_.append(after)
        range_ = NR_
    return A + range_

def find_lowest_location(seeds: list[int], maps: list[tuple[int, int, int]]) -> int:
    current_numbers = seeds

    for current_map in maps:
        current_numbers = [apply_map(num, current_map) for num in current_numbers]

    return min(current_numbers)

def find_lowest_location_pairs(pairs, maps) -> None:
    results = []
    for start, size in pairs:
        # inclusive on the left, exclusive on the right, eg : [1, 3) = [1, 2], length of [a, b) = b-a
        R = [(start, start + size)]
        for current_map in maps:
            R = apply_map_range(R, current_map)
        results.append(min(R))
    return min(results[0])  # multiple answers are possible, but let's take the minimum
        

if __name__ == "__main__":
    
    file = open("input.txt", "r")
    lines = file.readlines()
    
    seeds, pairs, maps = parse_input([line.rstrip("\n") for line in lines])

    print("The lowest location number is:", find_lowest_location(seeds, maps))  # part 1
    
    print("The lowest location number is:", find_lowest_location_pairs(pairs, maps))  # part 2