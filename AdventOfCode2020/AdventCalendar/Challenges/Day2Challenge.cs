using System;
using System.Collections.Generic;
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

			int totalSuccess = 0;
			string[] delimiters = new string[] { "-",": "," ","\r"};
			foreach(var entry in entries)
			{
				string[] result = entry.Split(delimiters, StringSplitOptions.RemoveEmptyEntries);

				int minCount = int.Parse(result[0]);
				int maxCount = int.Parse(result[1]);

				char letter = result[2].ToCharArray()[0];
				string password = result[3];

				int count = 0;
				foreach(var pass in password)
				{ 
					if(pass == letter)
					{
						count++;
					}
				}

				if(count >= minCount && count <= maxCount)
				{
					totalSuccess++;
				}
			}
			

			return totalSuccess.ToString();
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input2.txt");
			string[] entries = input.Split('\n');

			int totalSuccess = 0;
			string[] delimiters = new string[] { "-", ": ", " ", "\r" };
			foreach (var entry in entries)
			{
				string[] result = entry.Split(delimiters, StringSplitOptions.RemoveEmptyEntries);

				int firstPos = int.Parse(result[0]);
				int secondPos = int.Parse(result[1]);

				char letter = result[2].ToCharArray()[0];
				char[] password = result[3].ToCharArray();

				if (password[firstPos - 1] == letter ^ password[secondPos - 1] == letter)
				{
					totalSuccess++;
				}
			}


			return totalSuccess.ToString();
		}
	}
}
