trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Fighter {
    Warrior,
    Archer,
    Wizard
}


impl Attacker for Fighter {
    fn choose_style(&self) -> String {
        match self {
            Fighter::Warrior => "Fierce and strong".to_string(),
            Fighter::Archer => "Quick and agile".to_string(),
            Fighter::Wizard => "Cunning and intelligent".to_string(),
        }
    }

    fn choose_weapon(&self) -> String {
        match self {
            Fighter::Warrior => "Sword".to_string(),
            Fighter::Archer => "Bow".to_string(),
            Fighter::Wizard => "Staff".to_string(),
        }
        
    }
}

#[derive(Debug)]
enum Criminal {
    Thief,
    Fraudster,
    Murderer
}

impl Attacker for Criminal {
    fn choose_style(&self) -> String {
        match self {
            Criminal::Thief => "Stealthy and cunning".to_string(),
            Criminal::Fraudster => "Deceptive and manipulative".to_string(),
            Criminal::Murderer => "Brutal and ruthless".to_string(),
        }
    }

    fn choose_weapon(&self) -> String {
        match self {
            Criminal::Thief => "Dagger".to_string(),
            Criminal::Fraudster => "Poison".to_string(),
            Criminal::Murderer => "Axe".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

#[test]
fn tests_trait() {
    let banshee: Fighter = Fighter::Warrior;
    let bonzo: Fighter = Fighter::Archer;
    let bambi: Fighter = Fighter::Wizard;
    let gonzo: Criminal = Criminal::Thief;
    let nigel: Criminal = Criminal::Fraudster;
    let fred: Criminal = Criminal::Murderer;
    dbg!(banshee.choose_style());
    dbg!(bonzo.choose_style());
    dbg!(bambi.choose_style());
    dbg!(gonzo.choose_style());
    dbg!(nigel.choose_style());
    dbg!(fred.choose_style());
}