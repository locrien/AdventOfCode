using AdventCalendar.Challenges;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar
{
    abstract class Challenge
    {
        public virtual int DefaultPart
		{
			get
			{
				return 1;
			}
		}

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
                    challenge = new Day1Challenge();
                    break;
                case 5:
                    challenge = new Day1Challenge();
                    break;
                case 6:
                    challenge = new Day1Challenge();
                    break;
                case 7:
                    challenge = new Day1Challenge();
                    break;
                case 8:
                    challenge = new Day1Challenge();
                    break;
                case 9:
                    challenge = new Day1Challenge();
                    break;
                case 10:
                    challenge = new Day1Challenge();
                    break;
                case 11:
                    challenge = new Day1Challenge();
                    break;
                case 12:
                    challenge = new Day1Challenge();
                    break;
                case 13:
                    challenge = new Day1Challenge();
                    break;
                case 14:
                    challenge = new Day1Challenge();
                    break;
                case 15:
                    challenge = new Day1Challenge();
                    break;
                case 16:
                    challenge = new Day1Challenge();
                    break;
                case 17:
                    challenge = new Day1Challenge();
                    break;
                case 18:
                    challenge = new Day1Challenge();
                    break;
                case 19:
                    challenge = new Day1Challenge();
                    break;
                case 20:
                    challenge = new Day1Challenge();
                    break;
                case 21:
                    challenge = new Day1Challenge();
                    break;
                case 22:
                    challenge = new Day1Challenge();
                    break;
                case 23:
                    challenge = new Day1Challenge();
                    break;
                case 24:
                    challenge = new Day1Challenge();
                    break;
                case 25:
                    challenge = new Day1Challenge();
                    break;
            }

            return challenge;
        }

        public string Execute(int part)
		{
			if (part == 1)
			{
				return Part1();
			}
			else if (part == 2)
			{
				return Part2();
			}

			return "Wrong Part Number";
		}

		protected abstract string Part1();
		protected abstract string Part2();
    }
}
