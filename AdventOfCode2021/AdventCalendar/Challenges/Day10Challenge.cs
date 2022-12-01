using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day10Challenge : Challenge
	{
		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input10.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			int[] adapters = Array.ConvertAll<string, int>(entries, new Converter<string,int>(StringToInt));

			Array.Sort(adapters);

			int[] differences = new int[5];

			differences[3]++;
			for (int i = adapters.Length - 2; i >= 0; --i)
			{
				differences[adapters[i + 1] - adapters[i]]++;
			}

			differences[adapters[0] - 0]++;

			return (differences[1] * differences[3]).ToString();
		}

		private int StringToInt(string input)
		{
			return int.Parse(input);
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input10.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			int[] adapters = Array.ConvertAll<string, int>(entries, new Converter<string, int>(StringToInt));

			Array.Sort(adapters);

			List<int> adaptersList = new List<int>(adapters);
			adaptersList.Insert(0, 0);
			adaptersList.Insert(adaptersList.Count, adaptersList[adaptersList.Count - 1] + 3);


			List<int> nonRemovables = new List<int>();

			for(int i = 1; i< adaptersList.Count-1;++i)
			{
				if(adaptersList[i+1] - adaptersList[i] == 3 ||   adaptersList[i-1] - adaptersList[i] == 3)
				{
					nonRemovables.Add(i-1);
				}
			}

			List<int> adaptersList2 = new List<int>(adapters);

			// first chunk 0 to nonRemovables[0]
			var count = PermutationCount(adaptersList2,0, nonRemovables[0]);

			for (int i = 0; i < nonRemovables.Count-1;++i)
			{
				count += PermutationCount(adaptersList2, nonRemovables[i] + 1, nonRemovables[i + 1]);
			}

			// last chunk nonRemovables[nonRemovables.Count-1] to adaptersList2.Count
			count += PermutationCount(adaptersList2, nonRemovables[nonRemovables.Count - 1] + 1, adaptersList2.Count);

			return "";
		}

		private long PermutationCount(List<int> adapterList, int from, int to)
		{
			var chunk = adapterList.GetRange(from, to - from);

			return 0;
		}

		private long Factorial(long n)
		{
			long res = 1;
			while (n != 1)
			{
				res = res * n;
				n = n - 1;
			}
			return res;
		}
	}
}
