use rand::{distributions::{Distribution, Standard}, Rng,};
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

pub enum State
{
    Attack,
    Defend,
    Nothing,
}

pub enum Race
{
    Human,
    Ogre,
    DarkElf,
    Hobbit,
}

pub struct Enemy
{
    attack: i32,
    defense: i32,
    hp: i32,
    name: String,
    state: State,
    race: Race,
    race_name: String,
}

impl Distribution<Race> for Standard 
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race
    {
        match rng.gen_range(0, 4)
        {
            0 => Race::Human,
            1 => Race::Ogre,
            2 => Race::DarkElf,
            _ => Race::Hobbit,
        }
    }    
}

impl Enemy
{
    fn create_enemy(race: Race) -> Self
    {
        Enemy
        {
            race: race,
            attack: 2,
            defense: 2,
            hp: 50,
            name: String::new(),
            state: State::Nothing,
            race_name: String::new(),
        }
    }

    fn add_bonuses(&mut self)
    {
        let bonus_attack: i32;
        let bonus_defense: i32;
        let bonus_hp: i32;
        let bonus_race_name: &str;
        let bonus_name: String;

        fn get_name(file: &str) -> String
        {
            let mut name = String::new();
            let mut name_file = File::open(file).expect("Wrong");

            name_file.read_to_string(&mut name).expect("Wrong");

            let name_list = Vec::from_iter(name.split("\n").map(String::from));

            let enemy_name_index = name_list.len();
            let random_number0 = rand::thread_rng().gen_range(0, enemy_name_index);

            return name_list[random_number0].to_string();
        }

        match self.race
        {
            Race::DarkElf => {bonus_attack = 2; bonus_defense = 3; bonus_hp = 32; bonus_race_name = "Dark Elf"; bonus_name = get_name("src/enemy/name_lists/darkelf_names.txt"); },
            Race::Hobbit => {bonus_attack = 3; bonus_defense = -2; bonus_hp = -12; bonus_race_name = "Hobbit"; bonus_name = get_name("src/enemy/name_lists/hobbit_names.txt"); },
            Race::Human => {bonus_attack = 2; bonus_defense = -1; bonus_hp = 11; bonus_race_name = "Human"; bonus_name = get_name("src/enemy/name_lists/human_names.txt"); },
            Race::Ogre => {bonus_attack = 4; bonus_defense = 7; bonus_hp = 22; bonus_race_name = "Ogre"; bonus_name = get_name("src/enemy/name_lists/ogre_names.txt"); },
        }

        self.attack += bonus_attack;
        self.defense += bonus_defense;
        self.hp += bonus_hp;
        self.race_name = bonus_race_name.to_string();
        self.name = bonus_name;
    }

    pub fn info(&mut self)
    {
        println!("{} has {} attack points, {} defense points and {} HP. I am a {}", self.name, self.attack, self.defense, self.hp, self.race_name);
    }

    pub fn create(race: Race) -> Enemy
    {
        let mut enemy = Enemy::create_enemy(race);
        enemy.add_bonuses();
        return enemy;
    }
}
