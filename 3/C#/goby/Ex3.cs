using System.Text.RegularExpressions;
using Microsoft.VisualBasic;

namespace Goby_AoC_2023;

public partial class Ex3
{
    static Schematic _map;
    static Dictionary<int, List<int>> _gears = new Dictionary<int, List<int>>();

    public static int Ex3Resolve(string input)
    {
        int result = 0;
        _map = CreateMapFromInputString(input);

        MatchCollection numbersGroups = Regex.Matches(
            input.ReplaceLineEndings(""),
            "(?<number>\\d+)"
        );
        foreach (Match match in numbersGroups)
        {
            if (isMatchValid(match))
            {
                result += Int16.Parse(match.Groups["number"].Value);
            }
        }

        Console.WriteLine($"Sum of numbers : {result}");

        Console.WriteLine("");
        int gearRatio = 0;
        foreach (var gear in _gears)
        {
            //Console.WriteLine($"Gear {gear.Key} : {gear.Value.Count} values");
            if (gear.Value.Count == 2)
            {
                gearRatio += gear.Value[0] * gear.Value[1];
                // Console.WriteLine($"Gear {gear.Key} : {gear.Value[0]} * {gear.Value[1]} = {gear.Value[0] * gear.Value[1]}");
            }
        }

        Console.WriteLine($"Sum of gears ratio : {gearRatio}");

        return 0;
    }

    static Schematic CreateMapFromInputString(string input)
    {
        string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
        char[,] map = new char[lines[0].Length, lines.Length];
        for (int y = 0; y < lines.Length; y++)
        {
            char[] chars = lines[y].ToCharArray();
            for (int x = 0; x < chars.Length; x++)
            {
                map[x, y] = chars[x];
            }
        }

        return new Schematic(map);
    }

    static bool isMatchValid(Match match)
    {
        int index = match.Index;
        int length = match.Length;
        //Console.WriteLine($"Match : {match.Value}, Index : {index}, Length : {length}");

        int xFirstChar = index % _map.colsNumber;
        int yFirstChar = index / _map.colsNumber;
        //Console.WriteLine($"X : {xFirstChar}, Y : {yFirstChar}");
        bool isMatchValid = false;

        for (int y = yFirstChar - 1; y <= yFirstChar + 1; y++)
        {
            for (int x = xFirstChar - 1; x <= xFirstChar + length; x++)
            {
                if (x < 0 || y < 0 || x >= _map.colsNumber || y >= _map.rowsNumber)
                {
                    continue;
                }
                if (Regex.Match(_map.getXY(x, y).ToString(), "[^.0-9]").Success)
                {
                    isMatchValid = true;
                    if (_map.getXY(x, y) == '*')
                    {
                        int gearIndex = x + y * _map.colsNumber;
                        if (!_gears.ContainsKey(gearIndex))
                        {
                            _gears.Add(gearIndex, new List<int>());
                        }
                        _gears[gearIndex].Add(Int16.Parse(match.Groups["number"].Value));
                    }
                }
            }
        }

        //Console.WriteLine($"Valid : {isMatchValid}\n");
        return isMatchValid;
    }

    internal class Schematic
    {
        internal char[,] map;
        internal int rowsNumber;
        internal int colsNumber;

        internal Schematic(char[,] input)
        {
            map = input;
            colsNumber = map.GetLength(0);
            rowsNumber = map.GetLength(1);
            //Console.WriteLine($"Rows: {rowsNumber}, Cols: {colsNumber}");
        }

        internal char getXY(int x, int y)
        {
            return map[x, y];
        }

        internal void PrintMap()
        {
            for (int y = 0; y < rowsNumber; y++)
            {
                for (int x = 0; x < colsNumber; x++)
                {
                    Console.Write(map[x, y]);
                }
                Console.WriteLine("");
            }
        }
    }
}
