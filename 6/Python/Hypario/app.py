"""
Author: Fabien Duterte
Alias: Hypario
Contact: fabienduterte@mailfence.com
"""

import unittest
import math
import sympy

def parse_input(lines: list[str]):
    times = [int(time) for time in lines[0].split(":")[1].split()]
    distances = [int(duration) for duration in  lines[1].split(":")[1].split()]
    
    return times, distances

class Tests(unittest.TestCase):
    
    file = open("example.txt").read().strip()
    times, distances = parse_input(file.split("\n"))
    
    time, distance = int(''.join(map(str, times))), int(''.join(map(str, distances)))
    
    def test_part1(self):
        result = math.prod([calculate_ways_naïve(self.times[i], self.distances[i]) for i in range(len(self.times))])
        self.assertEqual(result, 288)
    
    def test_part2(self):
        result = calculate_ways_naïve(self.time, self.distance)
        self.assertEqual(result, 71503)
    
    def test_part1_binary(self):
        result = math.prod([calculate_ways(self.times[i], self.distances[i]) for i in range(len(self.times))])
        self.assertEqual(result, 288)
    
    def test_part2_binary(self):
        result = calculate_ways(self.time, self.distance)
        self.assertEqual(result, 71503)
    
    def test_part1_sympy(self):
        result = math.prod([calculate_ways_sympy(self.times[i], self.distances[i]) for i in range(len(self.times))])
        self.assertEqual(result, 288)
    
    def test_part2_sympy(self):
        result = calculate_ways_sympy(self.time, self.distance)
        self.assertEqual(result, 71503)

def calculate_ways(time: int, distance: int) -> int:
    def f(x):
        return x * (time - x)
    
    # using binary search
    low = 0
    highest = time // 2
    if f(highest) < distance: 
        return 0
    
    while low + 1 < highest:
        m = (low + highest) // 2
        if f(m) > distance:
            highest = m
        else:
            low = m
    first = low + 1
    last = (time // 2) + (time // 2 - first) + (1 if time % 2 == 1 else 0)
    
    return last - first + 1

def calculate_ways_naïve(time: int, distance: int) -> int:
    ways = 0
    for x in range(time + 1):
        result = x * (time - x)
        if result > distance: ways += 1
    
    return ways


def calculate_ways_sympy(time: int, distance: int) -> int:
    def f(x):
        return x * (time - x)
    # we're looking for x * (time - x) - distance > 0
    x = sympy.symbols('x')
    expr = x * (time - x) - distance > 0
    solution = sympy.solve_univariate_inequality(expr, x, False).boundary.args
    
    # the interval includes x * (time - x) - distance = 0, you have to add 1 to the first, the last you might have equals, so remove one
    first, last = int(solution[0]) + 1, int(solution[1])
    if f(last) == distance: last = last - 1
    
    return  last - first + 1

def main():
    file = open("example.txt").read().strip()
    times, distances = parse_input(file.split("\n"))
    time, distance = int(''.join(map(str, times))), int(''.join(map(str, distances)))
    
    print("--- naïve search ---")
    print("part 1")
    print(math.prod([calculate_ways_naïve(times[i], distances[i]) for i in range(len(times))]))  # part 1
    
    print("part 2")
    print(calculate_ways_naïve(time, distance))  # part 2
    
    print("--- binary search ---")
    print("part 1")
    print(math.prod([calculate_ways(times[i], distances[i]) for i in range(len(times))])) 
    
    print("part 2")
    print(calculate_ways(time, distance))  # part 2
    
    print("--- using sympy ---")
    print("part 1")
    print(math.prod([calculate_ways_sympy(times[i], distances[i]) for i in range(len(times))]))
    
    print("part 2")
    print(calculate_ways_sympy(time, distance))
    

if __name__ == "__main__":
    main()
    unittest.main()