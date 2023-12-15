from utils import RollingStone, Platform

def parse_input(file_name):
    grid = []  # list of lists of form (ch, init_i, init_j)
    rock_list = []
    with open(file_name, "r") as f:
        for i, line in enumerate(f):
            if line.strip():
                row = []
                for j, ch in enumerate(line.strip()):
                    if ch == "O":
                        row.append(RollingStone(i, j))
                        rock_list.append(RollingStone(i, j))
                    else:
                        row.append((ch, i, j))
                grid.append(row)
    return grid, rock_list

def main():
    grid, rock_list = parse_input("input.txt")
    # part 1: 110565
    platform = Platform(grid, rock_list)
    platform.tilt(direction="N")
    load = sum(len(grid) - rock.i for rock in platform.rock_list)
    print("Answer Part 1:", load)

    # part 2: 89845
    platform.reset()
    platform.spin_cycle(n=1000000000)
    load = sum(len(grid) - rock.i for rock in platform.rock_list)
    print("Answer Part 2:", load)

if __name__ == "__main__":
    main()
