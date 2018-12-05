using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day4Challenge : Challenge
	{
		private Dictionary<int, Guard> _guards = new Dictionary<int, Guard>();

		class Guard
		{
			public int Id;
			public int[] SleepMinutes;
		}

		class Record : IComparable<Record>
		{
			public DateTime Timestamp;
			public int GuardId = -1;
			public bool Awake;

			public int CompareTo(Record other)
			{
				return DateTime.Compare(Timestamp,other.Timestamp);
			}
		}

		public override int DefaultPart
		{
			get
			{
				return 1;
			}
		}

		protected override string Part1()
		{
			string entryText = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input4.txt");
			string[] entries = entryText.Split('\n');

			var sortedRecords = new List<Record>();

			string sleepCommand = "falls asleep";
			string wakeCommand = "wakes up";

			foreach (var entry in entries)
			{
				// parse
				var separatorSpaceIdx = entry.IndexOf(']') + 1;
				var leftPart = entry.Substring(0, separatorSpaceIdx);
				var rightPart = entry.Substring(separatorSpaceIdx+1);

				var expression = new Regex("\\[(?<Year>[0-9]+)-(?<Month>[0-9]+)-(?<Day>[0-9]+) (?<Hour>[0-9]+):(?<Minute>[0-9]+)\\]");
				var line = leftPart;
				var match = expression.Match(line);

				int year = int.Parse(match.Groups["Year"].Value);
				int Month = int.Parse(match.Groups["Month"].Value);
				int Day = int.Parse(match.Groups["Day"].Value);
				int Hour = int.Parse(match.Groups["Hour"].Value);
				int Minute = int.Parse(match.Groups["Minute"].Value);
				DateTime dt = new DateTime(year, Month, Day, Hour, Minute, 0);

				var record = new Record();
				record.Timestamp = dt;
				if(rightPart.Equals(wakeCommand,StringComparison.Ordinal))
				{
					record.Awake = true;
				}
				else if(rightPart.Equals(sleepCommand,StringComparison.Ordinal))
				{
					record.Awake = false;
				}
				else
				{
					var rightExpr = new Regex(@"Guard #(?<Id>[0-9]+) begins shift");
					var rightMatch = rightExpr.Match(rightPart);

					record.GuardId = int.Parse(rightMatch.Groups["Id"].Value);
				}

				// sort the events by date
				sortedRecords.Add(record);


				// find the guard with the most sleep minutes

				// array with 60 for sleeptime per minute, add for total
			}

			sortedRecords.Sort();

			foreach(var record in sortedRecords)
			{
				if(record.GuardId >= 0)
				{
					Guard currentGuard = new Guard();
				}
				
			}

			return "";
		}

		protected override string Part2()
		{
			return "";
		}
	}
}
