from typing import List

def find_horizontal_reflection(pattern: List[str], offset: int=0):
    # first possible line of reflection is between 0 and 1
    for i in range(1, len(pattern)):
        if is_reflection(pattern, i, offset):
            return i
    return None

def is_reflection(pattern: List[str], i: int, offset: int):
    # assuming first reflection is the only reflection we are looking for
    assert i>0 and offset >=0
    diffs = 0
    for row1, row2 in zip(pattern[i:], pattern[i-1::-1]):
        # zip stops at the shortest iterator
        diffs += sum(s1 != s2 for s1,s2 in zip(row1, row2))
    if diffs==offset:
        return True
    return False

def transpose(pattern: List[str]):
    pattern_tr = []
    for j in range(len(pattern[0])):
        column = ""
        for row in pattern:
            column += row[j]
        pattern_tr.append(column)
    return pattern_tr

def parse_input(file_name):
    return [pattern.split() for pattern in open(file_name).read().split("\n\n")]

def main():
    pattern_list = parse_input("input.txt")
    total_part1 = 0
    total_part2 = 0
    for pattern in pattern_list:
        # part 1
        x = find_horizontal_reflection(pattern)
        if not x:
            pattern_transposed = transpose(pattern)
            y = find_horizontal_reflection(pattern_transposed)
            total_part1 += y
        else:
            total_part1 += 100*(x)

        # part 2
        x = find_horizontal_reflection(pattern, offset=1)
        if not x:
            pattern_transposed = transpose(pattern)
            y = find_horizontal_reflection(pattern_transposed, offset=1)
            total_part2 += y
        else:
            total_part2 += 100*(x)

    print("Solution part 1:", total_part1)
    print("Solution part 2:", total_part2)

if __name__ == "__main__":
    main()