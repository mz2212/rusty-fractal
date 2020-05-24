// The Sierpinski carpet. I just can't be bothered to spell it everywhere.

pub fn crunch(x: u32, y: u32, max_iter: u32) -> u32 {
    
    let mut result = 0;
    let mut newx = x;
    let mut newy = y;
    while newx > 0 || newy > 0 {
        if newx % 3 == 1 && newy % 3 == 1  {
            result = max_iter;
        }
        newx /= 3;
        newy /= 3;
    }
    result
}