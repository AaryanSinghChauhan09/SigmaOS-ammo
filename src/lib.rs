//! SigmaOS-ammo: Ammunition game model.
//!
//! Real implementation: rounds, magazines, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    pub id: String,
    pub name: String,
    pub caliber: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Magazine {
    pub id: String,
    pub round_id: String,
    pub capacity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmmoError {
    EmptyRoundName,
    DuplicateRound(String),
}

impl fmt::Display for AmmoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRoundName => write!(f, "round name must not be empty"),
            Self::DuplicateRound(id) => write!(f, "duplicate round {:?}", id),
        }
    }
}

impl std::error::Error for AmmoError {}

pub struct Ammo {
    pub name: String,
    rounds: BTreeMap<String, Round>,
    magazines: BTreeMap<String, Magazine>,
}

impl Ammo {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), rounds: BTreeMap::new(), magazines: BTreeMap::new() }
    }

    pub fn add_round(&mut self, r: Round) -> Result<(), AmmoError> {
        if r.name.trim().is_empty() { return Err(AmmoError::EmptyRoundName); }
        let id = r.id.clone();
        if self.rounds.contains_key(&id) {
            return Err(AmmoError::DuplicateRound(id.clone()));
        }
        self.rounds.insert(id, r);
        Ok(())
    }

    pub fn round_count(&self) -> usize { self.rounds.len() }

    pub fn get_round(&self, id: &str) -> Option<&Round> { self.rounds.get(id) }

    pub fn add_magazine(&mut self, m: Magazine) {
        self.magazines.insert(m.id.clone(), m);
    }

    pub fn get_magazine(&self, id: &str) -> Option<&Magazine> { self.magazines.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Ammo::new("Ammo");
        assert_eq!(a.name, "Ammo");
        assert_eq!(a.round_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Ammo::new("Ammo");
        let round = Round { id: "r1".into(), name: "5.56mm".into(), caliber: "5.56".into() };
        a.add_round(round).unwrap();
        assert_eq!(a.round_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Ammo::new("Ammo");
        let round = Round { id: "x".into(), name: "".into(), caliber: "x".into() };
        assert!(a.add_round(round).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Ammo::new("Ammo");
        let round = Round { id: "x".into(), name: "X".into(), caliber: "x".into() };
        a.add_round(round).unwrap();
        let round2 = Round { id: "x".into(), name: "X".into(), caliber: "x".into() };
        assert!(a.add_round(round2).is_err());
    }
}
