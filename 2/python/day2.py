import re

# part 1 & 2

def get_max_possible(game_line:str, color:str):
    color_info_str = ",".join(re.findall(r"\d+ "+color, game_line)) # str like "1 red, 5 red"
    color_number_list = [
        int(digit) for digit in color_info_str.replace(color, "").split(",")
        ]
    return max(color_number_list)

def is_possible_game(
        max_possible_red: int, max_possible_green: int, max_possible_blue: int):
    max_red = 12
    max_green = 13
    max_blue = 14
    return (max_possible_red <= max_red) and (
        max_possible_green <= max_green) and (max_possible_blue <= max_blue)

with open("./input.txt", "r") as f:
    power_sum = 0
    game_id_sum = 0
    for line in f:
        game_name = re.findall(r"Game \d+", line)[0]
        game_id = int(game_name.split(" ")[1])
        max_red_found = get_max_possible(line, "red")
        max_green_found = get_max_possible(line, "green")
        max_blue_found = get_max_possible(line, "blue")
        # part 1
        if is_possible_game(max_red_found, max_green_found, max_blue_found):
            game_id_sum += game_id
        # part 2
        power_sum += max_red_found * max_green_found * max_blue_found

print("The sum of the IDs of possible games", game_id_sum)
print("The sum of the power of minimal sets", power_sum)
