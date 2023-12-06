from math import floor, ceil, sqrt

with open("input.txt") as f:
    time_line = f.readline().split(": ")[1].strip()
    time_list = list(map(int, time_line.split()))
    record_line = f.readline().split(": ")[1].strip()
    record_list = list(map(int, record_line.split()))
    # part 1: write distance formula --> quadratic equation to be solved
    answer = 1
    for record, total_time in zip(record_list, time_list):
        discr = total_time**2 - 4*record
        # the solution: (total_time - sqrt(discr))/2) shouldn't be included
        lower_bound = floor(max(0, (total_time - sqrt(discr))/2)+1)
        upper_bound = ceil(min(total_time, (total_time + sqrt(discr))/2))
        total_ways_to_win = len([t for t in range(lower_bound, upper_bound)])
        answer *= total_ways_to_win
    print("total ways to beat all records", answer)

    # part 2: same idea just different numbers
    total_time_part_2 = int("".join([str(t) for t in time_list]))
    record_part_2 = int("".join([str(t) for t in record_list]))
    discr = total_time_part_2**2 - 4*record_part_2
    lower_bound = floor(max(0, (total_time_part_2 - sqrt(discr))/2)+1)
    upper_bound = ceil(min(total_time_part_2, (total_time_part_2 + sqrt(discr))/2))
    total_ways_to_win_part2 = len([t for t in range(lower_bound, upper_bound)])
    print("part 2: ", total_ways_to_win_part2)
