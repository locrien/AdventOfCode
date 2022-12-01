using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day2Challenge : Challenge
	{
		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input2.txt");
			string[] entries = input.Split('\n');

			int xPos = 0;
			int depth = 0;

			foreach (var entry in entries)
			{
				var splitEntry = entry.Split(' ');
				var direction = splitEntry[0];
				var distance = int.Parse(splitEntry[1]);

				switch (direction)
				{
					case "forward":
						xPos += distance;
						break;
					case "down":
						depth += distance;
						break;
					case "up":
						depth -= distance;
						depth = depth < 0 ? 0 : depth;
						break;
				}
			}

			return (xPos * depth).ToString();
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input2.txt");
			string[] entries = input.Split('\n');

			int xPos = 0;
			int depth = 0;
			int aim = 0;

			foreach (var entry in entries)
			{
				var splitEntry = entry.Split(' ');
				var direction = splitEntry[0];
				var distance = int.Parse(splitEntry[1]);

				switch (direction)
				{
					case "forward":
						xPos += distance;
						depth += aim * distance;
						break;
					case "down":
						aim += distance;
						break;
					case "up":
						aim -= distance;
						break;
				}
			}

			return (xPos * depth).ToString();
		}
	}
}
