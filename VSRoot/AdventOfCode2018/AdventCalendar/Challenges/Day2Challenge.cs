using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day2Challenge : Challenge
	{
		public override int DefaultPart
		{
			get
			{
				return 2;
			}
		}

		protected override string Part1()
		{
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input2.txt");
			string[] ids = entry.Split('\n');

			int twoCounts = 0;
			int threeCounts = 0;
			var characterCounts = new Dictionary<char, int>();

			foreach(var id in ids)
			{
				characterCounts.Clear();

				foreach (var c in id)
				{
					if(!characterCounts.ContainsKey(c))
					{
						characterCounts.Add(c,0);
					}
					characterCounts[c]++;
				}

				bool has2Count = false;
				bool has3Count = false;
				foreach(var charCount in characterCounts)
				{
					if(!has2Count && charCount.Value == 2)
					{
						twoCounts++;
						has2Count = true;
						if (has3Count)
						{
							break;
						}
					}
					else if(!has3Count && charCount.Value == 3)
					{
						threeCounts++;
						has3Count = true;
						if (has2Count)
						{
							break;
						}
					}
				}
			}

			return (twoCounts * threeCounts).ToString();
		}

		protected override string Part2()
		{
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input2.txt");
			string[] ids = entry.Split('\n');

			// find the ids with 1 difference, return the same part.
			for(int i = 0;i<ids.Length;++i)
			{
				for(int j = i;j < ids.Length;++j)
				{
					int idx = CompareIds(ids[i],ids[j]);
					if(idx != -1)
					{
						return ids[i].Remove(idx, 1);
					}
				}
			}

			return "NOT FOUND";
		}

		/// <summary>
		/// Returns -1 if there are not 1 differences. Returns the index of the difference if a single one is found.
		/// </summary>
		/// <param name="id1"></param>
		/// <param name="id2"></param>
		/// <returns></returns>
		private int CompareIds(string id1, string id2)
		{
			int index = -1;
			int differences = 0;
			for(int i = 0;i<id1.Length;++i)
			{
				if(id1[i] != id2[i])
				{
					index = i;
					differences++;

					if(differences > 1)
					{
						return -1;
					}
				}
			}
			
			return index;
		}
	}
}
