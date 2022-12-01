using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day3Challenge : Challenge
	{
		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");

			var repeatWidth = input.IndexOf('\r');
			var positions = input.ToCharArray();

			var collisionCount = CalculateCollisions(3,1, positions, repeatWidth);

			return collisionCount.ToString();
		}

		private int CalculateCollisions(int slopeX, int slopeY, char[] positions, int repeatWidth)
		{
			var x = 0;
			var y = 0;
			
			var index = 0;

			var collisionCount = 0;

			while (index < positions.Length - 1)
			{
				var element = positions[index];
				switch (element)
				{
					case '#':
						collisionCount++;
						break;
				}

				x += slopeX;
				y += slopeY;

				index = (x % (repeatWidth)) + (y * (repeatWidth + 2));
			}

			return collisionCount;
		}
		
		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");

			var repeatWidth = input.IndexOf('\r');
			var positions = input.ToCharArray();

			long count = CalculateCollisions(1, 1, positions, repeatWidth);
			count *= CalculateCollisions(3, 1, positions, repeatWidth);
			count *= CalculateCollisions(5, 1, positions, repeatWidth);
			count *= CalculateCollisions(7, 1, positions, repeatWidth);
			count *= CalculateCollisions(1, 2, positions, repeatWidth);
			
			return count.ToString();
		}
	}
}
