import re

class Card():
    winning_numbers: set[int]
    scratched_numbers: set[int]
    copies: int
    
    def __init__(self, winning_numbers: set[int], scratched_numbers: set[int], copies: int) -> None:
        self.winning_numbers = winning_numbers
        self.scratched_numbers = scratched_numbers
        self.copies = copies

def parse_input(lines: list[str]) -> list[Card]:
    # returns the number of winning numbers
    numbers = [line.split(":")[1].strip() for line in lines]
    
    cards = []
    
    for line in numbers:
        winning, scratched = line.split("|")
        winning_set, scratched_set = set(), set()
        for match in re.finditer(r"\d+", winning):
            winning_set.add(int(match.group()))
        for match in re.finditer(r"\d+", scratched):
            scratched_set.add(int(match.group()))
        
        cards.append(Card(winning_set, scratched_set, 1))
    
    return cards

def calculate_points(card: Card) -> int:
    power = len(card.winning_numbers & card.scratched_numbers)
    return int(2 ** (power - 1))

def main():
    file = open("input.txt", "r")
    lines = file.readlines()
    cards = parse_input([line.rstrip("\n") for line in lines])
    
    print(sum([calculate_points(card) for card in cards]))  # part 1

if __name__ == "__main__":
    main()