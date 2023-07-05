use std::{fmt::Display, time::Duration};

fn main() {
    let _n = 42;
    let _p = 6;
    power_test_1();

    println!("Hello, world!");
    let tuple: (f32, f32, _) = (0.1, 0.2, 42);
    dbg!(tuple);
    dbg!(tuple.2);
    // dbg!(@tuple.42);

    let tab: [_; 4] = [0, 1, 2, 3];
    dbg!(tab);
    dbg!(tab[0]);

    // dbg!(power(n,p, None));

    test_duration();
    display_diff(None);

    // references try
    let mut str = String::from("Can I ask a");
    let str2 = String::from(" question?");
    concat(&mut str, &str2); // ne renvoie que le str non concaténé
    dbg!(&str); // je n'arrive pas a faire le debug ici

    //repeat test

    let str = "test ";
    let n = 3;
    println!(
        "repeating {n} times '{str}' gives '{}'",
        repeat_str(str, n)
    );

    show_point((12, 42, 45, String::from("test")));

    // GAME

    let (mut gandalf, mut legolas) = game();
    let pv = gandalf.pv; // attention valeur copié et non pas reference
    dbg!(pv);
    gandalf.heal(3);
    dbg!(gandalf.pv);
    gandalf.cast_spell(&mut legolas);
    dbg!(legolas.pv);
    dbg!(gandalf.pv);
    dbg!(gandalf.clone());
    println!("{}", gandalf);

    // collections

    collections();
    collections2();
}

fn power_test_1() {
    let n = 42;
    let p = 6;
    let mut r: i128 = 1;

    // TODO

    for _ in 0..p {
        r *= n;
    }

    dbg!(r);
}

// fn power(n: i64, p: i64, total: Option<i64>) -> i64 {
//     if total.is_none(){
//         total = 1;
//     }
//     if p <= 0{
//         return n;
//     }
//     return power(n*total, p-1, total);
// }

fn test_duration() {
    let duration = Duration::new(17896, 0);
    let duration2 = Duration::new(1546, 154);
    let diff: Duration = duration - duration2;
    // println!("diff is {diff}");
    dbg!(diff);
    let _diff_with_fn = duration.saturating_sub(duration2);
    let diff_negativ_with_fn = duration2.saturating_sub(duration);
    dbg!(diff_negativ_with_fn);
}

fn display_diff(duration: Option<Duration>) {
    if duration.is_some() {
        dbg!(duration);
    } else {
        println!("Duration is not here")
    }
}

fn concat(str1: &mut String, str2: &str) {
    str1.push_str(str2);
    dbg!(str1);
}

fn repeat_str(str: &str, mut nb: i32) -> String {
    let mut str_res = String::from("");
    while nb > 0 {
        str_res.push_str(str);
        nb -= 1;
    }
    str_res
}

type Point = (i32, i32, i32, String);

fn show_point(point: Point) {
    println!("<{}>: ({}, ({}), ({})", point.0, point.1, point.2, point.3,)
}
#[derive(Debug, Clone)]
enum Class {
    Paladin,
    Voleur,
    Mage { points_of_magic: i32 },
}
#[derive(Debug, Clone)]
struct Perso {
    name: String,
    pv: u16,
    class: Class,
}

impl Display for Perso {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This perso is {} and have {} pv", &self.name, &self.pv)
    }
}

impl Perso {
    fn heal(&mut self, points_to_add: u16) {
        if self.pv + points_to_add > 100 {
            self.pv = 100;
        }
        self.pv += points_to_add;
    }
    fn cast_spell(&mut self, perso: &mut Perso) {
        self.pv -= 5;
        match self.class {
            Class::Mage {
                mut points_of_magic,
            } => {
                if points_of_magic >= 25 {
                    points_of_magic -= 25; // how to get point of magic?
                }
                perso.pv -= 50;
            }
            Class::Paladin => todo!(),
            Class::Voleur => todo!(),
        }
    }
}
fn game() -> (Perso, Perso) {
    let gandalf = Perso {
        name: String::from("Gandalf"),
        pv: 95,
        class: Class::Mage {
            points_of_magic: 50,
        },
    };
    let _aragorn = Perso {
        name: String::from("Aragorn"),
        pv: 75,
        class: Class::Voleur,
    };
    let legolas = Perso {
        name: String::from("Legolas"),
        pv: 85,
        class: Class::Paladin,
    };
    let _gimli = Perso {
        name: String::from("Gimli"),
        pv: 60,
        class: Class::Paladin,
    };
    (gandalf, legolas)
}

fn collections() {
    let words = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("third"),
    ];
    // for word in words{
    //     if(word == String::from("third")){
    //         println!("yes you did find {}", word)
    //     }
    // }
    let str = "thirds".to_owned();
    words.contains(&str);
}

#[derive(Debug)]
struct Character {
    name: String,
    height: u16,
    mass: u16,
}

fn collections2() {
    let characters = vec![
        Character {
            name: "Luke Skywalker".to_owned(),
            height: 172,
            mass: 77,
        },
        Character {
            name: "C-3PO".to_owned(),
            height: 167,
            mass: 75,
        },
        Character {
            name: "R2-D2".to_owned(),
            height: 96,
            mass: 32,
        },
        Character {
            name: "Darth Vader".to_owned(),
            height: 202,
            mass: 136,
        },
        Character {
            name: "Leia Organa".to_owned(),
            height: 150,
            mass: 49,
        },
        Character {
            name: "Owen Lars".to_owned(),
            height: 178,
            mass: 120,
        },
        Character {
            name: "Beru Whitesun lars".to_owned(),
            height: 165,
            mass: 75,
        },
        Character {
            name: "R5-D4".to_owned(),
            height: 97,
            mass: 32,
        },
        Character {
            name: "Biggs Darklighter".to_owned(),
            height: 183,
            mass: 84,
        },
        Character {
            name: "Obi-Wan Kenobi".to_owned(),
            height: 182,
            mass: 77,
        },
    ];

    let imcs_persos = characters
        .iter()
        .map(|caracter| caracter.mass as f32 / (caracter.height as f32 * caracter.height as f32))
        .collect::<Vec<_>>();
    dbg!(imcs_persos);
}
