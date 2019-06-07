namespace IncrementalSociety.Population
{
	public class PopUnits
	{
		public double PopMin { get; }

		public PopUnits (double popMin)
		{
			PopMin = popMin;
		}

		public double GetPopUnitsForTotalPopulation (double population)
		{
			if (population < 1000) {
				return population / 100;
			} else if (population < 2000) {
				return 10 + (population - 1000) / 200;
			} else if (population < 4000) {
				return 15 + (population - 2000) / 500;
			} else if (population < 10000) {
				return 19 + (population - 4000) / 1000;
			} else if (population < 50000) {
				return 25 + (population - 10000) / 5000;
			} else if (population< 100000) {
				return 32 + (population - 50000) / 10000;
			} else {
				return 37 + (population - 100000) / 50000;
			}
		}

		public double GetNextPopBreakpoint (double current)
		{
			if (current < 1000)
				return current + 100;
			else if (current < 2000)
				return current + 200;
			else if (current < 4000)
				return current + 500;
			else if (current < 10000)
				return current + 1000;
			else if (current < 50000)
				return current + 5000;
			else if (current < 100000)
				return current + 10000;
			else
				return current + 50000;
		}

		public double GetPreviousPopBreakpoint (double current)
		{
			if (current == PopMin)
				return current;

			if (current <= 1000)
				return current - 100;
			else if (current <= 2000)
				return current - 200;
			else if (current <= 4000)
				return current - 500;
			else if (current <= 10000)
				return current - 1000;
			else if (current <= 50000)
				return current - 5000;
			else if (current <= 100000)
				return current - 10000;
			else
				return current - 50000;
		}
	}
}