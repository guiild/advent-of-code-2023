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

def parse_input(lines: list[str]) -> list[Card]:
    numbers = [line.split(":")[1].strip() for line in lines]  # get all numbers for each cards
    
    cards = []
    
    for line in numbers:  # for each cards, separates the winning numbers, and the number we got
        winning, scratched = line.split("|")
        winning_set, scratched_set = set(), set()
        for match in re.finditer(r"\d+", winning):
            winning_set.add(int(match.group()))
        for match in re.finditer(r"\d+", scratched):
            scratched_set.add(int(match.group()))
        
        #  add our card with their respective values to a list
        cards.append(Card(winning_set, scratched_set))
    
    return cards

def main():
    file = open("input.txt", "r")
    lines = file.readlines()
    file.close()
    cards = parse_input([line.rstrip("\n") for line in lines])
    
    # the points are calculated as follow 1 * 2 * 2 * ... * n which can be simplified as 2^(n-1)
    print(sum([int(2 ** (card.number_of_matches() - 1)) for card in cards]))  # part 1
    
    # part 2
    
    # for each card we increment a card's copies by the copies for each match
    for card_number, card in enumerate(cards):
        matches = card.number_of_matches()
        for i in range(matches):
            cards[card_number + 1 + i].increment_copy_by(cards[card_number].copies)
    
    print(sum([card.copies for card in cards])) 

if __name__ == "__main__":
    main()