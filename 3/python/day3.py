from typing import List
import re

# part 1
def is_part_number(
        d_start: int, d_end: int, symbol_idx_list: List[int], line_width: int):
    """Checks if the digit is adjacent to a symbol (even diagonally).
    
    Args:
        d_start (int): start index of the digit.
        d_end (int): start index of the digit (inclusive).
        symbol_idx_list (list[int]): list of symbol indexes to check.
        line_width (int): line's width.

    Returns:
        (bool): wether the digit is a part number or not.
    """
    for s_index in symbol_idx_list:
        distance_start = s_index - d_start
        distance_end = s_index - d_end
        if distance_start < -1-line_width:
            continue
        elif distance_end > line_width + 1:
            return False

        distance_start = (s_index - d_start) % line_width
        distance_end = (s_index - d_end) % line_width
        adjacent_index_list = [0, 1, line_width-1]
        if distance_start in adjacent_index_list or distance_end in adjacent_index_list:
            return True
    return False

# part 2
def find_gear_ratio(s_idx: int, lines_str: str, line_width: int):
    """Finds a gear ratio if s_idx is for a gear, otherwise 0.

    A gear is any * symbol that is adjacent to exactly two part numbers.

    Args:
        s_idx (int): index of a "*" symbol.
        lines_str (str): engine schematic.
        line_width (int): line's width.

    Returns:
        (int): gear ratio if s_idx is for a gear, otherwise 0.    
    """
    # cut closest 4 lines and update s_idx
    first = max(0, s_idx - line_width - s_idx % line_width)
    end = s_idx + line_width + line_width - s_idx % line_width
    lines_str = lines_str[first: end]
    s_idx = s_idx - first
    # get digits that are adjacent to the gear
    digit_iter = re.finditer(r"\d+", lines_str)
    adjacent_parts = [
        int(m.group()) for m in digit_iter 
        if is_part_number(m.span()[0], m.span()[1]-1, [s_idx], line_width)
        ]
    if len(adjacent_parts) == 2:
        return adjacent_parts[0] * adjacent_parts[1]
    return 0

with open("input.txt", "r") as f:
    line_width = len(f.readline())
    f.seek(0)
    lines_str = "".join(f.readlines())
    # extract indices of all digits
    digit_iter = re.finditer(r"\d+", lines_str)
    # extract indices of all symbols (non-digit and non-period)
    symbol_idx_list = [
        m.start() for m in re.finditer(r"(?!(\d|\.|\s|\A|\Z))", lines_str)]

    # part 1
    sum_part_num = 0
    for d_match in digit_iter:
        d_start, d_end = d_match.span()
        d_end = d_end - 1  # so that the two ends are inclusive
        if is_part_number(d_start, d_end, symbol_idx_list, line_width):
            sum_part_num += int(d_match.group())
    
    # part 2
    sum_gear_ratio = 0
    for s_idx in symbol_idx_list:
        if lines_str[s_idx] == "*":  # could be a gear
            sum_gear_ratio += find_gear_ratio(s_idx, lines_str, line_width)  # zero if not a gear
    
    print("Sum of the parts numbers:", sum_part_num)
    print("sum of all of the gear ratios", sum_gear_ratio)
