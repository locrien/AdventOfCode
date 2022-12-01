using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace AdventCalendar.Challenges
{

	class Day4Challenge : Challenge
	{
		private class Board
		{
			private class Space
			{
				public int Value;
				public bool Marked;
			}

			private List<List<Space>> Spaces;

			public Board(string entryString)
			{
				Spaces = new List<List<Space>>();
				var entries = entryString.Split(new string[] { "\r\n" }, StringSplitOptions.RemoveEmptyEntries);

				foreach(var entryLine in entries)
				{
					var lineValues = entryLine.Split(new string[] { "\r\n", " ", "\r", "\n" }, StringSplitOptions.RemoveEmptyEntries);
					var column = new List<Space>();
					foreach(var entry in lineValues)
					{
						column.Add(new Space() { Marked = false, Value = int.Parse(entry) });
					}
					Spaces.Add(column);
				}
			}

			public void Mark(int number)
			{
				for(int i = 0; i < Spaces.Count;++i)
				{
					for(int j = 0;j<Spaces[i].Count;++j)
					{
						if(Spaces[i][j].Value == number)
						{
							Spaces[i][j].Marked = true;
						}
					}
				}
			}

			public int SumUnmarked()
			{
				var sum = 0;
				for (int i = 0; i < Spaces.Count; ++i)
				{
					for (int j = 0; j < Spaces[i].Count; ++j)
					{
						if (!Spaces[i][j].Marked)
						{
							sum += Spaces[i][j].Value;
						}
					}
				}
				return sum;
			}

			public bool IsWinner()
			{
				bool isWinner = true;
				// check lines
				for (int i = 0; i < Spaces.Count; ++i)
				{
					isWinner = true;
					for (int j = 0; j < Spaces[i].Count; ++j)
					{
						if(!Spaces[i][j].Marked)
						{
							isWinner = false;
							break;
						}
					}
					if(isWinner)
					{
						return true;
					}
				}

				// check columns
				for (int i = 0; i < Spaces.Count; ++i)
				{
					isWinner = true;
					for (int j = 0; j < Spaces[i].Count; ++j)
					{
						if (!Spaces[j][i].Marked)
						{
							isWinner = false;
							break;
						}
					}
					if (isWinner)
					{
						return true;
					}
				}

				return false;
			}
		}
		
		protected override string Part1()
		{
			string entryText = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input4.txt");
			var entries = entryText.Split(new string[]{ "\n\r"}, StringSplitOptions.RemoveEmptyEntries);

			var numbersCalled = entries[0].Replace("\n"," ").Replace("\r", " ").Split(',');

			var boards = new List<Board>();
			for(var i = 1; i<entries.Length;++i)
			{
				boards.Add(new Board(entries[i]));
			}

			foreach(var numberCalled in numbersCalled)
			{
				foreach(var board in boards)
				{
					board.Mark(int.Parse(numberCalled));
					if(board.IsWinner())
					{
						return (int.Parse(numberCalled) * board.SumUnmarked()).ToString();
					}
				}
			}

			return "failed";
		}

		protected override string Part2()
		{
			string entryText = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input4.txt");
			var entries = entryText.Split(new string[] { "\n\r" }, StringSplitOptions.RemoveEmptyEntries);

			var numbersCalled = entries[0].Replace("\n", " ").Replace("\r", " ").Split(',');

			var boards = new List<Board>();
			for (var i = 1; i < entries.Length; ++i)
			{
				boards.Add(new Board(entries[i]));
			}

			
			foreach (var numberCalled in numbersCalled)
			{
				List<Board> boardsToRemove = new List<Board>();
				foreach (var board in boards)
				{
					board.Mark(int.Parse(numberCalled));
					if (board.IsWinner())
					{
						if(boards.Count == 1)
						{
							return (int.Parse(numberCalled) * board.SumUnmarked()).ToString();
						}
						else
						{
							boardsToRemove.Add(board);
						}
					}
				}

				foreach(var boardToRemove in boardsToRemove)
				{
					boards.Remove(boardToRemove);
				}
				boardsToRemove.Clear();
			}

			return "failed";
		}
	}
}
