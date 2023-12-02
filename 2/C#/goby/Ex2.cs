using System.Text.RegularExpressions;

namespace Goby_AoC_2023;

public partial class Ex2
{
    private const int MAX_RED = 12;
    private const int MAX_GREEN = 13;
    private const int MAX_BLUE = 14;

    public static int Ex2Resolve(string input)
    {
        int sumValidIds = 0;
        int sumPowers = 0;
        string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
        foreach (string line in lines)
        {
            GameRound currentRound = RegexExtract(line);
            if (
                currentRound.maxRed <= MAX_RED
                && currentRound.maxGreen <= MAX_GREEN
                && currentRound.maxBlue <= MAX_BLUE
            )
            {
                sumValidIds += currentRound.id;
            }

            sumPowers += currentRound.maxRed * currentRound.maxGreen * currentRound.maxBlue;
        }
        Console.WriteLine("Sum of ids where it works : " + sumValidIds);
        Console.WriteLine("Sum of powers : " + sumPowers);
        return sumValidIds;
    }

    static GameRound RegexExtract(string input)
    {
        int id = Int16.Parse(Regex.Match(input, "^Game (?<id>(\\d*)):").Groups["id"].Value);

        int maxRed = MaxFromRegex(input, " (?<number>(\\d*)) red");
        int maxGreen = MaxFromRegex(input, " (?<number>(\\d*)) green");
        int maxBlue = MaxFromRegex(input, " (?<number>(\\d*)) blue");

        return new GameRound(id, maxRed, maxGreen, maxBlue);
    }

    static int MaxFromRegex(string input, string regex)
    {
        int maxNumber = 0;
        foreach (Match currentMatch in Regex.Matches(input, regex))
        {
            int value = Int16.Parse(currentMatch.Groups["number"].Value);
            maxNumber = maxNumber < value ? value : maxNumber;
        }
        return maxNumber;
    }

    internal class GameRound
    {
        internal int id;
        internal int maxRed;
        internal int maxGreen;
        internal int maxBlue;

        internal GameRound(int id, int maxRed, int maxGreen, int maxBlue)
        {
            this.id = id;
            this.maxRed = maxRed;
            this.maxGreen = maxGreen;
            this.maxBlue = maxBlue;
        }
    }
}
