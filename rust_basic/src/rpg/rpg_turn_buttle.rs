pub mod turn_buttle {

    #[derive(Clone)]
    struct Object {
        collision_detection: bool,
    }

    impl Object {
        fn new(collision_detection: bool) -> Self {
            Self {
                collision_detection,
            }
        }
    }

    #[derive(Clone)]
    struct Character {
        object_feature: Object,
        chara_name: String,
        hp: i16,
        attack: i16,
        defence: i16,
        dead: bool,
    }

    impl Character {
        fn new(name: String, hp: i16, attack: i16, defence: i16) -> Self {
            Self {
                object_feature: Object::new(true),
                chara_name: name,
                hp,
                attack,
                defence,
                dead: false,
            }
        }
    }

    impl std::fmt::Debug for Character {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter
                .debug_struct("Character")
                .field("name", &self.chara_name)
                .field("hp", &self.hp)
                .finish()
        }
    }

    trait Fight {
        fn attack(&self) -> i16;
        fn attacked(&mut self, damage: i16) -> bool;
    }

    impl Fight for Character {
        fn attack(&self) -> i16 {
            self.attack.clone()
        }

        fn attacked(&mut self, damage: i16) -> bool {
            let damage = damage - (self.defence.clone() as f32 * 0.1) as i16;
            self.hp = self.hp.clone() - damage;
            if self.hp < 0 {
                self.dead = true;
                self.dead.clone()
            } else {
                self.dead.clone()
            }
        }
    }

    struct ButtleField {
        player_chara: Character,
        enemy_chara: Character,
        turn: i16,
    }

    trait MainButtle {
        fn main_buttle(&mut self);
    }

    impl MainButtle for ButtleField {
        fn main_buttle(&mut self) {
            println!("{:?}", &self.player_chara);
            println!("{:?}", &self.enemy_chara);

            loop {
                if self.enemy_chara.attacked(self.player_chara.attack()) {
                    println!("{:?}", &self.enemy_chara);
                    println!("勝ち申した！");
                    break;
                }
                if self.player_chara.attacked(self.enemy_chara.attack()) {
                    println!("{:?}", &self.player_chara);
                    println!("負けてしまった！");
                    break;
                }
                println!("{:?}", &self.player_chara);
                println!("{:?}", &self.enemy_chara);
            }
        }
    }

    pub fn print_buttle_start() {
        println!("buttle starts!!");
    }

    pub fn yamasaki_buttle() {
        let player_chara = Character::new(String::from("akechi mitsuhide"), 100, 30, 10);
        let enemy_chara = Character::new(String::from("hashiba hideyoshi"), 80, 40, 0);
        let mut buttle_field = ButtleField {
            player_chara: player_chara.clone(),
            enemy_chara: enemy_chara.clone(),
            turn: 0,
        };

        buttle_field.main_buttle();
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
}
