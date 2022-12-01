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

			var timesIncreased = 0;
			for(int i = 1; i < numbers.Length; ++i)
			{
				if(numbers[i] > numbers[i-1])
				{
					timesIncreased++;
				}
			}

			return timesIncreased.ToString() ;
        }

		protected override string Part2()
        {
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input1.txt");
			string[] entries = entry.Split('\n');
			int[] numbers = Array.ConvertAll(entries, int.Parse);

			var timesIncreased = 0;
			for (int i = 3; i < numbers.Length; ++i)
			{
				var window1 = numbers[i] + numbers[i - 1] + numbers[i - 2];
				var window2 = numbers[i-1] + numbers[i - 2] + numbers[i - 3];
				if (window1 > window2)
				{
					timesIncreased++;
				}
			}

			return timesIncreased.ToString();
		}
    }
}
