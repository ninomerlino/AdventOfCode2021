import java.util.ArrayList;

public class Day6
{
    private static final String input = "Time:        44     80     65     72\nDistance:   208   1581   1050   1102";
    private class Race
    {
        private final long _totalTime;
        private final long _minWinningDistance;

        public Race(long totalTime, long distance)
        {
            this._totalTime = totalTime;
            this._minWinningDistance = distance + 1;
        }

        public long GetWinningStratsCount()
        {
            double delta = Math.sqrt((Math.pow(_totalTime, 2) - 4 * _minWinningDistance));
            double min = ((double)_totalTime - delta) / 2;
            double max = ((double)_totalTime + delta) / 2;
            return (long)(Math.floor(max) - Math.ceil(min) + 1.0);
        }

        @Override
        public String toString() {
            return "Race("+_totalTime+","+_minWinningDistance+")";
        }
    }


    private ArrayList<Race> GeneratePart1Input()
    {
        var races = new ArrayList<Race>();
        var tmp = input.split("\n");
        var times = tmp[0].split(" ");
        var distances = tmp[1].split(" ");
        var tI = 1;
        var dI = 1;
        while (tI < times.length)
        {
            if (times[tI].isEmpty())
            {
                tI += 1;
                continue;
            }
            if (distances[dI].isEmpty())
            {
                dI += 1;
                continue;
            }
            var time = Long.parseLong(times[tI]);
            var distance = Long.parseLong(distances[dI]);
            races.add(new Race(time,distance));
            tI += 1;
            dI += 1;
        }
        return races;
    }

    private Race GeneratePart2Input()
    {
        String[] tmp = input.split("\n");
        String time = tmp[0].split(":")[1].replace(" ","");
        String distance = tmp[1].split(":")[1].replace(" ","");
        long uTime = Long.parseLong(time);
        long uDistance = Long.parseLong(time);
        return new Race(uTime, uDistance);
    }

    private long SolvePart1()
    {
        var races = GeneratePart1Input();
        long total = 1;
        for (var race : races)
        {
            total *= race.GetWinningStratsCount();
        }

        return total;
    }

    private long SolvePart2()
    {
        return GeneratePart2Input().GetWinningStratsCount();
    }

    public static void Solve(String[] args)
    {
        var self = new Day6();
        System.out.println("Solving part1");
        var start = System.nanoTime();
        var sol1 = self.SolvePart1();
        var time1 = System.nanoTime() - start;
        System.out.println("Solving part2");
        start = System.nanoTime();
        var sol2 = self.SolvePart2();
        var time2 = System.nanoTime() - start;
        System.out.printf("Solution 1: %s [%dus]\n",sol1,time1/1000);
        System.out.printf("Solution 2: %s [%dus]\n",sol2,time2/1000);
    }

}