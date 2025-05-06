
trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}


#[derive(Debug)]
enum Character{
    Warrior,
    Mage,
    Archer,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => String::from("Melee"),
            Character::Mage => String::from("Magic"),
            Character::Archer => String::from("Ranged"),
        }
    }

    fn choose_weapon(&self) -> String {
        match self {
            Character::Warrior => String::from("Sword"),
            Character::Mage => String::from("Staff"),
            Character::Archer => String::from("Bow"),
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::*; 

    #[test]
    fn test_traits() {
        let my_character = Character::Warrior;
        let my_character_style = my_character.choose_style();
        let my_character_weapon = my_character.choose_weapon();
        dbg!(my_character_style);
        dbg!(my_character_weapon);
    }
}