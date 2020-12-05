using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day5Challenge : Challenge
	{
		private const int _kRows = 128;
		private const int _kColumns = 8;

		private bool[] _seats = new bool[1024];

		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input5.txt");
			string[] entries = input.Split(new string[] { "\r\n" },StringSplitOptions.RemoveEmptyEntries);

			int row = 0;
			int column = 0;
			// test value
			(row, column) = CalculatePosition("FBFBBFFRLR", _kRows, _kColumns);

			int highestId = 0;
			foreach(var entry in entries)
			{
				(row, column) = CalculatePosition(entry,_kRows,_kColumns);

				int id = row * 8 + column;
				_seats[id] = true;

				if (id > highestId)
				{
					highestId = id;
				}
			}
			return highestId.ToString();
		}

		private (int, int) CalculatePosition(string seatCode, int rows, int columns)
		{
			int minRow = 0;
			int maxRow = rows - 1;
			int minColumn = 0;
			int maxColumn = columns - 1;

			foreach (var letter in seatCode)
			{
				switch (letter)
				{
					case 'F':
						maxRow -= (int)Math.Ceiling((maxRow - minRow) / 2f);
						break;
					case 'B':
						minRow += (int)Math.Ceiling((maxRow - minRow) / 2f);
						break;
					case 'L':
						maxColumn -= (int)Math.Ceiling((maxColumn - minColumn) / 2f);
						break;
					case 'R':
						minColumn += (int)Math.Ceiling((maxColumn - minColumn) / 2f);
						break;
				}
			}

			if(minColumn != maxColumn || minRow != maxRow)
			{
				throw new Exception("Seat not found. Invalid seat code.");
			}

			return (minRow, minColumn);
		}
		
		protected override string Part2()
		{
			Part1();

			for(int i = 1; i < _seats.Length - 1; ++i)
			{
				if(!_seats[i] && _seats[i+1] && _seats[i - 1])
				{
					return i.ToString();
				}
			}

			return (-1).ToString();
		}
	}
}
