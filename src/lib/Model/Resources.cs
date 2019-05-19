using System;
using System.Collections.Generic;
using System.Linq;

namespace IncrementalSociety
{
	public class ResourceConfig
	{
		public int ResourceLength;
		public Dictionary <string, int> Index;

		public ResourceConfig (List<string> resources)
		{
			ResourceLength = resources.Count;
			Index = new Dictionary<string, int> ();
			for (int i = 0 ; i < ResourceLength ; ++i)
				Index.Add (resources[i], i);
		}

		public Resources Create () => new Resources (this);
	}

	public class Resources
	{
		ResourceConfig Config;
		double [] Inventory;

		int ResourceLength => Config.ResourceLength;
		Dictionary <string, int> Index => Config.Index;

		public Resources (ResourceConfig config)
		{
			Config = config;
			Inventory = new double [ResourceLength];
		}

		protected Resources (ResourceConfig config, double [] inventory)
		{
			Config = config;
			Inventory = new double [ResourceLength];
			Array.Copy (inventory, Inventory, ResourceLength);
		}

		public virtual double this [string key]
		{
			get => Inventory [Index[key]];
		}

		public Buidler ToBuilder ()
		{
			return new Buidler (Config, Inventory);
		}

		public class Buidler : Resources
		{
			internal Buidler (ResourceConfig config, double [] inventory) : base (config, inventory) {}

			public new double this [string key]
			{
				get => Inventory [Index[key]];
				set => Inventory [Index[key]] = value;
			}

			public Resources ToResources ()
			{
				return new Resources (Config, Inventory);
			}
		}
	}
}
