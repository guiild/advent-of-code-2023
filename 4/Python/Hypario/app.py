"""
Author: Fabien Duterte
alias: Hypario
contact: fabienduterte@mailfence.com
"""

import re

class Card():
    winning_numbers: set[int]
    scratched_numbers: set[int]
    copies: int
    
    def __init__(self, winning_numbers: set[int], scratched_numbers: set[int],) -> None:
        self.winning_numbers = winning_numbers
        self.scratched_numbers = scratched_numbers
        self.copies = 1
    
    def number_of_matches(self) -> int:
        return len(self.winning_numbers & self.scratched_numbers)
    
    def increment_copy_by(self, increment):
        self.copies += increment

def parse_input(line: str) -> Card:
    _, numbers = line.split(":") # extract numbers
    
    winning, scratched = numbers.split("|")
    winning_set, scratched_set = set(), set()
    
    for match in re.finditer(r"\d+", winning):
        winning_set.add(int(match.group()))
    for match in re.finditer(r"\d+", scratched):
        scratched_set.add(int(match.group()))
    
    return Card(winning_set, scratched_set)

def main():
    file = open("input.txt", "r")
    s = 0
    cards = []
    for i, line in enumerate(file.readlines()):
        cards.append(parse_input(line.rstrip("\n")))
        
        # the points are calculated as follow 1 * 2 * 2 * ... * n which can be simplified as 2^(n-1)
        s += int(2 ** (cards[i].number_of_matches() - 1))  # part 1
    
    file.close()
    
    for card_number, card in enumerate(cards):
        matches = card.number_of_matches()
        for offset in range(matches):
            cards[card_number + 1 + offset].increment_copy_by(cards[card_number].copies)
    
    print(s)
    print(sum([card.copies for card in cards]))

if __name__ == "__main__":
    main()