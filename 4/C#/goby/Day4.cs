using System.Text.RegularExpressions;

namespace Goby_AoC_2023;

public class Ex4
{

  GameRound[] rounds = new GameRound[0];
  Dictionary<int, int> copies = new Dictionary<int, int>();
  public void Ex4Resolve(string input)
  {

    int points = 0;
    rounds = InputExtract(input);
    int copiesNumber = rounds.Length;

    // Init copies
    foreach (GameRound round in rounds)
    {
      copies.Add(round.id, 1);
    }

    // Boucle principale
    foreach (GameRound round in rounds)
    {
      int matches = round.GetNumberOfMatches();
      if (matches > 0)
      {
        for (int i = 1; i <= matches; i++)
        {
          copies[round.id + i] += copies[round.id];
          copiesNumber += copies[round.id];
        }
      }
      points += round.GetPoints();
    }

    Console.WriteLine($"Points : {points}");
    Console.WriteLine($"Copies : {copiesNumber}");
  }

  GameRound[] InputExtract(string input)
  {
    string[] lines = input.Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
    GameRound[] rounds = new GameRound[lines.Length];

    foreach (string line in lines)
    {
      GroupCollection groups = Regex.Match(line, "^Card *(?<id>\\d{1,3}):(?<win>( {1,2}\\d{1,2})*) \\|(?<try>( {1,2}\\d{1,2})*)").Groups;
      int id = Int16.Parse(groups["id"].Value);
      int[] winners = Regex.Matches(groups["win"].Value, "\\d{1,2}").Select(x => int.Parse(x.Value.Trim(' '))).ToArray();
      int[] tries = Regex.Matches(groups["try"].Value, "\\d{1,2}").Select(x => int.Parse(x.Value.Trim(' '))).ToArray();
      rounds[id - 1] = new GameRound(id, winners, tries);
    }
    return rounds;
  }

  internal class GameRound
  {
    internal int id;
    internal int[] winners;
    internal int[] tries;

    public GameRound(int id, int[] winners, int[] tries)
    {
      this.id = id;
      this.winners = winners;
      this.tries = tries;
    }

    public int GetNumberOfMatches()
    {
      int matches = 0;
      foreach (int winner in winners)
      {
        if (tries.Contains(winner))
        {
          matches++;
        }
      }
      return matches;
    }

    public int GetPoints()
    {
      int matches = GetNumberOfMatches();
      return matches == 0 ? 0 : 1 << (matches - 1);
    }
  }
}
