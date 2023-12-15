from typing import List

def parse_input(file_name: str):
    seq = []  # list of strings
    with open(file_name, "r") as f:
        for line in f:
            if line.strip():
                seq.extend(line.strip().split(","))
    return seq

def hash_alg(text: str):
    current_value = 0
    for ch in text:
        current_value += ord(ch)
        current_value *= 17
        current_value = current_value % 256
    return current_value

def calc_focus_power(box_num: int, lens_idx: int, focal_length: int):
    value = (box_num + 1) * (lens_idx + 1) * (focal_length)
    return value

def hash_map(sequence: List[str]):
    # list of lists of tuples of label, focal length
    all_boxes = [[] for i in range(256)]
    for step in sequence:
        if "-" in step:
            lens_label = step[:-1]
            box_number = hash_alg(lens_label)
            # remove any lens with same label if it is present in the box
            # move lens to the front (beginning of the list)
            rel_box = all_boxes[box_number]
            for lens in rel_box:
                if lens[0] == lens_label:
                    i = rel_box.index(lens)
                    rel_box = rel_box[:i] + rel_box[i+1:]
            all_boxes[box_number] = rel_box
        elif "=" in step:
            lens_label = step[:-2]
            focal_length = int(step[-1])
            box_number = hash_alg(lens_label)
            rel_box = all_boxes[box_number]
            # replace existing lens, if any, or add them to the end of the box
            found = False
            for lens in rel_box:
                if lens[0] == lens_label:
                    found = True
                    i = rel_box.index(lens)
                    rel_box[i] = (lens[0], focal_length)
            if not found:
                rel_box.append((lens_label, focal_length))
            all_boxes[box_number] = rel_box  # not needed, but just to be clear
        else:
            raise ValueError("no - nor =")
    return all_boxes

def main():
    # part 1
    sequence = parse_input("input.txt")
    answer = sum(hash_alg(x) for x in sequence)
    print("Answer for part 1:", answer)

    # part 2
    all_boxes = hash_map(sequence)
    power = 0
    for box_num, box in enumerate(all_boxes):
        for lens_idx, (_, focal_length) in enumerate(box):
            power += calc_focus_power(box_num, lens_idx, focal_length)
    print("Answer for part 2:", power)

if __name__ == "__main__":
    main()
