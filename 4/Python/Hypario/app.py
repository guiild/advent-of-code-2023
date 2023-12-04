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
    numbers = [line.split(":")[1].strip() for line in lines]
    
    cards = []
    
    for line in numbers:
        winning, scratched = line.split("|")
        winning_set, scratched_set = set(), set()
        for match in re.finditer(r"\d+", winning):
            winning_set.add(int(match.group()))
        for match in re.finditer(r"\d+", scratched):
            scratched_set.add(int(match.group()))
        
        cards.append(Card(winning_set, scratched_set))
    
    return cards

def main():
    file = open("input.txt", "r")
    lines = file.readlines()
    cards = parse_input([line.rstrip("\n") for line in lines])
    
    print(sum([int(2 ** (card.number_of_matches() - 1)) for card in cards]))  # part 1
    
    # part 2
    for card_number, card in enumerate(cards):
        matches = card.number_of_matches()
        for i in range(matches):
            cards[card_number + 1 + i].increment_copy_by(cards[card_number].copies)
    
    print(sum([card.copies for card in cards]))  # part 2

if __name__ == "__main__":
    main()