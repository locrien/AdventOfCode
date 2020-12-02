using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
    class Day1Challenge : Challenge
    {
		private Dictionary<long, int> entryCounts = new Dictionary<long, int>();

		protected override string Part1()
        {
            string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input1.txt");
            string[] entries = entry.Split('\n');
			int[] numbers = Array.ConvertAll(entries, int.Parse);

			for(int i = 0; i < numbers.Length; ++i)
			{
				for (int j = i+1; j < numbers.Length; ++j)
				{
					if (numbers[i] + numbers[j] == 2020)
					{
						return (numbers[i] * numbers[j]).ToString();
					}
				}
			}

            return "";
        }

		protected override string Part2()
        {
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input1.txt");
			string[] entries = entry.Split('\n');
			int[] numbers = Array.ConvertAll(entries, int.Parse);

			for (int i = 0; i < numbers.Length; ++i)
			{
				for (int j = i + 1; j < numbers.Length; ++j)
				{
					for (int k = j + 1; k < numbers.Length; ++k)
					{
						if (numbers[i] + numbers[j] + numbers[k] == 2020)
						{
							return (numbers[i] * numbers[j] * numbers[k]).ToString();
						}
					}
				}
			}

			return "";
		}
    }
}
