use crate::{player::Player, terminal};
use std::{io};
#[derive(Clone)]
pub struct Enemy {
    pub name: String,
    pub hp: usize,
    pub dmg: usize,
    pub def: usize,
    pub exp: usize
}

impl Enemy {
    pub fn default() -> Self {
        Enemy {
            name: String::from("unknown"),
            hp: 0,
            dmg: 0,
            def: 0,
            exp: 0
        }
    }

    pub fn fight(&self, ply: &Player) -> Option<usize> {
        let mut player = ply.clone();
        let mut enemy = self.clone();
        println!("!!!FIGHT!!!\nYou are fighting a {}\nthe {} has {} hp", enemy.name, enemy.name, enemy.hp);
        println!("do you A: attack or B: heal");
        for (i, line) in io::stdin().lines().enumerate() {
            if i > 0 {
                terminal::clear_screen();
            }
            let line = line.expect("error");
            let input = line.to_lowercase();
            if input.eq("a") {
                enemy.hp = enemy.hp.saturating_sub(player.dmg);
                println!("You attack dealing {} damage\nthe {} looks a little weaker, now being on {} hp", player.dmg, enemy.name, enemy.hp);
            } else if input.eq("b") {
                player.hp = player.hp.saturating_add(player.dmg);
                println!("You heal yourself for {} hp\nyou are now on {}", player.dmg, player.hp);
            } else if input.eq("q") {
                return None
            } else {
                println!("You sit there twiddling your thumbs");
            }

            player.hp = player.hp.saturating_sub(enemy.dmg);
            println!("the {} attacks you for {} hp\nYou are now on {} hp", enemy.name, enemy.dmg, player.hp);

            if enemy.hp <= 0 {
                player.exp = player.exp.saturating_add(enemy.exp);
                println!("You beat the {} and gain {} exp\nyou now have {} exp\n", enemy.name, enemy.exp, player.exp);
                break;
            } else if player.hp <= 0 {
                println!("You died to the {}", enemy.name);
                break;
            }
            println!("\n\n!!!FIGHT!!!\nYou are fighting a {}\nthe {} has {} hp", enemy.name, enemy.name, enemy.hp);
            println!("do you A: attack or B: heal");
        }

        if enemy.hp <= 0 {
            player.exp = player.exp.saturating_add(enemy.exp);
            println!("You beat the {} and gain {} exp\nyou now have {} exp\n", enemy.name, enemy.exp, player.exp);
            return Some(enemy.exp)
        } else if player.hp <= 0 {
            println!("You died to the {}", enemy.name);
        }
        None
    }
}