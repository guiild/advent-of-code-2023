from copy import deepcopy
from typing import List

# S will have exactly two pipes connecting to it
PIPE_TYPE = ["|", "-", "L", "J", "7", "F", "S"]

def parse_input(file_name: str):
    grid = Grid()
    lines = [l for l in open(file_name, "r").readlines() if l.strip()]
    for i, line in enumerate(lines):
        line_list = []  # list of pipes or tuples of the form (ch, i, j)
        for j, ch in enumerate(line.strip()):
            if ch not in [".", "S"]:
                pipe = Pipe((i,j), ch)
                line_list.append(pipe)
            else:
                line_list.append((ch, i, j))
                if ch == "S":
                    starting_i, starting_j = (i, j)
        grid.append_row(line_list)
    grid.pad()
    starting_i += 1
    starting_j += 1
    return grid, starting_i, starting_j

def find_loop(grid, starting_i, starting_j):
    # Assumption: S will have exactly two pipes connecting to it
    # find the next pipe to starting position
    pipe_start_loc_list = [
        (starting_i, starting_j+1),
        (starting_i, starting_j-1),
        (starting_i+1, starting_j),
        (starting_i-1, starting_j),
    ]
    for i, j in pipe_start_loc_list:
        next_tile = grid[i, j]
        if isinstance(next_tile, Pipe) and next_tile.can_reach((starting_i, starting_j)):
            second_pipe = next_tile
            break
    # define a long pipe and extend when possible
    long_pipe = Loop([second_pipe])
    while not long_pipe.is_finished:
        long_pipe.extend(grid)
    loop = long_pipe.pipe_list

    return loop

def infer_start_pipe_type(starting_i, starting_j, second, last):
    # second and last are Pipes (below)
    pipe_same_row = second if second.i == starting_i else last
    pipe_same_col = second if second.j == starting_j else last
    if starting_j - pipe_same_row.j > 0:  # S is 7 or J
        starting_type = "J" if starting_i - pipe_same_col.i > 0 else "7"
    else:  # S is L or F
        starting_type = "L" if starting_i - pipe_same_col.i > 0 else "F"
    return Pipe((starting_i, starting_j), starting_type)

def scan(grid, main_loop):
    # grid: list of lists
    # returns list of lists
    width = len(grid[0])
    # one from left
    for i, row in enumerate(grid):
        inner = 0
        last_pipe = "."
        for j, elem in enumerate(row):
            if elem in main_loop:
                if elem.type in ["|", "L", "F"]:
                    if elem.type == "|":
                        inner = (inner + 1) %2
                    last_pipe = elem.type
                elif elem.type == "J" and last_pipe == "F":
                    inner = (inner + 1) %2
                    last_pipe = elem.type
                elif elem.type == "7" and last_pipe =="L":
                    inner = (inner + 1) %2
                    last_pipe = elem.type
                continue
            if inner:
                grid[i][j] = elem + 1 if isinstance(elem, int) else 1
    # one from right
    for i, row in enumerate(grid):
        inner = 0
        for j, elem in enumerate(row[::-1]):
            if elem in main_loop: # switch
                if elem.type in ["|", "J", "7"]:
                    if elem.type == "|":
                        inner = (inner + 1) %2
                    last_pipe = elem.type
                elif elem.type == "L" and last_pipe == "7":
                    inner = (inner + 1) %2
                    last_pipe = elem.type
                elif elem.type == "F" and last_pipe == "J":
                    inner = (inner + 1) %2
                    last_pipe = elem.type
                continue
            if inner:
                grid[i][width-j-1] = elem + 1 if isinstance(elem, int) else 1
    return grid

class Pipe:
    def __init__(self, location: tuple, typ: str):
        assert typ in PIPE_TYPE
        self.location = location
        self.i = location[0]
        self.j = location[1]
        self.type = typ
        self.get_next_location()

    def get_next_location(self):
        i = self.i
        j = self.j
        if self.type == "|":
            self.start_i, self.start_j = i-1, j
            self.end_i, self.end_j = i+1, j
        elif self.type == "-":
            self.start_i, self.start_j = i, j-1
            self.end_i, self.end_j = i, j+1
        elif self.type == "L":
            self.start_i, self.start_j = i-1, j
            self.end_i, self.end_j = i, j+1
        elif self.type == "J":
            self.start_i, self.start_j = i-1, j
            self.end_i, self.end_j = i, j-1
        elif self.type == "7":
            self.start_i, self.start_j = i, j-1
            self.end_i, self.end_j = i+1, j
        elif self.type == "F":
            self.start_i, self.start_j = i+1, j
            self.end_i, self.end_j = i, j+1

    def can_reach(self, loc):
        return loc in [(self.start_i, self.start_j), (self.end_i, self.end_j)]
    def __repr__(self) -> str:
        return f"Pipe '{self.type}' at {self.location}"
    def __str__(self) -> str:
        return self.type
    def __eq__(self, other):
        if not isinstance(other, Pipe):
            return False
        return (self.i == other.i) and (self.j == other.j) and (self.type == other.type)

class Loop():
    # ordered list of pipes that connect to each other
    # A loop starts with a pipe that connects to the starting point S and another pipe
    def __init__(self, pipe_list):
        assert len(pipe_list) > 0
        # TODO: ensure the pipe_list is compatible
        self.pipe_list = deepcopy(pipe_list)
        self.is_finished = False

    def extend(self, grid):
        # find the next pipe that connects to the last pipe in self.pipe_list
        last_pipe = self.pipe_list[-1]
        # get the next pipe
        if len(self.pipe_list) == 1:
            # get the next pipe that is not an S
            next_pipe = grid[last_pipe.start_i, last_pipe.start_j]
            if isinstance(next_pipe, tuple) and next_pipe[0] == "S":
                next_pipe = grid[last_pipe.end_i, last_pipe.end_j]
        else:
            # get the next pipe that is not the one just before the last pipe
            next_pipe = grid[last_pipe.start_i, last_pipe.start_j]
            if next_pipe == self.pipe_list[-2]:
                next_pipe = grid[last_pipe.end_i, last_pipe.end_j]
            # check if you reached the starting tile
            if isinstance(next_pipe, tuple) and next_pipe[0] == "S":
                self.is_finished = True
                # don't add it so that self.pipe_list is a list of pipes only
                return
        # add to the list and check if it reaches the border of the grid
        self.pipe_list.append(next_pipe)

    def __repr__(self) -> str:
        pipe_repr = ''.join(pipe.type for pipe in self.pipe_list)
        start = self.pipe_list[0].location
        finish = self.pipe_list[-1].location
        return f"Pipe '{pipe_repr}' from {start} to {finish}"

class Grid:
    def __init__(self, content: List[List]=None):
        self.content = deepcopy(content) if content else []
    def append_row(self, row: list):
        # ensure they are of the same length
        if self.content:
            assert len(row) == len(self.content[0])
        self.content.append(deepcopy(row))
    def update(self, element, row, col):
        self.content[row][col] = deepcopy(element)
    def pad(self, fill_value="."):
        # add 1 to locations in both directions, then
        # pad two columns, then two rows
        self._shift_location(1, 1)
        for i, row in enumerate(self.content):
            self.content[i] = [(fill_value, i+1, 0)] + row + [(fill_value, i+1, len(row)+1)]
        first_row = [(fill_value, 0, j) for j in range(len(self.content[0]))]
        last_row = [(fill_value, len(self.content)+1, j) for j in range(len(self.content[0]))]
        self.content = [first_row] + self.content + [last_row]

    def __str__(self):
        rows_string = '\n'.join(
            ", ".join(p.type if isinstance(p, Pipe) else str(p[0]) for p in line_list)
            for line_list in self.content
            )
        return rows_string

    def __getitem__(self, coords):
        i, j = coords
        return self.content[i][j]

    def __setitem__(self, coords, value):
        i, j = coords
        self.content[i][j] = value

    @property
    def shape(self):
        return len(self.content), len(self.content[0])

    def get_subset(self, i_min, i_max, j_min, j_max, ignore_loc=False):
        subset = Grid([self.content[i][j_min: j_max+1] for i in range(i_min, i_max+1)])
        if ignore_loc:
            subset._shift_location(-i_min, -j_min)

        return subset

    def _shift_location(self, offset_i, offset_j):
        # positive offset moves to the right
        for i, row in enumerate(self.content):
            for j, elem in enumerate(row):
                if isinstance(elem, Pipe):
                    new_loc = (elem.i + offset_i, elem.j + offset_j)
                    self.content[i][j] =  Pipe(new_loc, elem.type)
                elif isinstance(elem, tuple):
                    new_loc = (elem[1] + offset_i, elem[2] + offset_j)
                    self.content[i][j] = (elem[0],) + new_loc
                else:
                    raise ValueError("Unknown element type")
