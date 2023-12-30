from functools import wraps
import time

import utils as ut

def timeit(f_py=None, unit="ms"):
    assert callable(f_py) or f_py is None
    assert unit in ["ms", "s"]
    def _decorator(func):
        @wraps(func)
        def timeit_wrapper(*args, **kwargs):
            start_time = time.perf_counter()
            result = func(*args, **kwargs)
            end_time = time.perf_counter()
            total_time = 1000*(end_time - start_time)  # ms
            if unit=="s":
                total_time /= 1000
            print(f'Function {func.__name__} Took {total_time:.4f} {unit}')
            return result
        return timeit_wrapper
    return _decorator(f_py) if callable(f_py) else _decorator

@timeit
def solve_part1(grid, starting_i, starting_j):
    loop = ut.find_loop(grid, starting_i, starting_j)
    ans_part1 = int((len(loop)+1)/2)
    return loop, ans_part1

@timeit(unit="s")
def solve_part2(grid, starting_i, starting_j, loop):
    starting_pipe = ut.infer_start_pipe_type(
        starting_i, starting_j, loop[0], loop[-1])
    loop.append(starting_pipe)
    grid[starting_i, starting_j] = starting_pipe
    new_grid = ut.scan(grid.content, loop)  # list of lists
    n_tiles_enclosed = sum(1 for row in new_grid for elem in row if elem==2)
    return grid, n_tiles_enclosed

def main():
    grid, starting_i, starting_j = ut.parse_input("input.txt")

    loop, ans_part1 = solve_part1(grid, starting_i, starting_j)
    print(f"Part 1: {ans_part1}")

    grid, n_tiles_enclosed = solve_part2(grid, starting_i, starting_j, loop)
    print(f"Part 2: {n_tiles_enclosed}")

if __name__ == "__main__":
    main()