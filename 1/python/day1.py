import re

# part 1
part1_regex = re.compile(r"\d")
with open("./input.txt", "r") as f:
    total = 0
    for line in f:
        search_list = re.findall(part1_regex, line)
        cal_value = 10*int(search_list[0]) + int(search_list[-1])
        total += cal_value
print("the sum of all of the calibration values:", total)

# part 2
alphabet_digits = {
    "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, 
    "six": 6, "seven": 7, "eight": 8, "nine": 9,
    }
part2_regex = re.compile(r"(?=(" + "|".join(alphabet_digits.keys()) + r"|\d))")
with open("./input.txt", "r") as f:
    total = 0
    for line in f:
        search_list = re.findall(part2_regex, line)
        first_digit = alphabet_digits.get(search_list[0], search_list[0])
        second_digit = alphabet_digits.get(search_list[-1], search_list[-1])
        cal_value = 10*int(first_digit) + int(second_digit)
        total += cal_value
print("the _corrected_ sum of all of the calibration values:", total)