#include <string>
#include <cstdint>
#include <cmath>
#include <chrono>
#include <vector>
#include <iostream>

static const std::string input = "Time:        44     80     65     72\nDistance:   208   1581   1050   1102";

class Race
{
private:
    uint64_t _totalTime;
    uint64_t _minWinningDistance;

public:
    Race(uint64_t totalTime, uint64_t distance) : _totalTime(totalTime), _minWinningDistance(distance + 1)
    {
    }

    uint64_t GetWinningStratsCount() const
    {
        double delta = sqrt((_totalTime * _totalTime) - 4.0 * _minWinningDistance);
        double min = ((double)_totalTime - delta) / 2.0;
        double max = ((double)_totalTime + delta) / 2.0;
        return (uint64_t)(floor(max) - ceil(min) + 1.0);
    }

    std::string ToString() const
    {
        return "Race(" + std::to_string(_totalTime) + "," + std::to_string(_minWinningDistance) + ")";
    }
};

class Day6
{
private:
    static bool IsDigit(char c)
    {
        return c >= '0' && c <= '9';
    }

    std::vector<Race> GeneratePart1Input()
    {
        std::vector<Race> races;
        std::vector<uint64_t> times;
        std::vector<uint64_t> distances;

        std::string buffer;
        size_t i = 0;

        while (!IsDigit(input[i]))
        {
            i++;
        }
        while (input[i] != '\n')
        {
            while (IsDigit(input[i]))
            {
                buffer.push_back(input[i]);
                i++;
            }
            if (!buffer.empty())
            {
                uint64_t num = std::stoull(buffer);
                times.push_back(num);
                buffer.clear();
                i--;
            }
            i++;
        }
        if (!buffer.empty())
        {
            uint64_t num = std::stoull(buffer);
            times.push_back(num);
            buffer.clear();
        }
        while (i < input.length())
        {
            while (IsDigit(input[i]))
            {
                buffer.push_back(input[i]);
                i++;
            }
            if (!buffer.empty())
            {
                uint64_t num = std::stoull(buffer);
                distances.push_back(num);
                buffer.clear();
                i--;
            }
            i++;
        }
        if (!buffer.empty())
        {
            uint64_t num = std::stoull(buffer);
            times.push_back(num);
            buffer.clear();
        }
        for (size_t j = 0; j < times.size(); j++)
        {
            races.emplace_back(times[j], distances[j]);
        }
        return races;
    }

    Race GeneratePart2Input()
    {
        uint64_t time;
        uint64_t distance;

        std::string buffer;
        size_t i = 0;

        while (!IsDigit(input[i]))
        {
            i++;
        }
        while (input[i] != '\n')
        {
            if (IsDigit(input[i]))
            {
                buffer.push_back(input[i]);
            }
            i++;
        }
        time = std::stoull(buffer);
        buffer.clear();
        while (i < input.length())
        {
            if (IsDigit(input[i]))
            {
                buffer.push_back(input[i]);
            }
            i++;
        }
        distance = std::stoull(buffer);
        buffer.clear();

        return {time, distance};
    }

    uint64_t SolvePart1()
    {
        auto races = GeneratePart1Input();
        uint64_t total = 1;
        for (auto race : races)
        {
            total *= race.GetWinningStratsCount();
        }

        return total;
    }

    uint64_t SolvePart2()
    {
        return GeneratePart2Input().GetWinningStratsCount();
    }

public:
    static void Main()
    {
        Day6 self = Day6();
        std::cout << "Solving part1" << std::endl;
        auto start = std::chrono::system_clock::now();
        uint64_t sol1 = self.SolvePart1();
        auto time1 = std::chrono::system_clock::now() - start;
        std::cout << "Solving part1" << std::endl;
        start = std::chrono::system_clock::now();
        uint64_t sol2 = self.SolvePart2();
        auto time2 = std::chrono::system_clock::now() - start;
        std::cout << "Solution 1: " << sol1 << " [" << time1.count() / 1000 << "us]" << std::endl;
        std::cout << "Solution 2: " << sol2 << " [" << time2.count() / 1000 << "us]" << std::endl;
    }
};

int main()
{
    Day6::Main();
    return 0;
}