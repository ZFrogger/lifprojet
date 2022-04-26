use rand::{thread_rng, Rng};

struct Labyrinthe {
    row:usize,
    col:usize,
    vertical: Vec<Vec<bool>>,
    horizontal: Vec<Vec<bool>>,
}

impl Labyrinthe {
    fn create(row: usize, col: usize) -> Labyrinthe {
        Labyrinthe {
            row: row,
            col: col,
            vertical: vec![vec![true; col+1]; row],
            horizontal: vec![vec![true; col]; row+1],
        }
    }
    fn mur_alea(&mut self) {
        //let mut rng = rand::thread_rng();
        for i in 0..self.row {
            self.vertical[i][self.col] = rand::random();
            for j in 0..self.col {
                self.horizontal[i][j] = rand::random();
                self.vertical[i][j] = rand::random();
                if i == 0 {
                    self.horizontal[self.row][j] = rand::random();
                }
            }
        }
    }
    fn affiche(&self) {
        for i in 0..self.row {
            for j in 0..self.col {
                print!("{}", if self.horizontal[i][j] { "  -----\t" } else { "\t" });
            }
            println!();
            for j in 0..(self.col+1) {
                print!("{}", if self.vertical[i][j] { "|\t" } else { "\t" });
            }
            println!();
        }
        for j in 0..self.col {
            print!("{}", if self.horizontal[self.row][j] { "  -----\t" } else { "\t" });
        }
        println!();
    }
    fn enlevermur(&mut self, cell1: (usize,usize), cell2: (usize,usize)) {
        //println!("ENLEVERMUR");
        if cell1.0 != cell2.0 {
            let mut max = cell2.0;
            if cell1.0 > cell2.0 {
                max = cell1.0;
            }
            self.horizontal[max][cell1.1] = false;
        } else if cell1.1 != cell2.1 {
            let mut max = cell2.1;
            if cell1.1 > cell2.1 {
                max = cell1.1;
            }   
            self.vertical[cell1.0][max] = false;
        }
    }
}

fn neighbours(x_y: (usize,usize), laby: &Labyrinthe) -> Vec::<(usize,usize)> {
    let mut temp = Vec::new();
    //println!("NEIGHBOURS DE {} {}", x, y);
    if x_y.0+1<laby.row {
        temp.push(((x_y.0+1),x_y.1));
    }
    if x_y.0>0 {
        temp.push(((x_y.0-1),x_y.1));
    }
    if x_y.1+1<laby.col {
        temp.push((x_y.0,(x_y.1+1)));
    }
    if x_y.1>0 {
        temp.push((x_y.0,(x_y.1-1)));
    }
    return temp;
}
//fn filter<T>(v: Vec<T>, p: &dyn FnMut(&mut T) -> bool) -> Vec<T> 
fn filter<T, F: FnMut(&mut T) -> bool>(mut v: Vec<T>, mut p: F) -> Vec<T> {
    let mut i = 0;
    while i < v.len() {
        if p(&mut v[i]) {
            v.remove(i);
        } else {
            i += 1;
        }
    }
    return v;
}
/*
fn recursive_backtrack<R: Rng>(x: usize, y: usize, laby: Box<Labyrinthe>, visited: &mut Vec<Vec<bool>>, rng : &mut R) {
    visited[x][y] = true;
    //let mut v = neighbours(x, y, laby).drain_filter(|x| visited[*x/laby.row][*x%laby.row]).collect();
    let mut v1 = neighbours(x, y, &laby).clone();
    let mut v: Vec<_> = v1.iter().filter(|x| visited[*x/laby.row][*x%laby.row]).collect();
    //let mut v = filter(neighbours(x, y, laby), |x: usize| visited[x/laby.row][x%laby.row]);
    while !v.is_empty() {
        let cell_rand = v[rng.gen_range(0..v.len())];
        laby.enlevermur(*cell_rand, x*laby.col+y);
        recursive_backtrack(cell_rand/laby.row, cell_rand%laby.row, laby, visited, rng);
        v = neighbours(x, y, &laby).clone().iter().filter(|x| visited[*x/laby.row][*x%laby.row]).collect();
        //v = filter(neighbours(x, y, laby), |x: usize| visited[x/laby.row][x%laby.row]);
        //v = neighbours(x, y, laby).drain_filter(|x| visited[*x/laby.row][*x%laby.row]).collect();
    }
}
*/
fn recursive_backtrack<R: Rng>(
    x_y: (usize,usize),
    laby: &mut Labyrinthe,
    visited: &mut Vec<Vec<bool>>,
    rng: &mut R,
) {
    //println!("RECURSIVE : {} {}", x_y.0, x_y.1);
    visited[x_y.0][x_y.1] = true;
    let n = neighbours(x_y, &laby);
    let mut v: Vec<_> = n
        .iter()
        .filter(|(x,y)| !visited[*x][*y])
        .collect();
    while !v.is_empty() {
        let cell_rand = v[rng.gen_range(0..v.len())];
        laby.enlevermur(*cell_rand, (x_y.0,x_y.1));
        recursive_backtrack(*cell_rand, laby, visited, rng);
        v = n
            .iter()
            .filter(|(x,y)| !visited[*x][*y])
            .collect();
    }
}

fn check_done(visited: &mut Vec<Vec<usize>>) -> bool {
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] != 2 {
                return false;
            }
        }
    }
    return true;
}

fn get_outs(visited: &mut Vec<Vec<usize>>) -> Vec::<(usize,usize)> {
    let mut temp = Vec::new();
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] == 1 {
                temp.push(((i),j));
            }
        }
    }
    return temp;
}

fn prim<R: Rng>(
    x_y: (usize,usize),
    laby: &mut Labyrinthe,
    visited: &mut Vec<Vec<usize>>,
    rng: &mut R,
) {
    let mut current_cell = x_y;
    let n = neighbours(current_cell, &laby);
    let mut v: Vec<_> = n
    .iter()
    .filter(|(x,y)| visited[*x][*y]==0)
    .collect();
    for i in 0..v.len() {
        visited[v[i].0][v[i].1] = 1;
    }
    visited[current_cell.0][current_cell.1] = 2;
    while !check_done(visited) {
        // get one random 'outs'
        let w = get_outs(visited);
        current_cell = w[rng.gen_range(0..w.len())];
        visited[current_cell.0][current_cell.1] = 2;
        // mark neighbours of current_cell as 'outs'
        let n = neighbours(current_cell, &laby);
        let mut v: Vec<_> = n
        .iter()
        .filter(|(x,y)| visited[*x][*y]==0)
        .collect();
        for i in 0..v.len() {
            visited[v[i].0][v[i].1] = 1;
        }
        // get random neighbour 'in' of current_cell
        let n2 = neighbours(current_cell, &laby);
        let mut v2: Vec<_> = n2
        .iter()
        .filter(|(x,y)| visited[*x][*y]==2)
        .collect();
        let cell_rand_in = v2[rng.gen_range(0..v2.len())];
        // remove wall between 'current_cell' and 'cell_rand_in'
        laby.enlevermur(current_cell, *cell_rand_in);
    }
}


fn main() {
    let row = 15;
    let col = 10;
    let mut rng = thread_rng();
    let mut test = vec![vec![0; col]; row];
    let mut visited = vec![vec![false; col]; row];
    /*
    for i in 0..test.len() {
        for j in 0..test[i].len() {
            print!("{}", test[i][j]);
        }
        println!();
    }
    */
    let mut laby = Labyrinthe::create(row,col);
    recursive_backtrack((0,0),&mut laby, &mut visited, &mut rng);
    laby.affiche();
    let mut laby2 = Labyrinthe::create(row,col);
    prim((0,0),&mut laby2, &mut test, &mut rng);
    laby2.affiche();
    //laby.mur_alea();
    //laby.affiche();

    /*
    laby.affiche();
    laby.mur_alea();
    laby.affiche();
    
    const ROW:usize = 5;
    const COL:usize = 3;
    const COL_FIXED:usize = COL+1;
    const ROW_FIXED:usize = ROW+1;
    let mut vertical = [[true; COL_FIXED] ; ROW];
    let mut horizontal = [[true; COL] ; ROW_FIXED];
    horizontal[1][2] = false;
    vertical[1][1] = false;
    vertical[0][1] = false;
    vertical[1][0] = false;
    
    for i in 0..ROW {
        for j in 0..COL {
            print!("{}", if horizontal[i][j] { "  -----\t" } else { "\t" });
        }
        println!();
        for j in 0..COL_FIXED {
            print!("{}", if vertical[i][j] { "|\t" } else { "\t" });
        }
        println!();
    }
    for j in 0..COL {
        print!("{}", if horizontal[ROW][j] { "  -----\t" } else { "\t" });
    }
    println!();

    expliquer algos

    */
    //algo
}