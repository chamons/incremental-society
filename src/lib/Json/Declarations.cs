using System;
using System.Collections.Generic;
using System.Text;

namespace IncrementalSociety.Json
{
    public class ResourceDeclaration
    {
        public string Name { get; set; }
        public string Type { get; set; }
        public string Age { get; set; }
        public bool Image_has_age_prefix { get; set; }
    }

    public class ResourceDeclarations
    {
        public List<ResourceDeclaration> Resources { get; set; }
    }

    public class RegionDeclaration
    {
        public string Name { get; set; }
        public List<string> Resources { get; set; }
        public bool? Can_spawn { get; set; }
    }

    public class RegionDeclarations
    {
        public List<RegionDeclaration> Regions { get; set; }
    }

    public class PopulationNeeds
    {
        public string Age { get; set; }
        public string Resource { get; set; }
        public double Amount { get; set; }
        public string Missing_effect { get; set; }
        public double Missing_power { get; set; }
    }

    public class GameDeclarations
    {
        public List<string> Ages { get; set; }
        public List<PopulationNeeds> Population_needs { get; set; }
    }

    public class Building
    {
        public string Name { get; set; }
        public string Introduced_age { get; set; }
        public string Valid_regions { get; set; }
        public string RequiredResource { get; set; }
    }

    public class Settlement
    {
        public string Name { get; set; }
        public string Introduced_age { get; set; }
        public string Valid_regions { get; set; }
    }

    public class BuildingDeclarations
    {
        public List<Building> Buildings { get; set; }
        public List<Settlement> Settlements { get; set; }
    }

    public class GameAction
    {
        public string Name { get; set; }
        public string Age { get; set; }
    }

    public class ActionDeclarations
    {
        public List<GameAction> Actions { get; set; }
    }
}
