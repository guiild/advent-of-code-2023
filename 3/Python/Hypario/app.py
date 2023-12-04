"""
based on : https://github.com/janek37/advent-of-code/blob/main/2023/day03.py
"""

import re

def parse_input(lines: list[str]) -> list[tuple[dict, dict]]:
    numbers = parse_numbers(lines)  # get all numbers in a {value: value, start: start, row: row, adjacent_locations: []} format
    symbols = parse_symbols(lines)  # get all symbols in a {(x, y): symbol} format
    
    results = []
    for number in numbers:
        for location in number["adjacent_locations"] & symbols.keys():  # filter numbers that have symbols in their adjacent locations
            results.append((number, {"symbol": symbols[location], "location": location}))  # add them to the list
    return results

def parse_numbers(lines: list[str]) -> list[dict]:
    numbers = []
    for row, line in enumerate(lines):
        for match in re.finditer(r'\d+', line):
            value = int(match.group())
            numbers.append(
                {
                    "value": value, 
                    "start": match.start(),
                    "end": match.end(),
                    "adjacent_locations": {  # calculate the adjacent positions for the number at detection time
                        (i, j)
                        for i in range(match.start() - 1, match.end() + 1)
                        for j in range(row - 1, row + 2)
                    }
                }
            )
    return numbers

def parse_symbols(lines: list[str]) -> list[dict]:
    symbols = {}
    for row, line in enumerate(lines):
        for match in re.finditer(r'[^.\d]', line):  # parse everything that's not a digit or a dot
            symbols[(match.start(), row)] = match.group()  # return symbol as a dict, with the position as key, and symbol as value
    return symbols


def get_gear_ratios(adjacencies: list[tuple[dict, dict]]):
    numbers_by_symbol = group_numbers_by_symbol(adjacencies)
    return (
        numbers[0]["value"] * numbers[1]["value"]
        for symbol, numbers in numbers_by_symbol
        if symbol[0] == '*' and len(numbers) == 2
    )

def group_numbers_by_symbol(adjacencies: list[tuple[dict, dict]]):
    sorted_data = sorted(adjacencies, key=lambda x: x[1]['symbol'])
    groups = {}
    grouped_data = []
    
    # for each values and their symbol we found, put them in a primitive hashmap
    for item in sorted_data:
        symbol = item[1]['symbol']
        location = item[1]['location']
        
        key = (symbol, location)
        if key not in groups:
            groups[key] = []
        groups[key].append(item[0])
    
    # convert the dictionary to a list of tuples (prob can make it a list of tuples without conversion, but it's late...)
    for key, group in groups.items():
        grouped_data.append((key, group))

    return grouped_data


def main():
    file = open("input.txt", "r")
    lines = [line.rstrip("\n") for line in file.readlines()]
    file.close()
    adjacencies = parse_input(lines)
    print(sum([number["value"] for number, _ in adjacencies]))  # part 1
    print(sum(get_gear_ratios(adjacencies)))   # part 2

if __name__ == '__main__':
    main()