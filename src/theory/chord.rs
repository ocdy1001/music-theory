use super::note::*;
use super::interval::*;
use super::scale::*;

type Chord = Vec<Note>;

pub const MAJOR_TRIAD: [Note; 2] = [MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR_TRIAD: [Note; 2] = [MINOR_THIRD, PERFECT_FIFTH];
pub const MAJOR_SIXTH_CHORD: [Note; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_CHORD: [Note; 3] = [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];

pub fn chord_from_intervals(base: Note, intervals: &[Note]) -> Chord{
    let mut chord = vec![base];
    for interval in intervals{
        chord.push(base + interval);
    }
    chord
}

pub const TRIAD_DEGREES: [usize; 2] = [3, 5];
pub const SEVENTH_DEGREES: [usize; 3] = [3, 5, 7];
pub const NINETH_DEGREES: [usize; 4] = [3, 5, 7, 9];
pub const ELEVENTH_DEGREES: [usize; 5] = [3, 5, 7, 9, 11];
pub const THIRTEENTH_DEGREES: [usize; 6] = [3, 5, 7, 9, 11, 13];

pub fn chord_from_scale(base: Note, scale: &Scale, degrees: &[usize]) -> Chord{
    let slen = scale.len();
    let mut chord = vec![base];
    let mut i = 1;
    let mut note = base;
    let mut index = 0;
    loop{
        if index >= degrees.len(){
            break;
        }
        let d = degrees[index];
        if i == d{
            chord.push(note);
            index += 1;
        }
        note += scale[(i - 1) % slen];
        i += 1;
    }
    chord
}
