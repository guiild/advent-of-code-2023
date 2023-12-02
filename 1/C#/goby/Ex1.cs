using System.Text.RegularExpressions;

namespace Goby_AoC_2023;

public class Ex1
{
    public static int Ex1Resolve(string input)
    {
        int result = 0;
        // Sanitize worded numbers
        input = sanitizeWords(input);

        // Split from lines
        string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);

        // Get sum of first and last digits
        foreach (string line in lines)
        {
            string nums = Regex.Replace(line, "\\D", String.Empty);
            int lineNumber = (nums[0] - '0') * 10 + (nums[nums.Length - 1] - '0');
            result += lineNumber;
            Console.WriteLine(line + " -> " + nums + " = " + lineNumber + " -> " + result);
        }

        Console.WriteLine("--------------");
        Console.WriteLine(result);

        return result;
    }

    static string sanitizeWords(string input)
    {
        (string, string)[] words =
        [
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
        foreach ((string a, string b) in words)
        {
            input = input.Replace(a, b);
        }

        return input;
    }
}
