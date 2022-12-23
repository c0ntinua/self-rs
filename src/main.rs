use rand::random;
use raylib::prelude::*;
mod calc; use calc::*;
mod plot; use plot::*;
mod init; use init::*;
mod respond; use respond::*;


fn main() {
    let mut p = ran_pos();
    let mut f = vec![vec![0usize;32];32];
    let mut positions = vec!(); positions.push(ran_pos()); for _ in 0..200 { positions.push(ran_pos())};
    randomize(&mut f);
    let (mut handle, thread) = handle_and_thread(32*32,32*32);
    unsafe{::raylib::ffi::SetTargetFPS(30);}
	while !handle.window_should_close() {
        respond(&mut f, &handle);
        for  p in &mut positions {
            flip(p, &mut f);
            *p = pos(&p,&f);

        }
        
        let mut screen = handle.begin_drawing(&thread);
        plot( &f,&mut screen);

    }

}
