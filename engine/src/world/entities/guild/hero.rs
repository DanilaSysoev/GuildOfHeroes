mod attributes;
pub mod builders;
mod traits;

use crate::world::entities::guild::MissionContext;

pub trait AttributeEffect {
    fn apply_to_mission(
        &self,
        attribute: &HeroAttribute,
        mission_context: &mut MissionContext,
    );
}

pub struct HeroAttribute {
    name: String,
    value: i32,
    effect: Box<dyn AttributeEffect>,
}

impl HeroAttribute {
    fn new(name: &str, value: i32, effect: Box<dyn AttributeEffect>) -> Self {
        HeroAttribute { name: name.to_string(), value, effect }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn apply_to_mission(&self, mission_context: &mut MissionContext) {
        self.effect.apply_to_mission(self, mission_context);
    }
}

pub trait TraitEffect {
    fn apply_addition_effect(&self, hero: &mut Hero);
    fn apply_removing_effect(&self, hero: &mut Hero);
}

pub struct HeroTrait {
    name: String,
    effect: Box<dyn TraitEffect>,
}

impl HeroTrait {
    fn new(name: &str, effect: Box<dyn TraitEffect>) -> Self {
        HeroTrait { name: name.to_string(), effect }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn apply_addition_effect(&self, hero: &mut Hero) {
        self.effect.apply_addition_effect(hero);
    }

    pub fn apply_removing_effect(&self, hero: &mut Hero) {
        self.effect.apply_removing_effect(hero);
    }
}

pub struct Hero {
    name: String,
    attributes: Vec<HeroAttribute>,
    traits: Vec<HeroTrait>,
}

impl Hero {
    pub fn new(name: &str) -> Self {
        Hero {
            name: name.to_string(),
            attributes: Vec::new(),
            traits: Vec::new(),
        }
    }

    pub fn with_attribute(mut self, attribute: HeroAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_trait(mut self, hero_trait: HeroTrait) -> Self {
        self.traits.push(hero_trait);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
