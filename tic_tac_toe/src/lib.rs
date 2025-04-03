pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[1][1] && player == table[2][2]) || 
    (player == table[2][0] && player == table[1][1] && player == table[0][2])
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[0][1] && player == table[0][2]) ||
    (player == table[1][0] && player == table[1][1] && player == table[1][2]) ||
    (player == table[2][0] && player == table[2][1] && player == table[2][2])
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (player == table[0][0] && player == table[1][0] && player == table[2][0]) ||
    (player == table[0][1] && player == table[1][1] && player == table[2][1]) ||
    (player == table[0][2] && player == table[1][2] && player == table[2][2])
}
    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];
        let horz = [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']];
        let vert = [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']];
        assert_eq!(diagonals('X', diag), true);
        assert_eq!(horizontal('X', horz), false);
        assert_eq!(vertical('O', vert), true);
    }
}
