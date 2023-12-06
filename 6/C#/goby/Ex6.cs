using System.Text.RegularExpressions;

namespace Goby_AoC_2023;

public partial class Ex6
{
  GameRound[] rounds = [];
  
  [GeneratedRegex("\\d{1,5}")]
  private static partial Regex RegexNumbers();

  [GeneratedRegex("\\D")]
  private static partial Regex RegexNoNumbers();

  public void Ex6Resolve(string input)
  {
    long productOfRecords = 1;
    rounds = InputExtractPart1(input);
    foreach (GameRound round in rounds)
    {
      if (round.GetPossibleRecords() != 0) { productOfRecords *= round.GetPossibleRecords(); }
    }
    Console.WriteLine($"Product of records : {productOfRecords}");
    Console.WriteLine($"Final race possibilities : {InputExtractPart2(input).GetPossibleRecords()}");
  }


  GameRound[] InputExtractPart1(string input)
  {
    string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);

    int[] times = RegexNumbers().Matches(lines[0]).Select(x => int.Parse(x.Value)).ToArray();
    int[] records = RegexNumbers().Matches(lines[1]).Select(x => int.Parse(x.Value)).ToArray();

    GameRound[] rounds = new GameRound[times.Length];
    for (int i = 0; i < times.Length; i++)
    {
      rounds[i] = new GameRound(times[i], records[i]);
    }
    return rounds;
  }

  GameRound InputExtractPart2(string input)
  {
    string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
    Console.WriteLine(RegexNoNumbers().Replace(lines[0], ""));
    Console.WriteLine(RegexNoNumbers().Replace(lines[1], ""));
    long time = long.Parse(RegexNoNumbers().Replace(lines[0], ""));
    long record = long.Parse(RegexNoNumbers().Replace(lines[1], ""));

    GameRound round = new GameRound(time, record);
    return round;
  }

  internal class GameRound
  {
    long time;
    long recordDistance;
    long possibleRecords = 0;

    public GameRound(long t, long d)
    {
      time = t;
      recordDistance = d;

      for (int i = 0; i < time; i++)
      {
        possibleRecords += (time - i) * i > recordDistance ? 1 : 0;
      }
    }

    public long GetPossibleRecords() { return possibleRecords; }
  }

}
