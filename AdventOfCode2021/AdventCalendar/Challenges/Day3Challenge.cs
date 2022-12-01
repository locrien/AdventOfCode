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
			string[] entries = input.Replace("\r","").Split('\n');

			var bitCount = entries[0].Length;
			var gammaValue = 0;
			var epsilonValue = 0;
			(var gamma, var epsilon) = GetGammaAndEpsilon(entries);

			for (int i = 0; i < gamma.Length; ++i)
			{
				var bitVal = (int)(Math.Pow(2, (bitCount - i - 1)));
				gammaValue += gamma[i] * bitVal;
				epsilonValue += epsilon[i] * bitVal;
			}

			return (gammaValue * epsilonValue).ToString();
		}
		
		private (int[], int[]) GetGammaAndEpsilon(string[] entries)
		{
			var bitCount = entries[0].Length;

			var gamma = new int[bitCount];
			var epsilon = new int[bitCount];
			
			for (int i = 0; i < bitCount; ++i)
			{
				foreach (var entry in entries)
				{
					var bitField = entry.ToCharArray();
					var bitString = bitField[i].ToString();
					var bitVal = int.Parse(bitString);
					if (bitVal == 0)
					{
						gamma[i]--;
						epsilon[i]--;
					}
					else
					{
						gamma[i]++;
						epsilon[i]++;
					}
				}
			}

			for (int i = 0; i < gamma.Length; ++i)
			{
				gamma[i] = gamma[i] > 0 ? 1 : 0;
				epsilon[i] = epsilon[i] > 0 ? 0 : 1;
			}

			return (gamma, epsilon);
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");
			string[] entries = input.Replace("\r", "").Split('\n');
			
			var bitCount = entries[0].Length;

			var remainingOxygenEntries = new List<string>(entries);
			
			for(int bit = 0; bit < bitCount;++bit)
			{
				var valCheck = 0;
				for (int i = remainingOxygenEntries.Count - 1; i >= 0; i--)
				{
					valCheck += int.Parse(remainingOxygenEntries[i].ToCharArray()[bit].ToString()) == 1 ? 1 : -1;
				}

				for (int i = remainingOxygenEntries.Count - 1; i >= 0; i--)
				{
					if(int.Parse(remainingOxygenEntries[i].ToCharArray()[bit].ToString()) == (valCheck >= 0 ? 1 : 0))
					{
						remainingOxygenEntries.RemoveAt(i);
					}
				}

				if(remainingOxygenEntries.Count <= 1)
				{
					break;
				}
			}

			var gamma = remainingOxygenEntries[0];
			var gammaValue = 0;
			for (int i = 0; i < gamma.Length; ++i)
			{
				var bitVal = (int)(Math.Pow(2, (bitCount - i - 1)));
				gammaValue += gamma[i] * bitVal;
			}


			return "";
		}
	}
}
