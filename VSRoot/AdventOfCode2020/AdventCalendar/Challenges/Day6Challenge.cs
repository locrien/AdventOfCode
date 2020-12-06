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
			string[] groups = input.Split(new string[] { Environment.NewLine + Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			int questionSum = 0;
			var questions = new HashSet<char>();
			foreach(var group in groups)
			{
				questions.Clear();
				string[] people = group.Split(new string[] { "\r\n" }, StringSplitOptions.RemoveEmptyEntries);
				foreach(var person in people)
				{
					foreach(var question in person)
					{
						if(!questions.Contains(question))
						{
							questions.Add(question);
						}
					}
				}

				questionSum += questions.Count;
			}

			return questionSum.ToString();
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input6.txt");
			string[] groups = input.Split(new string[] { Environment.NewLine + Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			int questionSum = 0;
			var questions = new Dictionary<char,int>();
			foreach (var group in groups)
			{
				questions.Clear();
				string[] people = group.Split(new string[] { "\r\n" }, StringSplitOptions.RemoveEmptyEntries);
				foreach (var person in people)
				{
					foreach (var question in person)
					{
						if (!questions.ContainsKey(question))
						{
							questions.Add(question,1);
						}
						else
						{
							questions[question]++;
						}
					}
				}

				foreach (var question in questions.Keys)
				{
					if(questions[question] == people.Length)
					{
						questionSum ++;
					}
				}
			}

			return questionSum.ToString();
		}
	}
}
