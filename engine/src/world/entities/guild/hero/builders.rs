use std::collections::HashMap;

use crate::{
    errors::GameError,
    world::entities::guild::hero::{
        AttributeEffect, HeroAttribute, HeroTrait, TraitEffect,
    },
};

type AttributeEffectFactory = dyn Fn() -> Box<dyn AttributeEffect>;
type TraitEffectFactory = dyn Fn() -> Box<dyn TraitEffect>;

#[derive(Default)]
pub struct HeroAttributeBuilder {
    effects: HashMap<String, Box<AttributeEffectFactory>>,
}

impl HeroAttributeBuilder {
    pub fn new() -> Self {
        Self { effects: HashMap::new() }
    }

    pub fn with_factory(
        mut self,
        name: &str,
        factory: Box<AttributeEffectFactory>,
    ) -> Result<Self, GameError> {
        if self.effects.insert(name.to_string(), factory).is_none() {
            return Ok(self);
        }

        Err(GameError::HeroAttributeFactoryAlreadyExists {
            name: name.to_string(),
        })
    }

    pub fn build(
        &self,
        name: &str,
        value: i32,
    ) -> Result<HeroAttribute, GameError> {
        if let Some(factory) = self.effects.get(name) {
            return Ok(HeroAttribute::new(
                name,
                value,
                create_attribute_effect(factory),
            ));
        }

        Err(GameError::HeroAttributeNotFound { name: name.to_string() })
    }
}

fn create_attribute_effect(
    factory: &AttributeEffectFactory,
) -> Box<dyn AttributeEffect> {
    factory()
}

#[derive(Default)]
pub struct HeroTraitBuilder {
    effects: HashMap<String, Box<TraitEffectFactory>>,
}

impl HeroTraitBuilder {
    pub fn new() -> Self {
        Self { effects: HashMap::new() }
    }

    pub fn with_factory(
        mut self,
        name: &str,
        factory: Box<TraitEffectFactory>,
    ) -> Result<Self, GameError> {
        if self.effects.insert(name.to_string(), factory).is_none() {
            return Ok(self);
        }

        Err(GameError::HeroTraitFactroyAlreadyExists {
            name: name.to_string(),
        })
    }

    pub fn build(&self, name: &str) -> Result<HeroTrait, GameError> {
        if let Some(factory) = self.effects.get(name) {
            Ok(HeroTrait::new(name, create_trate_effect(factory)))
        } else {
            Err(GameError::HeroTraitNotFound { name: name.to_string() })
        }
    }
}

fn create_trate_effect(factory: &TraitEffectFactory) -> Box<dyn TraitEffect> {
    factory()
}
