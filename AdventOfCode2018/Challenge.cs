using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar
{
    abstract class Challenge
    {
        public static Challenge Create(int day)
        {
            Challenge challenge = null;
            switch (day)
            {
                case 1:
                    challenge = new Day1Challenge();
                    break;
                case 2:
                    challenge = new Day2Challenge();
                    break;
                case 3:
                    challenge = new Day3Challenge();
                    break;
                case 4:
                    challenge = new Day4Challenge();
                    break;
                case 5:
                    challenge = new Day5Challenge();
                    break;
                case 6:
                    challenge = new Day6Challenge();
                    break;
                case 7:
                    challenge = new Day7Challenge();
                    break;
                case 8:
                    challenge = new Day8Challenge();
                    break;
                case 9:
                    challenge = new Day9Challenge();
                    break;
                case 10:
                    challenge = new Day10Challenge();
                    break;
                case 11:
                    challenge = new Day11Challenge();
                    break;
                case 12:
                    challenge = new Day12Challenge();
                    break;
                case 13:
                    challenge = new Day13Challenge();
                    break;
                case 14:
                    challenge = new Day14Challenge();
                    break;
                case 15:
                    challenge = new Day15Challenge();
                    break;
                case 16:
                    challenge = new Day16Challenge();
                    break;
                case 17:
                    challenge = new Day17Challenge();
                    break;
                case 18:
                    challenge = new Day18Challenge();
                    break;
                case 19:
                    challenge = new Day19Challenge();
                    break;
                case 20:
                    challenge = new Day20Challenge();
                    break;
                case 21:
                    challenge = new Day21Challenge();
                    break;
                case 22:
                    challenge = new Day22Challenge();
                    break;
                case 23:
                    challenge = new Day23Challenge();
                    break;
                case 24:
                    challenge = new Day24Challenge();
                    break;
                case 25:
                    challenge = new Day25Challenge();
                    break;
            }

            return challenge;
        }

        public abstract string Execute(int part);
    }
}
