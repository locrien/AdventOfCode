using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day8Challenge : Challenge
	{
		public override int DefaultPart
		{
			get
			{
				return 1;
			}
		}

		protected override string Part1()
		{
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");
			string[] ids = entry.Split('\n');
			
			return "";
		}

		protected override string Part2()
		{
			return "";
		}
	}
}
