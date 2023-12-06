using System.Text.RegularExpressions;

namespace Goby_AoC_2023;

public partial class Ex5
{

  long[] seeds = [];
  List<RessourceMap>[] operations = [];

  public void Ex5ResolvePart1(string input)
  {
    InputExtract(input);
    calcOperations(seeds);
  }

  public void Ex5ResolvePart2(string input)
  {
    long minimumResult = 999999999999;
    InputExtract(input);
    for (int i = 0; i < seeds.Length; i += 2)
    {
      Console.Write($"Seed {seeds[i]:n0} to {seeds[i] + seeds[i + 1]:n0} - ");
      List<long> newSeeds = new List<long>();
      for (long x = seeds[i]; x < seeds[i] + seeds[i + 1]; x++)
      {
        newSeeds.Add(x);
      }
      long calc = calcOperations(newSeeds.ToArray());
      if (calc < minimumResult) { minimumResult = calc; }
    }
    Console.WriteLine($"LOWEST LOCATION: {minimumResult:n0} / {minimumResult}");
  }

  private long calcOperations(long[] seeds)
  {
    long[] endLocations = seeds;
    for (int i = 0; i < seeds.Length; i++)
    {
      foreach (List<RessourceMap> ranges in operations)
      {
        long newLocation = -1;
        foreach (RessourceMap range in ranges)
        {
          long currentLocation = range.getDestination(endLocations[i]);
          if (currentLocation != -1)
          {
            newLocation = currentLocation;
            break;
          }
        }
        endLocations[i] = newLocation == -1 ? endLocations[i] : newLocation;
      }
    }
    Console.WriteLine($"LOWEST LOCATION: {endLocations.Min()}");
    return endLocations.Min();
  }



  [GeneratedRegex("^seeds:(?<seeds>.*)(\\r\\n)*seed-to-soil map:(\\r\\n)*(?<soil>.*)(\\r\\n)*soil-to-fertilizer map:(\\r\\n)*(?<fert>.*)(\\r\\n)*fertilizer-to-water map:(\\r\\n)*(?<wate>.*)(\\r\\n)*water-to-light map:(\\r\\n)*(?<ligh>.*)(\\r\\n)*light-to-temperature map:(\\r\\n)*(?<temp>.*)(\\r\\n)*temperature-to-humidity map:(\\r\\n)*(?<humi>.*)(\\r\\n)*humidity-to-location map:(\\r\\n)*(?<loca>.*)", RegexOptions.Singleline)]
  private static partial Regex RegexMaps();

  private void InputExtract(string input)
  {
    Match inputExtract = RegexMaps().Match(input);
    seeds = Regex.Matches(inputExtract.Groups["seeds"].Value, "\\d{1,10}").Select(x => long.Parse(x.Value)).ToArray();

    operations = new List<RessourceMap>[7];
    operations[0] = sanitizeGroup(inputExtract.Groups["soil"].Value);
    operations[1] = sanitizeGroup(inputExtract.Groups["fert"].Value);
    operations[2] = sanitizeGroup(inputExtract.Groups["wate"].Value);
    operations[3] = sanitizeGroup(inputExtract.Groups["ligh"].Value);
    operations[4] = sanitizeGroup(inputExtract.Groups["temp"].Value);
    operations[5] = sanitizeGroup(inputExtract.Groups["humi"].Value);
    operations[6] = sanitizeGroup(inputExtract.Groups["loca"].Value);
  }

  private List<RessourceMap> sanitizeGroup(string group)
  {
    List<RessourceMap> result = new List<RessourceMap>();
    string[] lines = group.Trim(['\r', '\n']).Split(new string[] { "\r\n", "\r", "\n" }, StringSplitOptions.None);
    foreach (string line in lines)
    {
      long[] groupMembers = Regex.Matches(line, "\\d{1,10}").Select(x => long.Parse(x.Value)).ToArray();
      result.Add(new RessourceMap(groupMembers[1], groupMembers[0], groupMembers[2]));
    }
    return result;
  }

  internal class RessourceMap
  {
    long sourceStart;
    long destinationStart;
    long mapRange;

    public RessourceMap(long source, long destination, long range)
    {
      sourceStart = source;
      destinationStart = destination;
      mapRange = range;
    }

    public long getDestination(long source)
    {
      if (source >= sourceStart && source < sourceStart + mapRange)
      {
        return destinationStart + (source - sourceStart);
      }
      else return -1;
    }

  }
}
