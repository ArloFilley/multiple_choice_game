#[derive(Clone)]
pub struct Player {
    pub hp: usize,
    pub dmg: usize,
    pub def: usize,
    pub exp: usize
}

impl Player {
    pub fn default() -> Self {
        Player { 
            hp: 100,
            dmg: 15,
            def: 20,
            exp: 0
        }
    }

    pub fn level_up(&mut self) {
        while self.exp >= 100 {
            println!("LEVEL UP");
            self.hp = self.hp.saturating_add(10);
            self.dmg = self.dmg.saturating_add(5);
            self.def = self.def.saturating_add(5);
            self.exp = self.exp.saturating_sub(100);
            println!("{} hp, {} dmg, {} def", self.hp, self.dmg, self.def);
        }
    }
}
