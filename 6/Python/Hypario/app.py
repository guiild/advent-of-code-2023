"""
Author: Fabien Duterte
Alias: Hypario
Contact: fabienduterte@mailfence.com
"""

import unittest
import math
import time as pytime

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
        result = math.prod([calculate_ways_quadratic(self.times[i], self.distances[i]) for i in range(len(self.times))])
        self.assertEqual(result, 288)
    
    def test_part2_sympy(self):
        result = calculate_ways_quadratic(self.time, self.distance)
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
    
    print("binary :", first, last)
    return last - first + 1

def calculate_ways_naïve(time: int, distance: int) -> int:
    ways = 0
    for x in range(time + 1):
        result = x * (time - x)
        if result > distance: ways += 1
    
    return ways


def calculate_ways_quadratic(time: int, distance: int) -> int:
    def f(x):
        return x * (time - x)
    # we're looking for x * (time - x) - distance = 0, quadratic is : -x^2 + time * x - distance = 0, we solving a simple quadratic formula
    # solutions = (-b +/- sqrt(b**2 - 4ac)) / 2a
    solutions = [
         ( (-time + math.sqrt((time ** 2) - (4 * distance))) // -2 ), # delta = b^2 - 4ac
        ( (-time - math.sqrt((time ** 2) - (4 * distance))) // -2 )  
    ]
    # the interval includes x * (time - x) - distance = 0, you have to add 1 to the first, the last you might have equals, so remove one
    first, last = int(solutions[0] + 1), int(solutions[1])
    
    if f(last) == distance: last = last - 1
    return abs(last - first + 1)

def main():
    file = open("input.txt").read().strip()
    times, distances = parse_input(file.split("\n"))
    time, distance = int(''.join(map(str, times))), int(''.join(map(str, distances)))
    
    print("--- naïve search ---")
    print("part 1")
    start = pytime.perf_counter()
    print(math.prod([calculate_ways_naïve(times[i], distances[i]) for i in range(len(times))]))  # part 1
    end = pytime.perf_counter()
    timepart1 = end - start
    
    print("naïve search part 1 took : %s" % timepart1)
    
    print("part 2")
    start = pytime.perf_counter()
    print(calculate_ways_naïve(time, distance))  # part 2
    end = pytime.perf_counter()
    timepart2 = end - start
    
    print("naïve search part 2 took : %s" % timepart2)
    
    print("--- binary search ---")
    print("part 1")
    start = pytime.perf_counter()
    print(math.prod([calculate_ways(times[i], distances[i]) for i in range(len(times))])) 
    end = pytime.perf_counter()
    timepart1 = end - start
    
    print("binary search part 1 took : %s" % timepart1)
    
    print("part 2")
    start = pytime.perf_counter()
    print(calculate_ways(time, distance))  # part 2
    end = pytime.perf_counter()
    timepart2 = end - start
    
    print("binary search part 2 took : %s" % timepart2)
    
    print("--- using quadratic formula ---")
    print("part 1")
    start = pytime.perf_counter()
    print(math.prod([calculate_ways_quadratic(times[i], distances[i]) for i in range(len(times))]))
    end = pytime.perf_counter()
    timepart1 = end - start
    
    print("solving equation part 1 took : %s" % timepart1)
    
    print("part 2")
    start = pytime.perf_counter()
    print(calculate_ways_quadratic(time, distance))
    end = pytime.perf_counter()
    timepart2 = end - start
    
    print("solving equation part 2 took : %s" % timepart2)
    

if __name__ == "__main__":
    main()
    unittest.main()