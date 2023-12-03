import re

def parse_input(lines: list[str]) -> list[tuple[dict, dict]]:
    lines = list(lines)
    numbers = parse_numbers(lines)
    symbols = parse_symbols(lines)
    
    results = []
    for number in numbers:
        for location in number["adjacent_locations"] & symbols.keys():
            results.append((number, {"symbol": symbols[location], "location": location}))
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
                    "row": row,
                    "adjacent_locations": {
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
        for match in re.finditer(r'[^.\d]', line):
            symbols[(match.start(), row)] = match.group()
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
    
    for item in sorted_data:
        symbol = item[1]['symbol']
        location = item[1]['location']
        
        key = (symbol, location)
        if key not in groups:
            groups[key] = []
        groups[key].append(item[0])
    
    for key, group in groups.items():
        grouped_data.append((key, group))

    return grouped_data


def main():
    file = open("input.txt", "r")
    lines = [line.rstrip("\n") for line in file.readlines()]
    adjacencies = parse_input(lines)
    print(sum([number["value"] for number, _ in adjacencies]))
    print(sum(get_gear_ratios(adjacencies)))

if __name__ == '__main__':
    main()