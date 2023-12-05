import re

def parse_input(lines: list[str]) -> tuple[list[int], list[list[tuple[int, int, int]]]]:
    lines = list(filter(None, lines))
    seeds = [int(seed.group()) for seed in re.finditer("\d+", lines[0])]
    
    # extract seed start and length pairs
    seed_pairs = []
    for pairs in re.finditer("\d+\s\d+", lines[0]):
        pair = pairs.group().split(" ")
        seed_pairs.append((int(pair[0]), int(pair[1])))
    
    # print(all_seed_numbers)
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

def convert_number(number, mapping):
    for dest_start, source_start, length in mapping:
        if source_start <= number < source_start + length:
            return dest_start + (number - source_start)
    return number

def find_lowest_location(seeds, maps):
    current_numbers = seeds

    for current_map in maps:
        current_numbers = [convert_number(num, current_map) for num in current_numbers]

    return min(current_numbers)

def find_lowest_location_pairs(pairs, maps):
    current_numbers = set()
    
    for start, length in pairs:
        for current_map in maps:
            current_numbers.update(convert_number(num, current_map) for num in range(start, start + length))
    
    return min(current_numbers)
        

if __name__ == "__main__":
    
    file = open("input.txt", "r")
    lines = file.readlines()
    
    seeds, pairs, maps = parse_input([line.rstrip("\n") for line in lines])

    print("The lowest location number is:", find_lowest_location(seeds, maps))  # part 1
    
    print("The lowest location number is:", find_lowest_location_pairs(pairs, maps))  #  part 2 ??? (probably not working, takes too long)