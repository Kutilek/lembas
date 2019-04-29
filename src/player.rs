use crate::input;
use std::fmt;

pub struct Stat
{
    value: i32,
    title: String,
}

impl fmt::Display for Stat
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}: {}\n", self.title, self.value)
    }
}

pub struct Player
{
    attack: Stat,
    defense: Stat,
    hp: Stat,
    name: String,
}

impl Player
{
    pub fn info(&self)
    {
        println!("Name: {}{}{}{}",self.name, self.attack, self.defense, self.hp);
    }

    pub fn create_player() -> Self
    {
        fn get_number() -> i32
        {
            let input0 = input::input();
            let input1 = input0.trim_end();
            let value = input1.parse::<i32>().unwrap();

            return value;
        }

        fn get_name() -> String
        {
            println!("What is your name?");
            let mut name = input::input();
            
            loop
            {
                println!("So {} it is? yes or no?", &name.trim_end());
                let input = input::input();
                match input.trim_end()
                {
                    "no" => { println!("Oh..."); name = get_name(); break;},
                    "yes" => { println!("Oh what a nice name"); println!("{}", name); break;},
                    _ => println!("Sorry, we did not understand you..."),
                }
            }
            return name.to_string();
        }

        fn get_points(stre: &str, re: &mut i32) -> i32
        {
            println!("You have {} character points left to assign\nHow many {} points do you wish to have?", re, stre);
            
            let mut points: i32 = get_number();

            if points > *re
            {
                println!("You don't have that many points.. Please come again");
                points = get_points(stre, re);
            }
            else 
            {
                *re -= points;
            }
            
            return points;
        }

        let mut character_points = 30;
        let name = get_name();
        let attack = Stat {value: get_points("Attack", &mut character_points), title: "Attack".to_string()};
        let defense = Stat {value: get_points("Defense", &mut character_points), title: "Defense".to_string()};
        let hp = Stat {value: get_points("HP", &mut character_points)*10, title: "HP".to_string()};

        Player
        {
            attack: attack,
            defense: defense,
            hp: hp,
            name: name,
        }
    }
}
