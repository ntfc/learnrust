/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves = Vec::new();
    
    let mut src_peg: Vec<u32> = Vec::new();
    let mut aux_peg: Vec<u32> = Vec::new();
    let mut dst_peg: Vec<u32> = Vec::new();

    let number_of_moves = 2u32.pow(num_discs);

    for disc in (0..num_discs).rev() {
        src_peg.push(disc);
    };
    
    for i in 1..number_of_moves {
        match i % 3 {
            1 => {
                match num_discs % 2 {
                    0 => moves.push(make_a_move(&mut src_peg, src, &mut aux_peg, aux)),
                    _ => moves.push(make_a_move(&mut src_peg, src, &mut dst_peg, dst)),
                }
            },
            2 => {
                match num_discs % 2 {
                    0 => moves.push(make_a_move(&mut src_peg, src, &mut dst_peg, dst)),
                    _ => moves.push(make_a_move(&mut src_peg, src, &mut aux_peg, aux)),
                }
            },
            0 => {
                match num_discs % 2 {
                    0 => moves.push(make_a_move(&mut dst_peg, dst, &mut aux_peg, aux)),
                    _ => moves.push(make_a_move(&mut aux_peg, aux, &mut dst_peg, dst)),
                }
            },
            _ => (),
        }
    }

    moves
}

pub fn make_a_move(src_discs: &mut Vec<u32>, src_peg: Peg, dst_discs: &mut Vec<u32>, dst_peg: Peg) -> Move {
    // dst -> src
    let m1: Move = if src_discs.is_empty() {
        src_discs.push(dst_discs.pop().unwrap());
        (dst_peg, src_peg)
    // src -> dst
    } else if dst_discs.is_empty() {
        dst_discs.push(src_discs.pop().unwrap());
        (src_peg, dst_peg)
    // dst -> src
    } else if src_discs.last().unwrap() > dst_discs.last().unwrap() {
        src_discs.push(dst_discs.pop().unwrap());
        (dst_peg, src_peg)
    }
    // src -> dst
    else {
        dst_discs.push(src_discs.pop().unwrap());
        (src_peg, dst_peg)
    };
    m1
}
