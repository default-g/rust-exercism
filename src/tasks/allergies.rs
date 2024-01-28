pub struct Allergies {
    allergies: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl TryFrom<u32> for Allergen {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
       match value {
        0 => Ok(Self::Eggs),
        1 => Ok(Self::Peanuts),
        2 => Ok(Self::Shellfish),
        3 => Ok(Self::Strawberries),
        4 => Ok(Self::Tomatoes),
        5 => Ok(Self::Chocolate),
        6 => Ok(Self::Pollen),
        7 => Ok(Self::Cats),
        _ => Err(()),
       }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = vec![];
        for i in (0..=7).rev() {
            if score & (1 << i) != 0 {
                allergies.push(Allergen::try_from(i).unwrap());
            }
        }

        Self {
            allergies
        }

    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
       self.allergies.to_vec()
    }
}
