"""
Author: Fabien Duterte
Alias: Hypario
contact: fabienduterte@mailfence.com
"""

def draws_are_valid(draws: dict[str, int], cube_config: dict[str, int]) -> bool:
    return all(draws.get(color, 0) <= count for color, count in cube_config.items())

def is_game_valid(draws: list[dict[str, int]]) -> bool:
    target_cube_config = {"red": 12, "green": 13, "blue": 14}
    return all(draws_are_valid(draw, target_cube_config) for draw in draws)

def find_power(draws: list[dict[str, int]]) -> int:
    min_set = {}
    # get max value of each colors to get the set of cubes
    for draw in draws:
        for color in ("red", "green", "blue"):
            min_set[color] = max(min_set.get(color, 0), draw.get(color, 0))
        
    # calculate the power of the minimum set of cubes
    power = 1
    for value in min_set.values():
        power *= value
    return power

sum_of_ids = 0   # initialize the sum of IDs for possible games
sum_of_powers = 0
with open("games.txt") as file:
    for game_id, line in enumerate(file, 1):
        # split the game record into ID and draws 
        _, draws = line.split(':')   # we are only interested in the draws (eg : 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green)
        
        # format draws
        draw_records = []
        for subsets in draws.split(";"):  # we split in one draw (eg : 3 blue, 4 red)
            draws = {}
            for cubes in subsets.split(","):  # format the draw in object format (eg : { blue: 3, red: 4 } )
                count_str, color = cubes.strip().split(" ")
                count = int(count_str)
                draws[color] = count
            draw_records.append(draws.copy())
        
        sum_of_powers += find_power(draw_records)  # part 2 of the puzzle
        if is_game_valid(draw_records):
            sum_of_ids += game_id
        
    print(sum_of_ids)
    print(sum_of_powers)