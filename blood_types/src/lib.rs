#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::fmt;
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty blood type string".into());
        }
        let (antigen_part, rh_part) = s.split_at(s.len() - 1);
        let antigen = antigen_part.parse()?;
        let rh_factor = rh_part.parse()?;
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then_with(|| self.rh_factor.cmp(&other.rh_factor))
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check antigen compatibility
        let antigen_compatible = match (other.antigen.clone(), self.antigen.clone()) {
            (Antigen::O, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::AB) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::AB) => true,
            (Antigen::AB, Antigen::AB) => true,
            _ => false,
        };

        // Check Rh compatibility
        let rh_compatible = other.rh_factor == RhFactor::Negative || self.rh_factor == RhFactor::Positive;

        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];
        let mut donors = Vec::new();

        for antigen in &antigens {
            for rh in &rh_factors {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }

        donors.sort();
        donors
    }

    pub fn recipients(&self) -> Vec<Self> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];
        let mut recipients = Vec::new();

        for antigen in &antigens {
            for rh in &rh_factors {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }

        recipients.sort();
        recipients
    }
}