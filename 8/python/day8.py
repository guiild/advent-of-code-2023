"""Solution for Problem 8.

The problem statement wasn't the best. The assumption:
- All direct sequences of steps between an A node and a Z node are of the same length.
was not stated in the problem and can be only seen in the example.
This is not enough IMO. However, once we agree on this assumption, the problem is a
simple application of the LCM.

If we want to relax this assumption, and allow sequences of different length, 
I don't think we could have a solution without exhaustively looking for all paths in all
steps until we found the time where all nodes are Z nodes. An example of such case is:

```
L

11A = (11B,XXX)
11B = (11Z,XXX)
11Z = (11A,XXX)
22A = (22Z,XXX)
22Z = (22B,XXX)
22B = (22C,XXX)
22C = (22A,XXX)
XXX = (XXX,XXX)
```

If we want to make it a little harder (and ugly IMO), we could replace the above 
assumption by two assumptions: 
- Distance between two consecutive occurrences of a starting A node is the always same.
- All distances from an A node to a Z node are coprime.
and the problem becomes an application of the Chinese Remaining Theorem.
"""
import math
from typing import List, Callable

def parse_input(file_name: str):
    with open(file_name, "r") as f:
        node_dict = {}
        for line in f:
            if not line.strip():
                continue
            if "=" not in line:
                instruction_seq = line.strip()
            else:
                node, dest = line.split(" = ")
                left, right = dest.strip()[1: -1].split(", ")
                node_dict[node] = (left, right)
    return instruction_seq, node_dict

def get_n_step(
        node_name: str, instruction_seq: List[str], node_dict: dict, is_target: Callable):
    found = 0
    step = 0
    while not found:
        for instr in instruction_seq:
            node_name = node_dict[node_name][0] if instr=="L" else node_dict[node_name][1]
            step += 1
            if is_target(node_name):
            # if node == "ZZZ":
                found=1
                break
    return step

def main():
    instruction_seq, node_dict = parse_input("input.txt")
    # part 1
    node = "AAA"
    n_step = get_n_step(node, instruction_seq, node_dict, lambda node: node=="ZZZ")
    print("Steps required to reach the node ZZZ:", n_step)

    # part 2
    node_list = [node for node in node_dict if node.endswith("A")]
    step_list = []
    for node in node_list:
        n_step = get_n_step(
            node, instruction_seq, node_dict, lambda node: node.endswith("Z"))
        step_list.append(n_step)
    print("Steps until you're only on nodes that end with Z", math.lcm(*step_list))


if __name__ == "__main__":
    main()
