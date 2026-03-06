//! Planet types for the space-western trading game
//! Based on ADR 0005: Market/Economy System

use crate::simulation::commodity::CommodityType;
use serde::{Deserialize, Serialize};

/// Enum representing all planet types as defined in ADR 0005
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetType {
    /// Agricultural Planet
    /// Supplies: Water, Foodstuffs
    /// Demands: Medicine, Firearms, Ammunition, Electronics
    /// Ignores: Metals, Antimatter, Narcotics, Alien Artefacts
    Agricultural,

    /// Mega City Planet
    /// Supplies: Electronics, Medicine, Narcotics
    /// Demands: Water, Foodstuffs, Firearms, Ammunition
    /// Ignores: Metals, Antimatter, Alien Artefacts
    MegaCity,

    /// Mining Planet
    /// Supplies: Metals, Antimatter, Electronics
    /// Demands: Water, Foodstuffs, Medicine, Ammunition
    /// Ignores: Narcotics, Alien Artefacts
    Mining,

    /// Pirate Space Station
    /// Supplies: Narcotics, Ammunition
    /// Demands: Foodstuffs, Firearms, Medicine
    /// Ignores: Water, Metals, Antimatter, Electronics, Alien Artefacts
    PirateSpaceStation,

    /// Research Outpost
    /// Supplies: Electronics, Medicine, Alien Artefacts
    /// Demands: Water, Foodstuffs
    /// Ignores: Firearms, Ammunition, Metals, Antimatter, Narcotics
    ResearchOutpost,

    /// Industrial Planet
    /// Supplies: Electronics, Metals, Ammunition, Antimatter
    /// Demands: Water, Foodstuffs, Medicine
    /// Ignores: Narcotics, Alien Artefacts
    Industrial,

    /// Frontier Colony
    /// Supplies: Water, Foodstuffs
    /// Demands: Medicine, Firearms, Ammunition, Electronics, Metals, Antimatter, Alien Artefacts
    /// Ignores: Narcotics
    FrontierColony,
}

impl PlanetType {
    /// Get all planet types
    pub fn all() -> Vec<PlanetType> {
        vec![
            PlanetType::Agricultural,
            PlanetType::MegaCity,
            PlanetType::Mining,
            PlanetType::PirateSpaceStation,
            PlanetType::ResearchOutpost,
            PlanetType::Industrial,
            PlanetType::FrontierColony,
        ]
    }

    /// Get the display name of the planet type
    pub fn display_name(&self) -> &'static str {
        match self {
            PlanetType::Agricultural => "Agricultural Planet",
            PlanetType::MegaCity => "Mega City Planet",
            PlanetType::Mining => "Mining Planet",
            PlanetType::PirateSpaceStation => "Pirate Space Station",
            PlanetType::ResearchOutpost => "Research Outpost",
            PlanetType::Industrial => "Industrial Planet",
            PlanetType::FrontierColony => "Frontier Colony",
        }
    }

    /// Get a brief description of the planet type
    pub fn description(&self) -> &'static str {
        match self {
            PlanetType::Agricultural => {
                "Agricultural Planet - Supplies Water, Foodstuffs; Demands Medicine, Firearms, Ammunition, Electronics"
            }
            PlanetType::MegaCity => {
                "Mega City Planet - Supplies Electronics, Medicine, Narcotics; Demands Water, Foodstuffs, Firearms, Ammunition"
            }
            PlanetType::Mining => {
                "Mining Planet - Supplies Metals, Antimatter, Electronics; Demands Water, Foodstuffs, Medicine, Ammunition"
            }
            PlanetType::PirateSpaceStation => {
                "Pirate Space Station - Supplies Narcotics, Ammunition; Demands Foodstuffs, Firearms, Medicine"
            }
            PlanetType::ResearchOutpost => {
                "Research Outpost - Supplies Electronics, Medicine, Alien Artefacts; Demands Water, Foodstuffs"
            }
            PlanetType::Industrial => {
                "Industrial Planet - Supplies Electronics, Metals, Ammunition, Antimatter; Demands Water, Foodstuffs, Medicine"
            }
            PlanetType::FrontierColony => {
                "Frontier Colony - Supplies Water, Foodstuffs; Demands Medicine, Firearms, Ammunition, Electronics, Metals, Antimatter, Alien Artefacts"
            }
        }
    }

    /// Get the commodities that this planet type supplies
    pub fn supplies(&self) -> Vec<CommodityType> {
        match self {
            PlanetType::Agricultural => vec![CommodityType::Water, CommodityType::Foodstuffs],
            PlanetType::MegaCity => vec![
                CommodityType::Electronics,
                CommodityType::Medicine,
                CommodityType::Narcotics,
            ],
            PlanetType::Mining => vec![
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::Electronics,
            ],
            PlanetType::PirateSpaceStation => {
                vec![CommodityType::Narcotics, CommodityType::Ammunition]
            }
            PlanetType::ResearchOutpost => vec![
                CommodityType::Electronics,
                CommodityType::Medicine,
                CommodityType::AlienArtefacts,
            ],
            PlanetType::Industrial => vec![
                CommodityType::Electronics,
                CommodityType::Metals,
                CommodityType::Ammunition,
                CommodityType::Antimatter,
            ],
            PlanetType::FrontierColony => vec![CommodityType::Water, CommodityType::Foodstuffs],
        }
    }

    /// Get the commodities that this planet type demands
    pub fn demands(&self) -> Vec<CommodityType> {
        match self {
            PlanetType::Agricultural => vec![
                CommodityType::Medicine,
                CommodityType::Firearms,
                CommodityType::Ammunition,
                CommodityType::Electronics,
            ],
            PlanetType::MegaCity => vec![
                CommodityType::Water,
                CommodityType::Foodstuffs,
                CommodityType::Firearms,
                CommodityType::Ammunition,
            ],
            PlanetType::Mining => vec![
                CommodityType::Water,
                CommodityType::Foodstuffs,
                CommodityType::Medicine,
                CommodityType::Ammunition,
            ],
            PlanetType::PirateSpaceStation => vec![
                CommodityType::Foodstuffs,
                CommodityType::Firearms,
                CommodityType::Medicine,
            ],
            PlanetType::ResearchOutpost => vec![CommodityType::Water, CommodityType::Foodstuffs],
            PlanetType::Industrial => vec![
                CommodityType::Water,
                CommodityType::Foodstuffs,
                CommodityType::Medicine,
            ],
            PlanetType::FrontierColony => vec![
                CommodityType::Medicine,
                CommodityType::Firearms,
                CommodityType::Ammunition,
                CommodityType::Electronics,
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::AlienArtefacts,
            ],
        }
    }

    /// Get the commodities that this planet type ignores (neither supplies nor demands)
    pub fn ignores(&self) -> Vec<CommodityType> {
        match self {
            PlanetType::Agricultural => vec![
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::Narcotics,
                CommodityType::AlienArtefacts,
            ],
            PlanetType::MegaCity => vec![
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::AlienArtefacts,
            ],
            PlanetType::Mining => vec![CommodityType::Narcotics, CommodityType::AlienArtefacts],
            PlanetType::PirateSpaceStation => vec![
                CommodityType::Water,
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::Electronics,
                CommodityType::AlienArtefacts,
            ],
            PlanetType::ResearchOutpost => vec![
                CommodityType::Firearms,
                CommodityType::Ammunition,
                CommodityType::Metals,
                CommodityType::Antimatter,
                CommodityType::Narcotics,
            ],
            PlanetType::Industrial => vec![CommodityType::Narcotics, CommodityType::AlienArtefacts],
            PlanetType::FrontierColony => vec![CommodityType::Narcotics],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_type_properties() {
        assert_eq!(
            PlanetType::Agricultural.display_name(),
            "Agricultural Planet"
        );
        assert_eq!(
            PlanetType::Agricultural.description(),
            "Agricultural Planet - Supplies Water, Foodstuffs; Demands Medicine, Firearms, Ammunition, Electronics"
        );

        let agricultural_supplies = PlanetType::Agricultural.supplies();
        assert!(agricultural_supplies.contains(&CommodityType::Water));
        assert!(agricultural_supplies.contains(&CommodityType::Foodstuffs));

        let agricultural_demands = PlanetType::Agricultural.demands();
        assert!(agricultural_demands.contains(&CommodityType::Medicine));
        assert!(agricultural_demands.contains(&CommodityType::Firearms));
        assert!(agricultural_demands.contains(&CommodityType::Ammunition));
        assert!(agricultural_demands.contains(&CommodityType::Electronics));

        let agricultural_ignores = PlanetType::Agricultural.ignores();
        assert!(agricultural_ignores.contains(&CommodityType::Metals));
        assert!(agricultural_ignores.contains(&CommodityType::Antimatter));
        assert!(agricultural_ignores.contains(&CommodityType::Narcotics));
        assert!(agricultural_ignores.contains(&CommodityType::AlienArtefacts));
    }

    #[test]
    fn test_all_planet_types_exist() {
        let all_types = PlanetType::all();
        assert_eq!(all_types.len(), 7);
        assert!(all_types.contains(&PlanetType::Agricultural));
        assert!(all_types.contains(&PlanetType::MegaCity));
        assert!(all_types.contains(&PlanetType::Mining));
        assert!(all_types.contains(&PlanetType::PirateSpaceStation));
        assert!(all_types.contains(&PlanetType::ResearchOutpost));
        assert!(all_types.contains(&PlanetType::Industrial));
        assert!(all_types.contains(&PlanetType::FrontierColony));
    }

    #[test]
    fn test_planet_type_supply_demand_correctness() {
        // Test each planet type has the correct supply/demand patterns as specified in ADR 0005

        // Agricultural Planet
        let ag_supplies = PlanetType::Agricultural.supplies();
        assert_eq!(ag_supplies.len(), 2);
        assert!(ag_supplies.contains(&CommodityType::Water));
        assert!(ag_supplies.contains(&CommodityType::Foodstuffs));

        let ag_demands = PlanetType::Agricultural.demands();
        assert_eq!(ag_demands.len(), 4);
        assert!(ag_demands.contains(&CommodityType::Medicine));
        assert!(ag_demands.contains(&CommodityType::Firearms));
        assert!(ag_demands.contains(&CommodityType::Ammunition));
        assert!(ag_demands.contains(&CommodityType::Electronics));

        // Mega City Planet
        let city_supplies = PlanetType::MegaCity.supplies();
        assert_eq!(city_supplies.len(), 3);
        assert!(city_supplies.contains(&CommodityType::Electronics));
        assert!(city_supplies.contains(&CommodityType::Medicine));
        assert!(city_supplies.contains(&CommodityType::Narcotics));

        let city_demands = PlanetType::MegaCity.demands();
        assert_eq!(city_demands.len(), 4);
        assert!(city_demands.contains(&CommodityType::Water));
        assert!(city_demands.contains(&CommodityType::Foodstuffs));
        assert!(city_demands.contains(&CommodityType::Firearms));
        assert!(city_demands.contains(&CommodityType::Ammunition));

        // Mining Planet
        let mining_supplies = PlanetType::Mining.supplies();
        assert_eq!(mining_supplies.len(), 3);
        assert!(mining_supplies.contains(&CommodityType::Metals));
        assert!(mining_supplies.contains(&CommodityType::Antimatter));
        assert!(mining_supplies.contains(&CommodityType::Electronics));

        let mining_demands = PlanetType::Mining.demands();
        assert_eq!(mining_demands.len(), 4);
        assert!(mining_demands.contains(&CommodityType::Water));
        assert!(mining_demands.contains(&CommodityType::Foodstuffs));
        assert!(mining_demands.contains(&CommodityType::Medicine));
        assert!(mining_demands.contains(&CommodityType::Ammunition));

        // Pirate Space Station
        let pirate_supplies = PlanetType::PirateSpaceStation.supplies();
        assert_eq!(pirate_supplies.len(), 2);
        assert!(pirate_supplies.contains(&CommodityType::Narcotics));
        assert!(pirate_supplies.contains(&CommodityType::Ammunition));

        let pirate_demands = PlanetType::PirateSpaceStation.demands();
        assert_eq!(pirate_demands.len(), 3);
        assert!(pirate_demands.contains(&CommodityType::Foodstuffs));
        assert!(pirate_demands.contains(&CommodityType::Firearms));
        assert!(pirate_demands.contains(&CommodityType::Medicine));

        // Research Outpost
        let research_supplies = PlanetType::ResearchOutpost.supplies();
        assert_eq!(research_supplies.len(), 3);
        assert!(research_supplies.contains(&CommodityType::Electronics));
        assert!(research_supplies.contains(&CommodityType::Medicine));
        assert!(research_supplies.contains(&CommodityType::AlienArtefacts));

        let research_demands = PlanetType::ResearchOutpost.demands();
        assert_eq!(research_demands.len(), 2);
        assert!(research_demands.contains(&CommodityType::Water));
        assert!(research_demands.contains(&CommodityType::Foodstuffs));

        // Industrial Planet
        let industrial_supplies = PlanetType::Industrial.supplies();
        assert_eq!(industrial_supplies.len(), 4);
        assert!(industrial_supplies.contains(&CommodityType::Electronics));
        assert!(industrial_supplies.contains(&CommodityType::Metals));
        assert!(industrial_supplies.contains(&CommodityType::Ammunition));
        assert!(industrial_supplies.contains(&CommodityType::Antimatter));

        let industrial_demands = PlanetType::Industrial.demands();
        assert_eq!(industrial_demands.len(), 3);
        assert!(industrial_demands.contains(&CommodityType::Water));
        assert!(industrial_demands.contains(&CommodityType::Foodstuffs));
        assert!(industrial_demands.contains(&CommodityType::Medicine));

        // Frontier Colony
        let frontier_supplies = PlanetType::FrontierColony.supplies();
        assert_eq!(frontier_supplies.len(), 2);
        assert!(frontier_supplies.contains(&CommodityType::Water));
        assert!(frontier_supplies.contains(&CommodityType::Foodstuffs));

        let frontier_demands = PlanetType::FrontierColony.demands();
        assert_eq!(frontier_demands.len(), 7);
        assert!(frontier_demands.contains(&CommodityType::Medicine));
        assert!(frontier_demands.contains(&CommodityType::Firearms));
        assert!(frontier_demands.contains(&CommodityType::Ammunition));
        assert!(frontier_demands.contains(&CommodityType::Electronics));
        assert!(frontier_demands.contains(&CommodityType::Metals));
        assert!(frontier_demands.contains(&CommodityType::Antimatter));
        assert!(frontier_demands.contains(&CommodityType::AlienArtefacts));
    }
}
