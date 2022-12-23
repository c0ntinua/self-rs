
use rand::random;
pub type Grid = Vec<Vec<usize>>;
pub type Pos = (usize,usize);

pub fn randomize(f : &mut Grid) { for r in 0..32 {for c in 0..32 { f[r][c] = random::<usize>()%2;}}}
pub fn safe(i : i32) -> usize {if 0 <= i && i < 32 {return i as usize}; if i < 0 {return (i + 32) as usize }; return (i - 32) as usize; }
pub fn v(f : &Grid, i : usize) -> usize {f[i/32][2*(i%16)]}
pub fn h(f : &Grid, i : usize) -> usize {f[i/32][2*(i%16)+1]}

//pub fn flip(p : &Pos, f : &mut Grid ) {f[p.0][p.1] = ran_bit();}
pub fn flip(p : &Pos, f : &mut Grid ) {if f[p.0][p.1] == 0 {f[p.0][p.1] = 1;} else {f[p.0][p.1] = 0;}}


pub fn pos(p : &Pos, f : &Grid) -> Pos { 
    let pos_mod = vec![(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    let mut q = p;
    let c = code(p,f,&pos_mod);
    ( safe(p.0 as i32 + m(v(f,c))) , safe(  p.1 as i32 + m(h(f,c))) )
}

pub fn m(x : usize) -> i32 {if x== 0 {-1} else {1}}    
pub fn code(p : &Pos, f :&Grid, md : &[(i32,i32)]) -> usize {
    let mut s =0usize;let mut c = 1; for m in md {s += c*f[safe(p.0 as i32 +m.0)][safe(p.1 as i32 +m.1)]; c *= 2;} s}
pub fn ran_pos() -> Pos {  (rand::random::<usize>()%32,rand::random::<usize>()%32) }
pub fn ran_bit() -> usize {rand::random::<usize>()%2 }