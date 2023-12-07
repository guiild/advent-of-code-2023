DECK = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
CARD_POWER_DICT_PART1 = {
    "A":14, "K":13, "Q":12, "J":11, "T":10, "9":9, "8":8, "7":7, "6":6, "5":5, "4":4, "3":3, "2":2
}
CARD_POWER_DICT_PART2 = {
    "A":14, "K":13, "Q":12, "T":10, "9":9, "8":8, "7":7, "6":6, "5":5, "4":4, "3":3, "2":2, "J":1
}
def calculate_hand_rank(hand: str, j_joker=False):
    # get the unique values of the cards, look for how many in the hand, then classify
    # J could be changed to other cards that gives the highest rank of a hand
    if j_joker and "J" in hand:
        return max(calculate_hand_rank(hand.replace("J", s))
                   for s in DECK if s != "J")

    card_count = {}
    for card in hand:
        card_count[card] = card_count.get(card, 0) + 1
    if len(card_count) == 1:  # Five of a kind
        return 7
    if len(card_count)==2: # four of a kind or full house
        return 6 if 4 in card_count.values() else 5
    if len(card_count)==3:  # three of a kind or two pairs
        return 4 if 3 in card_count.values() else 3
    if len(card_count)==4:  # one pair
        return 2
    if len(card_count)==5:  # high card
        return 1

with open("input.txt", "r") as f:
# with open("input_ex.txt", "r") as f:
    bid_list = []
    hand_part1_list = []
    hand_part2_list = []
    for line in f:
        hand, bid = line.strip().split()
        bid_list.append(int(bid))

        # part 1
        hand_rank = calculate_hand_rank(hand)
        hand_tuple = (hand_rank, ) + tuple(CARD_POWER_DICT_PART1[card] for card in hand)
        hand_part1_list.append(hand_tuple)

        # part 2
        hand_rank = calculate_hand_rank(hand, j_joker=True)
        hand_tuple = (hand_rank, ) + tuple(CARD_POWER_DICT_PART2[card] for card in hand)
        hand_part2_list.append(hand_tuple)

    # part 1
    hand_bid_dict = dict(zip(hand_part1_list, bid_list))
    hand_sorted = sorted(hand_part1_list)  # lowest to highest
    total = sum(hand_bid_dict[hand] * (i+1) for i, hand in enumerate(hand_sorted))
    print("Total Winnings for part 1:", total)

    # part 1
    hand_bid_dict = dict(zip(hand_part2_list, bid_list))
    hand_sorted = sorted(hand_part2_list)
    total = sum(hand_bid_dict[hand] * (i+1) for i, hand in enumerate(hand_sorted))
    print("Total Winnings for part 2:", total)
