enum Cell {
    Fixed(u8),
    Normal(CellS),
}

#[derive(Debug)]
struct CellS {
    possible_values: [bool; 9],
}

impl CellS {
    pub fn new() -> CellS {
        CellS {
            possible_values: [true; 9],
        }
    }

    pub fn build(av: [bool; 9]) -> CellS {
        CellS {
            possible_values: av,
        }
    }

}

struct Sudoku {
    cells: Vec<Cell>,
}

impl Sudoku {
    pub fn new(input: [[u8; 9]; 9]) -> Sudoku {
        let mut cells: Vec<Cell> = Vec::with_capacity(81);
        for row in input {
            for el in row {
                if el != 0 {
                    cells.push(Cell::Fixed(el));
                } else {
                    cells.push(Cell::Normal(CellS::new()));
                }
            }
        }
        Sudoku { cells }
    }

    fn find_box_corner(val: usize) -> usize {
        let res = val % 3;
        let new_val = val - res;
        let res = val % 27;
        let a = res / 9;
        new_val - 9 * a
    }

    fn find_available_in_row(&self, i: usize) -> [bool; 9] {
        let mut av = [true; 9];
        let min_row = i % 9;
        let range = min_row..=min_row + 8 * 9;
        for row in range.step_by(9) {
            if let Cell::Fixed(v) = self.cells[row] {
                if v > 0 {
                    av[v as usize - 1] = false;
                }
            };
        }
        av
    }

    fn find_available_in_col(&self, i: usize) -> [bool; 9] {
        let mut av = [true; 9];

        let min_col = (i / 9) * 9;
        println!("Check here {i} {min_col}");
        for col in min_col..min_col + 9 {
            if let Cell::Fixed(v) = self.cells[col] {
                if v > 0 {
                    av[v as usize - 1] = false;
                }
            };
        }
        av
    }

    fn square_indices(idx: usize) -> [usize;9]{
        let mut res = [0;9];
        let mut bc = Sudoku::find_box_corner(idx);
        let mut i = 1;
        while i<=9{
            res[i-1] = bc;
            if i % 3 == 0{
                bc += 7;
            } else{
                bc += 1;
            }
            i+=1;
        }
        res
    }

    fn row_indices(idx: usize) -> [usize; 9]{
        let mut res = [0;9];
        let min_row = idx % 9;
        let mut i = 0;
        let range = min_row..=min_row + 8 * 9;
        for row in range.step_by(9) {
            res[i] = row;
            i += 1;
        }
        res
    }


    fn column_indices(idx: usize) -> [usize;9]{
        let mut res = [0;9];
        let min_col = (idx/ 9) * 9;
        let mut i = 0;
        for col in min_col..min_col + 9 {
            res[i] = col;
            i+=1;
        }
        res
    }

    fn find_available_in_square(&self, i: usize) -> [bool; 9] {
        let mut bc = Sudoku::find_box_corner(i);
        let mut av = [true; 9];
        let mut i: usize = 1;
        while i <= 9 {
            if let Cell::Fixed(v) = self.cells[bc] {
                if v > 0 {
                    av[v as usize - 1] = false;
                }
            }
            if i % 3 == 0 {
                bc += 7;
            } else {
                bc += 1;
            }
            i += 1;
        }
        av
    }

    fn initialize(&mut self) {
        for i in 1..81 {
            let possible_values = self.find_available(i);
            let cell = &self.cells[i];
            if let Cell::Fixed(_) = cell {
                continue;
            };
            // println!("i is {i}");
            if possible_values.iter().filter(|e| **e).count() > 1 {
                self.cells[i] = Cell::Normal(CellS::build(possible_values));
            } else {
                let val = possible_values.iter().position(|&e| e).unwrap() + 1;
                self.cells[i] = Cell::Fixed(val as u8);
            }
        }
    }

    fn find_next_to_fill(&self, i: usize) -> Option<usize> {
        for j in i..81 {
            match &self.cells[j] {
                Cell::Fixed(_) => continue,
                Cell::Normal(_) => return Some(j),
            }
        }
        for j in 0..81 {
            match &self.cells[j] {
                Cell::Fixed(_) => continue,
                Cell::Normal(_) => return Some(j),
            }
        }
        None
    }

    fn get_valid_values(pv: [bool; 9]) -> Vec<usize> {
        let mut v = Vec::new();
        for (idx, _el) in pv.iter().enumerate() {
            v.push(idx + 1);
        }
        v
    }

    fn is_valid(&self, i: usize, val: usize) -> bool {
        let col_av = self.find_available_in_col(i);
        let row_av = self.find_available_in_row(i);
        let s_av = self.find_available_in_square(i);
        if !col_av[val - 1] {
            return false;
        }
        if !row_av[val - 1] {
            return false;
        }
        if !s_av[val - 1] {
            return false;
        }
        true
    }

    fn find_available(&self, i: usize) -> [bool;9]{
        let mut possible_values = [true; 9];
        let col_av = self.find_available_in_col(i);
        let row_av = self.find_available_in_row(i);
        let s_av = self.find_available_in_square(i);
        combine_arr(&mut possible_values, &col_av);
        combine_arr(&mut possible_values, &row_av);
        combine_arr(&mut possible_values, &s_av);
        possible_values
    }

    fn find_must_be_value(&self, pv: [bool; 9], i: usize) -> Option<usize>{
        for idx in 0..9{
            let val = pv[idx];
            if !val{
                continue;
            }

            let row_indices = Sudoku::row_indices(i);
            let mut possible_in_row = false;
            for row_idx in row_indices{
                possible_in_row |= self.find_available(row_idx)[idx];
            }
            if !possible_in_row{
                return Some(idx);
            }

            let mut possible_in_col = false;
            let col_indices = Sudoku::column_indices(i);
            for col_idx in col_indices{
                possible_in_col |=  self.find_available(col_idx)[idx];
            }
            if !possible_in_col{
                return Some(idx);
            }

            let square_indices = Sudoku::square_indices(i);
            let mut possible_in_square = false;
            for si in square_indices{
                possible_in_square |= self.find_available(si)[idx];
            }
            if !possible_in_square{
                return Some(idx);
            }
        }
        None
    }

    fn fill_analytically(&mut self) {
        loop {
            let mut changed = false;
            for i in 0..81 {
                if let Cell::Fixed(_) = self.cells[i]{
                    continue;
                }
                let possible_values = self.find_available(i);
                if let Some(v) = self.find_must_be_value(possible_values, i) {
                    self.cells[i] = Cell::Fixed(v as u8 + 1);
                    changed = true;
                    break; 
                }
                if possible_values.iter().filter(|e| **e).count() > 1 {
                    self.cells[i] = Cell::Normal(CellS::build(possible_values));
                } else {
                    let val = match possible_values.iter().position(|&e| e){
                        Some(v) => v+1,
                        None => {
                            println!("robe");
                            0
                        }
                    };

                    self.cells[i] = Cell::Fixed(val as u8);
                    changed = true;
                    break;
                }
            }
            if !changed{
                break;
            }
        }
    }

    pub fn solve(&mut self, i: usize) -> bool {
        self.fill_analytically();
        let i = self.find_next_to_fill(i);
        if i.is_none() {
            return true;
        }
        let i = i.unwrap();
        let pv = match &self.cells[i] {
            Cell::Normal(a) => a.possible_values,
            _ => return false,
        };

        for e in Sudoku::get_valid_values(pv) {
            if !self.is_valid(i, e) {
                continue;
            }
            self.cells[i] = Cell::Fixed(e as u8);
            if self.solve(i) {
                return true;
            }

            self.cells[i] = Cell::Normal(CellS::build(pv));
        }
        false
    }

    pub fn print(&self) {
        for i in 0..81 {
            match self.cells[i] {
                Cell::Fixed(v) => print!(" {v} "),
                _ => print!(" e "),
            }
            if i % 9 == 8 {
                print!("\n");
            }
        }
    }
}

fn combine_arr(arr1: &mut [bool; 9], arr2: &[bool; 9]) {
    for (a, b) in arr1.iter_mut().zip(arr2) {
        *a = if !b { *b } else { *a };
    }
}

fn main() {
    let input: [[u8; 9]; 9] = [
        [5, 0, 0, 1, 0, 6, 0, 0, 4],
        [0, 8, 0, 0, 4, 5, 3, 9, 0],
        [0, 0, 0, 8, 3, 2, 0, 5, 0],
        [3, 1, 0, 0, 8, 4, 6, 0, 2],
        [4, 0, 0, 0, 0, 0, 0, 3, 1],
        [2, 7, 0, 0, 0, 0, 0, 0, 9],
        [6, 2, 0, 0, 0, 8, 4, 0, 5],
        [8, 5, 1, 0, 0, 0, 7, 0, 3],
        [0, 4, 0, 5, 0, 0, 0, 2, 0],
    ];
    let mut sudoku = Sudoku::new(input);
    sudoku.initialize();
    println!("Hello, world!");
    println!("{}", Sudoku::find_box_corner(79));
    sudoku.solve(0);
    sudoku.print();
}
