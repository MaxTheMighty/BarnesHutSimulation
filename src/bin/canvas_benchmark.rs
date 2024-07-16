#![feature(test)]
use barnes_hut::canvas::Canvas;


extern crate test;
fn main() {}

pub fn draw_square_unsafe(){
    let mut canvas: Canvas = Canvas::new(1000,1000,(0,0,0,0));
    canvas.draw_square(500,500,400,&(0,0,0,0));
}

pub fn draw_square_safe(){
    let mut canvas: Canvas = Canvas::new(1000,1000,(0,0,0,0));
    canvas.draw_square_safe(500,500,400,&(0,0,0,0));
}
#[cfg(test)]
mod tests{
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_draw_square_unsafe(b: &mut Bencher){
        b.iter(|| draw_square_unsafe());
    }

    #[bench]
    fn bench_draw_square_safe(b: &mut Bencher){
        b.iter(|| draw_square_safe());
    }

}
