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
		private struct Position
		{
			public int X;
			public int Y;

			public Position(int x, int y)
			{
				X = x;
				Y = y;
			}
		}

		protected override string Part1()
		{
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");
			string[] ids = entry.Split('\n');

			var claimedSpots = new Dictionary<Position, int>();

			// parse the data
			// format : #1 @ 916,616: 21x29

			// find the overlap between all the entries
			foreach(var id in ids)
			{
				int identifierIdx = id.IndexOf('#', 0);
				int positionIdx = id.IndexOf('@', identifierIdx + 1);
				int posSepIdx = id.IndexOf(',', positionIdx + 1);
				int areaIdx = id.IndexOf(':', posSepIdx + 1);
				int areaSepIdx = id.IndexOf('x', areaIdx + 1);

				int identifier = int.Parse(id.Substring(identifierIdx + 1, positionIdx - identifierIdx - 1));
				int posX = int.Parse(id.Substring(positionIdx + 1, posSepIdx - positionIdx - 1));
				int posY = int.Parse(id.Substring(posSepIdx + 1, areaIdx - posSepIdx - 1));
				int areaX = int.Parse(id.Substring(areaIdx + 1, areaSepIdx - areaIdx - 1));
				int areaY = int.Parse(id.Substring(areaSepIdx+1));

				for(int i = posX; i < posX + areaX;++i)
				{
					for(int j = posY;j<posY+areaY;++j)
					{
						var claimedPosition = new Position(i,j);
						if(!claimedSpots.ContainsKey(claimedPosition))
						{
							claimedSpots.Add(claimedPosition,1);
						}
						else
						{
							claimedSpots[claimedPosition]++;
						}
					}
				}
			}

			int overlapSpots = 0;
			foreach(var claimedSpot in claimedSpots)
			{
				if(claimedSpot.Value > 1)
				{
					overlapSpots++ ;
				}
			}

			return overlapSpots.ToString();
		}

		protected override string Part2()
		{
			string entry = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input3.txt");
			string[] ids = entry.Split('\n');

			var claimedSpots = new Dictionary<Position, List<int>>();

			// parse the data
			// format : #1 @ 916,616: 21x29
			var claims = new List<int>();

			// find the overlap between all the entries
			foreach (var id in ids)
			{
				int identifierIdx = id.IndexOf('#', 0);
				int positionIdx = id.IndexOf('@', identifierIdx + 1);
				int posSepIdx = id.IndexOf(',', positionIdx + 1);
				int areaIdx = id.IndexOf(':', posSepIdx + 1);
				int areaSepIdx = id.IndexOf('x', areaIdx + 1);

				int identifier = int.Parse(id.Substring(identifierIdx + 1, positionIdx - identifierIdx - 1));
				int posX = int.Parse(id.Substring(positionIdx + 1, posSepIdx - positionIdx - 1));
				int posY = int.Parse(id.Substring(posSepIdx + 1, areaIdx - posSepIdx - 1));
				int areaX = int.Parse(id.Substring(areaIdx + 1, areaSepIdx - areaIdx - 1));
				int areaY = int.Parse(id.Substring(areaSepIdx + 1));
				claims.Add(identifier);

				for (int i = posX; i < posX + areaX; ++i)
				{
					for (int j = posY; j < posY + areaY; ++j)
					{
						var claimedPosition = new Position(i, j);
						if (!claimedSpots.ContainsKey(claimedPosition))
						{
							var claimantsList = new List<int>();
							claimantsList.Add(identifier);
							claimedSpots.Add(claimedPosition, claimantsList);
						}
						else
						{
							claimedSpots[claimedPosition].Add(identifier);
							foreach(var claimant in claimedSpots[claimedPosition])
							{
								claims.Remove(claimant);
							}
						}
					}
				}
			}



			return claims[0].ToString() ;
		}
	}
}
