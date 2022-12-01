using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day5Challenge : Challenge
	{
		public class Line
		{

		}

		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input5.txt");
			string[] entries = input.Split('\n');

			var diagram = new Dictionary<(int, int), int>();

			foreach(var entry in entries)
			{
				var points = entry.Split(new string[] { " -> " }, StringSplitOptions.RemoveEmptyEntries);
				var startString = points[0].Split(',');
				var endString = points[1].Split(',');

				(int x, int y) start = (int.Parse(startString[0]), int.Parse(startString[1]));
				(int x, int y) end = (int.Parse(endString[0]), int.Parse(endString[1]));

				if (start.x == end.x || start.y == end.y)
				{
					var minX = Math.Min(start.x, end.x);
					var maxX = Math.Max(start.x, end.x);
					for (int i = minX; i<= maxX; ++i )
					{
						var minY = Math.Min(start.y, end.y);
						var maxY = Math.Max(start.y, end.y);
						for (int j = minY; j <= maxY; ++j)
						{
							if (diagram.ContainsKey((i, j)))
								diagram­[(i, j)]++;
							else
								diagram­.Add((i, j), 1);
						}
					}
				}
			}

			int numAtLeast2 = 0;
			foreach(var spot in diagram)
			{
				if(spot.Value >= 2)
				{
					numAtLeast2++;
				}
			}

			return numAtLeast2.ToString();
		}
		
		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input5.txt");
			string[] entries = input.Split('\n');

			var diagram = new Dictionary<(int, int), int>();

			foreach (var entry in entries)
			{
				var points = entry.Split(new string[] { " -> " }, StringSplitOptions.RemoveEmptyEntries);
				var startString = points[0].Split(',');
				var endString = points[1].Split(',');

				(int x, int y) start = (int.Parse(startString[0]), int.Parse(startString[1]));
				(int x, int y) end = (int.Parse(endString[0]), int.Parse(endString[1]));
				
				if (start.x == end.x || start.y == end.y || IsDiagonal(start, end))
				{
					int i = start.x;
					int j = start.y;

					var deltaX = end.x - start.x;
					deltaX = deltaX == 0 ? 0 : Math.Sign(deltaX);
					var deltaY = end.y - start.y;
					deltaY = deltaY == 0 ? 0 : Math.Sign(deltaY);

					var length = Math.Max(Math.Abs(end.x - start.x), Math.Abs(end.y - start.y))+1;

					for(int count=0;count< length;++count)
					{
						if (diagram.ContainsKey((i, j)))
							diagram­[(i, j)]++;
						else
							diagram­.Add((i, j), 1);

						i += deltaX;
						j += deltaY;
					}
				}
			}

			int numAtLeast2 = 0;
			foreach (var spot in diagram)
			{
				if (spot.Value >= 2)
				{
					numAtLeast2++;
				}
			}

			return numAtLeast2.ToString();
		}

		private bool IsDiagonal((int x, int y) start, (int x, int y) end)
		{
			var deltaX = Math.Abs(end.x - start.x);
			var deltaY = Math.Abs(end.y - start.y);

			return deltaX == deltaY;
		}
	}
}
