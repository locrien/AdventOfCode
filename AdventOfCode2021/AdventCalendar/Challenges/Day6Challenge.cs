using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day6Challenge : Challenge
	{


		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input6.txt");
			string[] initialFish = input.Split(new string[] {","}, StringSplitOptions.RemoveEmptyEntries);
			int[] fishInt = Array.ConvertAll(initialFish, int.Parse);

			var currentFish = new List<int>(fishInt);

			int days = 256;

			for(int i =0;i<days;++i)
			{
				for(int f= currentFish.Count-1;f>=0;f--)
				{
					currentFish[f]--;
					if(currentFish[f] < 0)
					{
						currentFish.Add(8);
						currentFish[f] = 6;
					}
				}
			}

			return currentFish.Count.ToString();
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input6.txt");
			int[] initialFish = Array.ConvertAll(input.Split(new string[] { "," }, StringSplitOptions.RemoveEmptyEntries), int.Parse);

			List<long> fishWithLife = new List<long>();

			for (int fishLife = 0; fishLife < 9; fishLife++)
			{
				var amountFishWithLife = initialFish.Where((int life) => { return life == fishLife; });
				fishWithLife.Add(amountFishWithLife.Count());
			}
			
			for (int i = 0; i < 256; ++i)
			{
				var spawnerFish = fishWithLife[0];
				fishWithLife.RemoveAt(0);
				fishWithLife[6] += spawnerFish;
				fishWithLife.Add(spawnerFish);
			}

			long totalFish = 0;
			foreach(var fish in fishWithLife)
			{
				totalFish += fish;
			}

			return totalFish.ToString();
		}
	}
}
