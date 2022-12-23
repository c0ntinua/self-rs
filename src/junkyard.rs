//pub fn _prnt(s : usize) { if s == 1 {print!("\u{2588}\u{2588}");} else {print!("  ");}}
//pub fn curs(v : usize, h : usize) {print!("\x1b[{};{}H",v,h);}
//pub fn clrs() {print!("\x1b[2J");}
//pub fn flip(p : &Pos, f : &mut Grid ) {if f[p.0][p.1] == 0 {f[p.0][p.1] = 1;} else {f[p.0][p.1] = 0;}}
//pub fn show(f : &Grid) { for r in 0..32 { for c in 0..32 { curs(r,2*c+1); _prnt(f[r][c]);}} }