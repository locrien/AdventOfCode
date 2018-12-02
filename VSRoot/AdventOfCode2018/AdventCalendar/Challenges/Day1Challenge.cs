using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
    class Day1Challenge : Challenge
    {
		public override int DefaultPart
		{
			get
			{
				return 2;
			}
		}

		private Dictionary<long, int> entryCounts = new Dictionary<long, int>();

		protected override string Part1()
        {
            string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input1.txt");
            string[] numbers = entry.Split('\n');

            int sum = 0;

            for (int i = 0; i < numbers.Length; ++i)
            {
                sum += int.Parse(numbers[i]);
            }

            return sum.ToString();
        }

		protected override string Part2()
        {
            string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input1.txt");

            int sum = 0;
            string[] numbers = entry.Split('\n');

            for (int i = 0; i < numbers.Length; ++i)
            {
                var number = numbers[i];
                    sum += int.Parse(number);

                    if (entryCounts.ContainsKey(sum))
                    {
                        return sum.ToString();
                    }
                    else
                    {
                        entryCounts[sum] = 1;
                    }

                if (i == numbers.Length - 1)
                {
                    i = -1;
                }
            }

            return sum.ToString();
        }
    }
}
