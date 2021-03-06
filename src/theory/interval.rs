use super::note::Note;
use std::cmp::Ordering;

pub const SEMI: Note = 1;
pub const WHOLE: Note = 2;

pub const UNISON: Note = 0;
pub const MINOR_SECOND: Note = 1;
pub const MAJOR_SECOND: Note = 2;
pub const MINOR_THIRD: Note = 3;
pub const MAJOR_THIRD: Note = 4;
pub const PERFECT_FOURTH: Note = 5;
pub const TRITONE: Note = 6;
pub const PERFECT_FIFTH: Note = 7;
pub const MINOR_SIXTH: Note =  8;
pub const MAJOR_SIXTH: Note = 9;
pub const MINOR_SEVENTH: Note = 10;
pub const MAJOR_SEVENTH: Note = 11;
pub const OCTAVE: Note = 12;

pub const FLAT_NINETH: Note = 13;
pub const NINETH: Note = 14;
pub const SHARP_NINETH: Note = 15;
pub const FLAT_ELEVENTH: Note = 16;
pub const ELEVENTH: Note = 17;
pub const SHARP_ELEVENTH: Note = 18;
pub const TWELVETH: Note = 19;
pub const FLAT_THIRTEENTH: Note = 20;
pub const THIRTEENTH: Note = 21;
pub const SHARP_THIRTEENTH: Note = 22;

pub const DIMINISHED_SECOND: Note = 0;
pub const AUGMENTED_UNISON: Note = 1;
pub const DIMINISHED_THIRD: Note = 2;
pub const AUGMENTED_SECOND: Note = 3;
pub const DIMINISHED_FOURTH: Note = 4;
pub const AUGMENTED_THIRD: Note = 5;
pub const DIMINISHED_FIFTH: Note = 6;
pub const AUGMENTED_FOURTH: Note = 6;
pub const DIMINISHED_SIXTH: Note = 7;
pub const AUGMENTED_FIFTH: Note = 8;
pub const DIMINISHED_SEVENTH: Note = 9;
pub const AUGMENTED_SIXTH: Note = 10;
pub const DIMINISHED_OCTAVE: Note = 11;
pub const AUGMENTED_SEVENTH: Note = 12;

pub fn interval_chord_extension(interval: Note) -> String{
    match interval{
        0 => "R",
        MINOR_SECOND => "♭2",
        MAJOR_SECOND => "♮2",
        MINOR_THIRD => "♭3",
        MAJOR_THIRD => "♮3",
        PERFECT_FOURTH => "♮4",
        TRITONE => "♭5",
        PERFECT_FIFTH => "♮5",
        MINOR_SIXTH => "♭6",
        MAJOR_SIXTH => "♮6",
        MINOR_SEVENTH => "♭7",
        MAJOR_SEVENTH => "♮7",
        OCTAVE => "",
        13 => "♭9",
        14 => "♮9",
        15 => "♯9",
        16 => "♭11",
        17 => "♮11",
        18 => "♯11",
        19 => "",
        20 => "♭13",
        21 => "♮13",
        22 => "♯13",
        _ => "",
    }.to_string()
}

pub fn to_relative_interval_non_nat(interval: Note) -> String{
    let mut res = String::new();
    let i = interval;
    match i.cmp(&0){
        Ordering::Less => { for _ in 0..-i{ res.push('♭'); } },
        Ordering::Greater => { for _ in 0..i{ res.push('♯'); } },
        Ordering::Equal => { res.push('♮'); },
    }
    res
}

pub fn to_degree(interval: Note) -> String{
    match interval{
        0 => "I",
        1 => "bII",
        2 => "II",
        3 => "bIII",
        4 => "III",
        5 => "IV",
        6 => "bV",
        7 => "V",
        8 => "bVI",
        9 => "VI",
        10 => "bVII",
        11 => "VII",
        _ => "[outofrange]",
    }.to_string()
}
