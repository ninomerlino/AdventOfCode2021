using System.Diagnostics;

public class Day6
{ 
    private static readonly String input = "Time:        44     80     65     72\nDistance:   208   1581   1050   1102";
    private class Race
    {
        private readonly ulong _totalTime;
        private readonly ulong _minWinningDistance;

        public Race(ulong totalTime, ulong distance)
        {
            this._totalTime = totalTime;
            this._minWinningDistance = distance + 1;
        }

        public ulong GetWinningStratsCount()
        {
            double delta = Math.Sqrt((Math.Pow(_totalTime, 2) - 4 * _minWinningDistance));
            double min = ((double)_totalTime - delta) / 2;
            double max = ((double)_totalTime + delta) / 2;
            return (ulong)(Math.Floor(max) - Math.Ceiling(min) + 1.0);
        }

        public override string ToString()
        {
            return "Race("+_totalTime.ToString()+","+_minWinningDistance+")";
        }
    }
    

    private List<Race> GeneratePart1Input()
    {
        var races = new List<Race>();
        var tmp = input.Split('\n');
        var times = tmp[0].Split(' ');
        var distances = tmp[1].Split(' ');
        var tI = 1;
        var dI = 1;
        while (tI < times.Length)
        {
            if (times[tI].Length == 0)
            {
                tI += 1;
                continue;
            }
            if (distances[dI].Length == 0)
            {
                dI += 1;
                continue;
            }
            var time = ulong.Parse(times[tI]);
            var distance = ulong.Parse(distances[dI]);
            races.Add(new Race(time,distance));
            tI += 1;
            dI += 1;
        }
        return races;
    }
    
    private Race GeneratePart2Input()
    {
        string[] tmp = input.Split('\n');
        string time = tmp[0].Split(':')[1].Replace(" ",string.Empty);
        string distance = tmp[1].Split(':')[1].Replace(" ",string.Empty);
        ulong uTime = ulong.Parse(time);
        ulong uDistance = ulong.Parse(time);
        return new Race(uTime, uDistance);
    }

    private ulong SolvePart1()
    {
        var races = GeneratePart1Input();
        ulong total = 1;
        foreach (var race in races)
        {
            total *= race.GetWinningStratsCount();
        }

        return total;
    }
    
    private ulong SolvePart2()
    {
        return GeneratePart2Input().GetWinningStratsCount();
    }

    public static void Main(string[] args)
    {
        var self = new Day6();
        var timer = new Stopwatch();
        Console.WriteLine("Solving part1");
        timer.Restart();
        var sol1 = self.SolvePart1();
        timer.Stop();
        var time1 = timer.Elapsed.TotalMicroseconds;
        Console.WriteLine("Solving part2");
        timer.Restart();
        var sol2 = self.SolvePart2();
        timer.Stop();
        var time2 = timer.Elapsed.TotalMicroseconds;
        Console.WriteLine("Solution 1: {0} [{1}us]",sol1,time1);
        Console.WriteLine("Solution 2: {0} [{1}us]",sol2,time2);
    }
    
}