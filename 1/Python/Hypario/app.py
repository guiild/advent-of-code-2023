import re

# part 1

total_sum = 0
with open("calibration document.txt", "r") as file:
    for line in file:
        digits = re.findall(r'(\d)', line)
        
        number = ''.join([digits[0], digits[0] if len(digits) == 1 else digits[-1]])
        
        total_sum += int(number)

print("The sum of the calibration values : ", total_sum) # 53334

# part 2

word_map = {'one': '1', 'two': '2', 'three': '3', 'four': '4', 'five': '5',
                 'six': '6', 'seven': '7', 'eight': '8', 'nine': '9', 'zero': '0'}

def word_to_digits(digit):
    return word_map.get(digit, digit)
    
total_sum = 0
with open("calibration document.txt", "r") as file:
    for line in file:
        digits = re.findall(r'(?:' + '|'.join(word_map.keys()) + '|\d)', line) # regex -> (?:one|two|three|four|five|six|seven|eight|nine|zero|\d)
        parsed = [word_to_digits(digits[0]), word_to_digits(digits[-1])] if len(digits) > 1 else  [word_to_digits(digits[0]), word_to_digits(digits[0])]
        
        total_sum += int(''.join(parsed))

print("The corrected sum of the calibration values : ", total_sum) # 52834