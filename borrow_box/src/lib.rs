#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(
            Self{
                id,
                p1: (p1_name, 0),
                p2: (p2_name, 0),
                nb_games,
            }
        )
    }

    pub fn read_winner(&self) -> (String, u16) {
        match self.p1.1 == self.p2.1 {
            true => ("Same score! tied".to_string(), self.p1.1),
            false => match self.p1.1 > self.p2.1 {
                        true => (self.p1.0.clone(), self.p1.1),
                        false => (self.p2.0.clone(), self.p2.1),
                     }
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        let max_score = (self.nb_games / 2) + 1;

        if self.p1.1 >= max_score || self.p2.1 >= max_score {
            return;
        }

        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        let id = self.id;
        let _ = Box::new(self);
        format!("game deleted: id -> {}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        assert_eq!(game, Box::new(GameSession{
            id: 0,
            p1: (String::from("Joao"), 0),
            p2: (String::from("Susana"), 0),
            nb_games: 5,
        }));
        assert_eq!(game.read_winner(), ("Same score! tied".to_string(), 0));
        game.update_score(String::from("Joao"));
        game.update_score(String::from("Joao"));
        assert_eq!(game.read_winner(), ("Joao".to_string(), 2));
        game.update_score(String::from("Susana"));
        game.update_score(String::from("Susana"));
        assert_eq!(game.read_winner(), ("Same score! tied".to_string(), 2));
        game.update_score(String::from("Joao"));
        assert_eq!(game.read_winner(), ("Joao".to_string(), 3));
        game.update_score(String::from("Susana"));
        assert_eq!(game.read_winner(), ("Joao".to_string(), 3));

    }
 
        #[test]
        fn test_read_winner_tied() {
            let session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 2),
                p2: ("Bob".to_string(), 2),
                nb_games: 5,
            };
            assert_eq!(session.read_winner(), ("Same score! tied".to_string(), 2));
        }
    
        #[test]
        fn test_read_winner_p1_leading() {
            let session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 3),
                p2: ("Bob".to_string(), 2),
                nb_games: 5,
            };
            assert_eq!(session.read_winner(), ("Alice".to_string(), 3));
        }
    
        #[test]
        fn test_read_winner_p2_leading() {
            let session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 1),
                p2: ("Bob".to_string(), 2),
                nb_games: 3,
            };
            assert_eq!(session.read_winner(), ("Bob".to_string(), 2));
        }
    
        // Tests for update_score
        #[test]
        fn test_update_score_increment_p1() {
            let mut session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 0),
                p2: ("Bob".to_string(), 0),
                nb_games: 3,
            };
            session.update_score("Alice".to_string());
            assert_eq!(session.p1.1, 1);
            assert_eq!(session.p2.1, 0);
        }
    
        #[test]
        fn test_update_score_blocked_clinched() {
            let mut session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 3),
                p2: ("Bob".to_string(), 0),
                nb_games: 5,
            };
            session.update_score("Alice".to_string());
            assert_eq!(session.p1.1, 3);
            assert_eq!(session.p2.1, 0);
        }
    
        #[test]
        fn test_update_score_blocked_sum_reached() {
            let mut session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 3),
                p2: ("Bob".to_string(), 2),
                nb_games: 5,
            };
            session.update_score("Alice".to_string());
            assert_eq!(session.p1.1, 3);
            assert_eq!(session.p2.1, 2);
        }
    
        #[test]
        fn test_update_score_allow_reaching_nb_games() {
            let mut session = GameSession {
                id: 1,
                p1: ("Alice".to_string(), 0),
                p2: ("Bob".to_string(), 0),
                nb_games: 2,
            };
            session.update_score("Alice".to_string());
            session.update_score("Alice".to_string());
            assert_eq!(session.p1.1, 2);
            assert_eq!(session.p2.1, 0);
        }
    
        // Test for delete
        #[test]
        fn test_delete() {
            let session = GameSession {
                id: 42,
                p1: ("A".to_string(), 0),
                p2: ("B".to_string(), 0),
                nb_games: 1,
            };
            let result = session.delete();
            assert_eq!(result, "game deleted: id -> 42");
        }
    }
