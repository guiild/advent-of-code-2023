from typing import List, Tuple
from copy import deepcopy

class Platform:
    MAX_ITERATION = 1000  # max iteration to detect a pattern in spin cycles
    def __init__(self, grid:List[List[Tuple]], rock_list: List[Tuple]):
        self.init_grid = deepcopy(grid)
        self.grid = deepcopy(grid)
        self.rock_list = deepcopy(rock_list)
        self.init_rock_list = deepcopy(rock_list)

    def tilt(self, direction: str):
        assert direction in ["N", "S", "W", "E"]
        if direction=="N":
            self._tilt_north()
        if direction=="S":
            self._tilt_south()
        if direction=="W":
            self._tilt_west()
        if direction=="E":
            self._tilt_east()

    def _tilt_north(self):
        rock_list = []
        for row in self.grid:
            for elem in row:
                if isinstance(elem, RollingStone):
                    elem = self._roll_rock_north(elem)
                    rock_list.append(elem)
        self.rock_list = rock_list

    def _tilt_south(self):
        rock_list = []
        for row in self.grid[::-1]:  # flip to work from south to north
            for elem in row:
                if isinstance(elem, RollingStone):
                    elem = self._roll_rock_south(elem)
                    rock_list.append(elem)
        self.rock_list = rock_list

    def _tilt_west(self):
        rock_list = []
        for row in self.grid:
            for elem in row:
                if isinstance(elem, RollingStone):
                    elem = self._roll_rock_west(elem)
                    rock_list.append(elem)
        self.rock_list = rock_list

    def _tilt_east(self):
        rock_list = []
        for row in self.grid:
            for elem in row[::-1]:
                if isinstance(elem, RollingStone):
                    elem = self._roll_rock_east(elem)
                    rock_list.append(elem)
        self.rock_list = rock_list

    def _roll_rock_north(self, rock):
        i, j = rock.i, rock.j
        while i-1>=0:
            elem_above = self.grid[i-1][j]
            if (isinstance(elem_above, tuple)) and (elem_above[0]=="."):
                self.grid[i][j] = elem_above
                rock.update_loc(i-1, j)
                self.grid[i-1][j] = rock
                i -= 1
            else:
                break
        return rock

    def _roll_rock_south(self, rock):
        i, j = rock.i, rock.j
        while i+1<len(self.grid):
            elem_below = self.grid[i+1][j]
            if (isinstance(elem_below, tuple)) and (elem_below[0]=="."):
                self.grid[i][j] = elem_below
                rock.update_loc(i+1, j)
                self.grid[i+1][j] = rock
                i += 1
            else:
                break
        return rock

    def _roll_rock_west(self, rock):
        i, j = rock.i, rock.j
        while j-1>=0:
            elem_left = self.grid[i][j-1]
            if (isinstance(elem_left, tuple)) and (elem_left[0]=="."):
                self.grid[i][j] = elem_left
                rock.update_loc(i, j-1)
                self.grid[i][j-1] = rock
                j -= 1
            else:
                break
        return rock

    def _roll_rock_east(self, rock):
        i, j = rock.i, rock.j
        while j+1<len(self.grid[0]):
            elem_right = self.grid[i][j+1]
            if (isinstance(elem_right, tuple)) and (elem_right[0]=="."):
                self.grid[i][j] = elem_right
                rock.update_loc(i, j+1)
                self.grid[i][j+1] = rock
                j += 1
            else:
                break
        return rock

    def spin_cycle(self, n:int = 1):
        if n>10:
            print("detecting pattern in cycles...")
            n_0, modulus = self._detect_pattern()
            if n_0 == -1:
                print("No pattern detected.")
            else:
                print(f"Pattern detected. \n\tStart (from current state):{n_0}\n\tLength {modulus}")
                n = ((n-n_0) % modulus) + n_0
            print(f"Doing {n} cycles...")
        for i in range(n):
            self._tilt_north()
            self._tilt_west()
            self._tilt_south()
            self._tilt_east()
    
    def _detect_pattern(self):
        current_grid = deepcopy(self.grid)
        current_rock_list = deepcopy(self.rock_list)
        halt = False
        i = 1
        rock_loc_hash_list = [hash(tuple((rock.i, rock.j) for rock in self.rock_list))]
        while not halt and i<self.MAX_ITERATION:
            self.spin_cycle()
            hash_temp = hash(tuple((rock.i, rock.j) for rock in self.rock_list))
            if hash_temp in rock_loc_hash_list:
                n_0 = rock_loc_hash_list.index(hash_temp)
                modulus = i - n_0
                halt = True
            else:
                rock_loc_hash_list.append(hash_temp)
            i+=1
        self.grid = current_grid
        self.rock_list = current_rock_list
        if halt:
            return n_0, modulus
        return -1

    def reset(self):
        self.grid = deepcopy(self.init_grid)
        self.rock_list = deepcopy(self.init_rock_list)

    def __repr__(self) -> str:
        return "\n".join(line for line in self.grid)
    def __str__(self) -> str:
        elem_str = lambda elem: elem[0] if isinstance(elem, tuple) else elem.ch
        return "\n".join("".join(elem_str(elem) for elem in line) for line in self.grid)

class RollingStone:
    def __init__(self, i, j):
        self.i = i
        self.j = j
        self.ch = "O"
    def update_loc(self, i, j):
        self.i = i
        self.j = j

    def __repr__(self) -> str:
        return f"(0, {self.i}, {self.j})"
