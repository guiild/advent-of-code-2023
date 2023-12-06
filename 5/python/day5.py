# part 2: lower than 13756552
import time

from utils import Interval, Subset, Mapper, get_last_output

start_time = time.time()

with open("input.txt", "r") as f:
# with open("input_ex.txt", "r") as f:
# with open("input_ex2.txt", "r") as f:
    seed_pair_list = list(map(int, f.readline().split(": ")[1].strip().split()))
    start_values = seed_pair_list[::2]
    width_values = seed_pair_list[1::2]
    seed_subset_part1 = Subset(Interval(i, i+1) for i in seed_pair_list)
    seed_subset_part2 = Subset(Interval(i, i+j) for i,j in zip(start_values, width_values))
    # read and create mappers
    mapper_dict = {}
    parse_map = False
    mapper_index = 0
    for line in f:
        if not line.strip():  # empty line
            if parse_map: # we have a mapper
                assert len(domain) == len(image) == i
                mapper_dict[map_key] = Mapper(Subset(domain), Subset(image), mapper_map)
                parse_map = False
                mapper_index += 1
            continue
        if "map" in line:  # start getting the map
            map_key = str(mapper_index) + "-" + line.split()[0]
            domain = []  # will be list of intervals
            image = []
            mapper_map = {}
            parse_map = True
            i = 0
            continue
        if parse_map:  # take the line and convert it to domain and image intervals
            image_start, domain_start, width = tuple(map(int, line.strip().split()))
            domain.append(Interval(domain_start, domain_start+width))
            image.append(Interval(image_start, image_start+width))
            mapper_map.update({domain_start: image_start})
            i+=1
    # last one 
    if map_key not in mapper_dict:
        mapper_dict[map_key] = Mapper(Subset(domain), Subset(image), mapper_map)

    location_intervals_part1 = get_last_output(seed_subset_part1, mapper_dict)
    location_intervals_part2 = get_last_output(seed_subset_part2, mapper_dict)
    # subsets are ordered
    print("Closest location for part 1:", location_intervals_part1[0].start)
    print("Closest location for part 2:", location_intervals_part2[0].start)

    end_time = time.time()
    elapsed_time = end_time - start_time
    print(f"Script execution time: {elapsed_time*1000:.0f} milliseconds")
