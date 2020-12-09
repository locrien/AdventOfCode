using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day9Challenge : Challenge
	{
		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input9.txt");
			string[] entriesStr = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			List<long> entries = new List<long>();
			foreach(var entryStr in entriesStr)
			{
				entries.Add(long.Parse(entryStr));
			}
			
			int preamble = 25;
			for(int i = preamble; i < entries.Count;++i)
			{
				if(!IsValid(entries, i, preamble))
				{
					return entries[i].ToString();
				}
			}

			return "";
		}

		private bool IsValid(List<long> entries, int index, int preamble)
		{
			for (int i = index - preamble; i < index; ++i)
			{
				for (int j = i + 1; j < index; ++j)
				{ 
					if(entries[i] + entries[j] == entries[index])
					{
						return true;
					}
				}
			}

			return false;
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input9.txt");
			string[] entriesStr = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			List<long> entries = new List<long>();
			foreach (var entryStr in entriesStr)
			{
				entries.Add(long.Parse(entryStr));
			}

			int maxIndex = entriesStr.Length - 1;
			int minIndex = maxIndex - 2;

			long sum = 0;
			for(int i = minIndex; i<=maxIndex;++i)
			{
				sum += entries[i];
			}

			var target = long.Parse(Part1());
			while(sum != target)
			{
				if(sum > target && maxIndex - minIndex > 2)
				{
					sum -= entries[maxIndex];
					maxIndex--;
				}
				else
				{
					minIndex--;
					sum += entries[minIndex];
				}
			}

			long smallest = long.MaxValue;
			long biggest = 0;

			for (int i = minIndex; i <= maxIndex; ++i)
			{
				if(entries[i] < smallest)
				{
					smallest = entries[i];
				}

				if(entries[i] > biggest)
				{
					biggest = entries[i];
				}
			}

			return (smallest + biggest).ToString();
		}
	}
}
