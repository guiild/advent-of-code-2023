using System;
using System.Text.RegularExpressions;

namespace Goby
{
  class Ex1
  {
    static int Main(string[] args)
    {
      string inputPath = "input/day1.txt";
      string input = File.ReadAllText(inputPath);
      int result = 0;

      // Sanitize worded numbers
      input = sanitizeWords(input);

      // Split from lines
      string[] lines = input.Split(
        new string[] { "\r\n", "\r", "\n" },
        StringSplitOptions.None);

      // Get sum of first and last digits
      foreach (string line in lines) {
        string nums = Regex.Replace(line, "\\D", String.Empty);
        int lineNumber = (nums[0] - '0') * 10 + (nums[nums.Length-1] - '0');
        result += lineNumber ;  
        Console.WriteLine(line + " -> " + nums + " = " + lineNumber + " -> " + result);
      }

      return result;
    }

    string sanitizeWords(string input) {
      (string,string)[] words = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "n9ne")
      ];
      foreach((string a, string b) in words) {
        input = input.Replace(a, b);
      }

      return input;
    }
  }
}