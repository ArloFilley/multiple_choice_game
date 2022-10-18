use crate::block::Block;
use crate::enemy::Enemy;
use crate::player::Player;
use std::fs;
use std::collections::HashMap;
use crate::terminal;

pub struct Story {
    pub filename: String,
    pub blocks: HashMap<String, Block>,
    pub enemies: HashMap<String, Enemy>,
    pub choices: String,
    pub player: Player,
}

impl Story {
    pub fn default() -> Self {
        Story { 
            filename: String::from("story.txt"), 
            blocks: HashMap::new(),
            enemies: HashMap::new(),
            choices: String::from(""),
            player: Player::default()
        }
    }

    pub fn read(&mut self) {
        self.read_story().expect("error reading story file");
        self.read_enemies().expect("error reading enemies file");
        self.read_player().expect("error reading player file");
    }

    fn read_story(&mut self) -> Result<(), std::io::Error> {
        let file_text = fs::read_to_string(&self.filename)?;
        let mut block = Block::default();
        let mut lastline = String::from("");
        let mut index = String::from("");

        for (i, line) in file_text.lines().enumerate() {
            let copy = line.clone();
            let line = line.to_lowercase();
            if i == 0 {
            } else if line.eq("newchoice") {
                self.blocks.insert(index.clone() ,block);
                block = Block::default();
            } else if lastline.eq("newchoice") {
                index = String::from(&line);

            } else if line.eq("choices"){
            } else if lastline.eq("choices") {
                let line = &line;
                for i in line.chars() {
                    block.choices.push(i);
                }

            } else if line.eq("fight") {
            } else if lastline.eq("fight") {
                block.fight = String::from(&line)
            } else if line.eq("gameover") {
                block.gameover = true;
            } else if line.eq("goto") {
            } else if lastline.eq("goto") {
                block.goto = String::from(&line);
            } else {
                block.content.push_str(&copy);
                block.content.push('\n');
            }
            lastline = line;
        }
        self.blocks.insert(index ,block);
        Ok(())
    }

    fn read_enemies(&mut self) -> Result<(), std::io::Error> {
        let file_text = fs::read_to_string("enemies.txt")?;
        let mut enemy = Enemy::default();
        for (i, line) in file_text.lines().enumerate() {
            println!("{}", line);
            if i % 7 == 0 {
                if enemy.def != 0 {
                    self.enemies.insert(enemy.name.clone(), enemy);
                    enemy = Enemy::default();
                }
                enemy.name = String::from(line).to_lowercase();
            } else if i % 7 == 1 {
                enemy.hp = (&line).trim().parse().unwrap();
            } else if i % 7 == 2 {
                enemy.dmg = (&line).trim().parse().unwrap();
            } else if i % 7 == 3 {
                enemy.def = (&line).trim().parse().unwrap();
            } else if i % 7 == 4 {
                enemy.exp = (&line).trim().parse().unwrap();
            }
        }
        self.enemies.insert(enemy.name.clone(), enemy);
        Ok(())
    }

    fn read_player(&mut self) -> Result<(), std::io::Error> {
        let file_text = fs::read_to_string("player.txt")?;
        for (i, line) in file_text.lines().enumerate() {
            println!("{}", line);
            if i % 4 == 0 {
                self.player.hp = (&line).trim().parse().unwrap();
            } else if i % 4 == 1 {
                self.player.dmg = (&line).trim().parse().unwrap();
            } else if i % 4 == 2 {
                self.player.def = (&line).trim().parse().unwrap();
            } else if i % 4 == 3 {
                self.player.exp = (&line).trim().parse().unwrap();
            }
        }
        Ok(())
    }

    pub fn intro(&mut self) {
        terminal::clear_screen();
        self.choices = String::from("i");
        self.blocks.get(&self.choices)
            .expect("error")
            .print_block();
    }

    pub fn read_block(&mut self) {
        terminal::clear_screen();
        loop {
            let block = self.blocks.get(&self.choices)
                .expect("error getting block");
            block.print_block();
            if !block.fight.eq("") {
                let hi = self.enemies.get(&block.fight)
                    .expect("no enemy found")
                    .fight(&self.player);
                if hi.is_some() {
                    self.player.exp = self.player.exp.saturating_add(hi.expect("error"));
                } else {
                    self.choices = String::from("end");
                }
            }

            if block.gameover == true {
                self.choices = String::from("end");
                return;
            }

            if !block.goto.eq("") {
                self.choices = block.goto.clone();
            } else {
                return;
            }
        }
    }

    pub fn get_choices(&mut self) -> Vec<char> {
        self.blocks.get(&self.choices).expect("error getting choices").choices.clone()
    }

    pub fn choose(&mut self, input: &str) {
        let choices = self.get_choices();

        let input = &input[0..1];
        if input.len() == 0 {
            return;
        }

        let mut choice: char = 'q';
        for c in input.chars() {
            choice = c;
        }
        if choice == 'q' {
            self.choices = String::from("end");
            return;
        }
        for i in choices.iter() {
            if choice == *i {
                self.choices.push(choice);
            }
        }
    }

    pub fn print_lose_message(&self) {
        println!("{}", fs::read_to_string("gameover_lose.txt").unwrap());
    }

    pub fn how_to_play(&self) {
        terminal::clear_screen();
        println!("{}", fs::read_to_string("welcome.txt").unwrap());
    }
}