def parse_input(line: str):
    """Parses a line of input containing card information and returns relevant data.

    Args:
        line (str): A string representing input with the following format:
        "Card <card_id>: <winning_set> | <elf_set>".

    Returns:
        Tuple[int, Set[int], Set[int]]: A tuple containing the card ID, winning set, 
            and elf set parsed from the input.

    Example:
    >>> parse_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    (1, {41, 48, 83, 86, 17}, {83, 86, 6, 31, 17, 9, 48, 53})
    """
    card_id_part = line.split(": ")[0].strip()
    card_id = int(card_id_part.strip().split()[1])
    two_sets_part = line.split(": ")[1].strip()
    winning_set_part, elf_set_part = two_sets_part.split("|")
    winning_set = set(int(i.strip()) for i in winning_set_part.strip().split())
    elf_set = set(int(i.strip()) for i in elf_set_part.strip().split())

    return card_id, winning_set, elf_set

with open("input.txt", "r") as f:
    # part 1 & 2
    total_points = 0
    new_pile_dict = {1: 1}  # count of each card
    for line in f:
        card_id, winning_set, elf_set = parse_input(line)

        # part 1
        n_winning_cards = len(winning_set.intersection(elf_set))
        if n_winning_cards > 0:
            total_points += 2**(n_winning_cards-1)

        # part 2
        new_card_list = [i for i in range(card_id+1, card_id+n_winning_cards+1)]
        # we win a card for each copy of the input card
        # add old copies too of the cards we win
        new_card_copies = {
            i: new_pile_dict.get(i, 1) + new_pile_dict.get(card_id, 1)
            for i in new_card_list
            }
        if card_id not in new_pile_dict:
            new_pile_dict[card_id] = 1
        # update dict
        for k, v in new_card_copies.items():
            new_pile_dict[k] = v

    total_n_cards = sum(v for v in new_pile_dict.values())
    print("Part 1: Total points:", total_points)
    print("Part 2: Total number of cards:", total_n_cards)
        



