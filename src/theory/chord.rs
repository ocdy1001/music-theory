use std::collections::HashSet;
use super::note::*;
use super::interval::*;
use super::scale::*;
use crate::utils::roman_numerals::to_roman_num;

pub const NUM_SUPS: [char; 10] = ['⁰', 'ⁱ', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
pub const NUM_SUBS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

pub const MAJOR: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH];
pub const MINOR: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH];
pub const MINOR_AUGMENTED: &[Note] = &[MINOR_THIRD, AUGMENTED_FIFTH];
pub const MAJOR_AUGMENTED: &[Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH];
pub const MINOR_DIMINISHED: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH];
pub const MAJOR_DIMINISHED: &[Note] = &[MAJOR_THIRD, DIMINISHED_FIFTH];
pub const SUS2: &[Note] = &[MAJOR_SECOND,PERFECT_FIFTH];
pub const SUS4: &[Note] = &[PERFECT_FOURTH,PERFECT_FIFTH];
pub const SUPER_SUS: &[Note] = &[MAJOR_SECOND,PERFECT_FOURTH];
pub const PHRYGIAN: &[Note] = &[MINOR_SECOND,PERFECT_FIFTH];
pub const LYDIAN: &[Note] = &[AUGMENTED_FOURTH,PERFECT_FIFTH];
pub const LOCRIAN2: &[Note] = &[MINOR_SECOND,DIMINISHED_FIFTH];
pub const LOCRIAN4: &[Note] = &[PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const SUPER_LOCRIAN: &[Note] = &[MINOR_SECOND,PERFECT_FOURTH,DIMINISHED_FIFTH];
pub const MAJOR_SIXTH_CHORD: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MINOR_SIXTH_CHORD: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH];
pub const MAJOR_SEVENTH_CHORD: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const MINOR_SEVENTH_CHORD: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const DOMINANT_SEVENTH: &[Note] = &[MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
pub const MINOR_MAJOR_SEVENTH: &[Note] = &[MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
pub const HALF_DIMINISHED_SEVENTH: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];
pub const DIMINISHED_SEVENTH_CHORD: &[Note] = &[MINOR_THIRD, DIMINISHED_FIFTH, DIMINISHED_SEVENTH];
pub const AUGMENTED_SEVENTH_CHORD: &[Note] = &[MAJOR_THIRD, AUGMENTED_FIFTH, MINOR_SEVENTH];
pub const MU_CHORD: &[Note] = &[MAJOR_SECOND,MAJOR_THIRD,PERFECT_FIFTH];
pub const SIX_NINE_CHORD: &[Note] = &[MAJOR_THIRD,PERFECT_FIFTH,MAJOR_SIXTH,NINETH];

// (pattern, name, major base string?, extended collection?)
pub type ChordBook = &'static [(&'static [Note],&'static str,bool,bool)];

pub const STD_CHORD_BOOK: ChordBook = &[
    (MAJOR, "", true, false),
    (MINOR, "", false, false),
    (MINOR_AUGMENTED, "+", false, true),
    (MAJOR_AUGMENTED, "+", true, false),
    (MINOR_DIMINISHED, "°", false, false),
    (MAJOR_DIMINISHED, "°", true, true),
    (SUPER_SUS, "ssus", true, true),
    (PHRYGIAN, "phry", true, false),
    (LYDIAN, "lyd", true, false),
    (LOCRIAN2, "loc2", true, false),
    (LOCRIAN4, "loc4", true, false),
    (SUPER_LOCRIAN, "o", true, true),
    (MAJOR_SIXTH_CHORD, "⁶", true, false),
    (MINOR_SIXTH_CHORD, "⁶", false, false),
    (MAJOR_SEVENTH_CHORD, "∆", true, false),
    (MINOR_SEVENTH_CHORD, "-", false, false),
    (DOMINANT_SEVENTH, "⁷", true, false),
    (MINOR_MAJOR_SEVENTH, "-∆", true, false),
    (HALF_DIMINISHED_SEVENTH, "ø", false, false),
    (DIMINISHED_SEVENTH_CHORD, "°⁷", false, false),
    (AUGMENTED_SEVENTH_CHORD, "+⁷", true, false),
    (MU_CHORD, "μ", true, true),
    (SIX_NINE_CHORD, "6/9", true, false),
];

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum ChordStyling{ Std, Extended, SpelledOut }

fn bit_on(num: usize, bit: usize) -> bool{
    let mut t = 1 << bit;
    t &= num;
    t != 0
}

impl Chord{
    pub fn new(intervals: &[Note]) -> Self{
        Chord(intervals.to_owned())
    }

    pub fn same_intervals(&self, blueprint: &[Note]) -> bool{
        self.0 == blueprint
    }

    pub fn has_intervals(&self, blueprint: &[Note]) -> bool{
        for note in blueprint{
            if !self.0.contains(note){
                return false;
            }
        }
        true
    }

    pub fn normalized(mut self) -> Self{
        if self.0.contains(&TWELVETH) && !self.0.contains(&PERFECT_FIFTH){
            self.0.push(PERFECT_FIFTH);
        }
        Chord(self.0.into_iter()
            .map(|i| i % (2*OCTAVE))
            .filter(|i| i != &OCTAVE && i != &TWELVETH)
            .collect::<Vec<_>>())
    }

    pub fn to_subseq_chords(&self) -> Vec<Chord>{
        let scale = self.to_scale(0).0;
        let mut sub_scales = HashSet::new();
        let slen = scale.len();
        let rlen = 2u32.pow(slen as u32) as usize;
        for i in 0..rlen{
            let mut subscale = Vec::new();
            for (j,note) in scale.iter().enumerate().take(slen){
                if bit_on(i,j){
                    subscale.push(*note);
                }
            }
            if subscale.len() < 2 { continue; }
            sub_scales.insert(Scale(subscale).into_chord());
        }
        let mut res = sub_scales.into_iter().collect::<Vec<Chord>>();
        res.sort_by(|a,b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        res
    }

    pub fn into_subseq_chords(self) -> Vec<Chord>{
        self.to_subseq_chords()
    }

    pub fn quality(&self, basestr: String, lower: bool, style: ChordStyling) -> String{
        // Just print intervals
        let spelled_out = |basestr: String|{
            let mut spelled_out = basestr;
            spelled_out.push('[');
            for int in &self.0{
                spelled_out.push_str(&interval_chord_extension(*int));
            }
            spelled_out.push(']');
            spelled_out
        };
        if style == ChordStyling::SpelledOut{
            return spelled_out(basestr);
        }
        let mut lowercase = String::new();
        for c in basestr.chars(){
            for l in c.to_lowercase(){
                lowercase.push(l);
            }
        }
        let mut minorcase = String::new();
        minorcase.push_str(&basestr);
        minorcase.push('m');
        let minorstr = if lower{ lowercase }
        else{ minorcase };
        let sname = |major_base| if major_base { basestr.clone() } else { minorstr.clone() };
        // Find exact matches in the book
        for (pattern,postfix,majorstr,ext) in STD_CHORD_BOOK{
            if pattern != &self.0 { continue; }
            if *ext && style == ChordStyling::Std { continue; }
            let mut name = sname(*majorstr);
            name.push_str(postfix);
            return name
        }
        // Extended chords
        let mut name = String::new();
        let mut baselen = 0;
        for (pattern,postfix,majorstr,ext) in STD_CHORD_BOOK{
            if *ext && style == ChordStyling::Std { continue; }
            if self.0.len() <= pattern.len() { continue; }
            if baselen >= pattern.len() { continue; }
            let base = self.0.iter().take(pattern.len()).copied().collect::<Vec<Note>>();
            if &base != pattern { continue; }
            baselen = pattern.len();
            name = sname(*majorstr);
            name.push_str(postfix);
        }
        let ext_name = |bl,mut name: String|{
            if bl >= self.0.len() { return name; }
            let ol = name.len();
            name.push('(');
            self.0.iter().skip(bl).for_each(|int|name.push_str(&interval_chord_extension(*int)));
            name.push(')');
            if name.len() == ol + 2{
                name.pop(); name.pop();
            }
            name
        };
        if baselen > 0 { return ext_name(baselen,name); }
        //Sus chords, maybe extended
        baselen = 0;
        for (pattern,postfix,_,ext) in STD_CHORD_BOOK{
            if *ext && style == ChordStyling::Std { continue; }
            if self.0.len() < pattern.len() { continue; }
            if baselen >= pattern.len() { continue; }
            let base = self.0.iter().take(pattern.len()).copied().collect::<Vec<Note>>();
            let res = pattern.iter().zip(base.iter()).fold(10, |res,(ba,se)|{
                if res == 0 { 0 }
                else {
                    if se == ba { 10 }
                    else if se == &MAJOR_SECOND && (ba == &MINOR_THIRD || ba == &MAJOR_THIRD) { 2 }
                    else if se == &PERFECT_FOURTH && (ba == &MINOR_THIRD || ba == &MAJOR_THIRD) { 4 }
                    else { 0 }.min(res)
                }
            });
            if res == 0 || res == 10 { continue; }
            baselen = pattern.len();
            name = sname(true);
            name.push_str(postfix);
            name.push_str(&format!("sus{}", res));
        }
        if baselen > 0 { return ext_name(baselen,name); }
        // Default to spelling out
        spelled_out(basestr)
    }

    pub fn as_string(&self, styling: ChordStyling) -> String{
        self.quality("X".to_string(), true, styling)
    }
}

impl ToScale for Chord{
    fn to_scale(&self, root: Note) -> Scale{
        let mut scale = vec![root];
        for int in &self.0{
            scale.push(root + *int);
        }
        Scale(scale)
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Default)]
pub struct RootedChord{
    pub root: Note,
    pub chord: Chord,
}

impl RootedChord{
    pub fn from_chord(root: Note, chord: Chord) -> Self{
        Self{ root, chord }
    }

    pub fn from_intervals(root: Note, intervals: &[Note]) -> Self{
        Self{ root, chord: Chord::new(intervals) }
    }

    pub fn from_scale(scale: Scale) -> Self{
        if scale.is_empty() { Self{ root: 0, chord: Chord(Vec::new()) } }
        else if scale.len() == 1 { Self{ root: scale.0[0], chord: Chord(Vec::new()) } }
        else { Self::from_chord(scale.0[0], scale.into_chord()) }
    }

    pub fn to_scale(&self) -> Scale{
        let mut scale = vec![self.root];
        for int in &self.chord.0{
            scale.push(self.root + *int);
        }
        Scale(scale)
    }

    fn normalized(self) -> Self{
        Self {
            root: self.root % OCTAVE,
            chord: self.chord.normalized(),
        }
    }

    pub fn to_subseq_chords(&self) -> Vec<RootedChord>{
        let scale = self.to_scale().0;
        let mut sub_scales = Vec::new();
        let slen = scale.len();
        let rlen = 2u32.pow(slen as u32) as usize;
        for i in 0..rlen{
            let mut subscale = Vec::new();
            for (j,note) in scale.iter().enumerate().take(slen){
                if bit_on(i,j){
                    subscale.push(*note);
                }
            }
            if subscale.len() < 2 { continue; }
            let subroot = subscale[0];
            let subchord = Scale(subscale).into_chord();
            sub_scales.push(Self::from_chord(subroot, subchord));
        }
        sub_scales.sort_by(|a,b| a.chord.len().cmp(&b.chord.len()).then(a.root.cmp(&b.root)).then(a.chord.cmp(&b.chord)));
        sub_scales
    }

    pub fn into_subseq_chords(self) -> Vec<RootedChord>{
        self.to_subseq_chords()
    }

    pub fn to_chordtone_wholetone_scale(&self) -> Scale{
        let mut res = Vec::new();
        let scale = self.to_scale();
        if scale.len() < 4 { return Scale(res); }
        for (i,note) in scale.0.iter().enumerate().take(4){
            res.push(*note);
            let between = if scale.len() > i + 4 { scale.0[i + 4] - OCTAVE }
            else { *note + MAJOR_SECOND };
            res.push(between);
        }
        Scale(res)
    }

    pub fn to_inversion(&self) -> RootedChord{
        let mut scale = self.to_scale();
        if scale.is_empty() { return RootedChord::default(); }
        let mut root = scale.0[0];
        if scale.len() == 1 { return RootedChord::from_intervals(root, &[]); }
        let top = scale.0[scale.len() - 1];
        while root < top {
            root += OCTAVE;
        }
        scale.0.remove(0);
        scale.0.push(root);
        Self::from_scale(scale)
    }

    pub fn all_inversions(&self) -> Vec<RootedChord>{
        let len = self.chord.len() + 1;
        let mut inv = self.clone();
        let mut res = Vec::new();
        for _ in 0..len{
            inv = inv.to_inversion();
            res.push(inv.clone());
        }
        res
    }

    pub fn as_string(&self, lower: bool, styling: ChordStyling) -> String{
        let root = self.root.to_pc().to_string_name(); //NamedNote::from_note(self.root).to_string_name();
        self.chord.quality(root, lower, styling)
    }
}

#[derive(PartialEq,Eq,Hash,Clone)]
pub struct RelativeChord{
    pub root: Note,
    pub chord: Chord,
}

impl RelativeChord{
    pub fn from_chord(root: Note, chord: Chord) -> Self{
        Self{ root, chord }
    }

    pub fn from_intervals(root: Note, intervals: &[Note]) -> Self{
        Self{ root, chord: Chord::new(intervals) }
    }

    pub fn from_template(semis: Note, intervals: &[Note]) -> Self{
        Self{ root: semis, chord: Chord::new(intervals) }
    }

    pub fn as_string(&self, lower: bool, styling: ChordStyling) -> String{
        let root = to_degree(self.root);
        self.chord.quality(root, lower, styling)
    }
}

impl std::fmt::Display for RelativeChord{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let root = format!("<X{}{}>", if self.root >= 0 { "+" } else { "" }, self.root);
        let res = self.chord.quality(root, true, ChordStyling::Extended);
        write!(f, "{}", res)
    }
}

pub fn print_chords(chords: &[Chord], sep: &str, styling: ChordStyling){
    let len = chords.len();
    if len == 0 { return; }
    for chord in chords.iter().take(len - 1){
        print!("{}{}", chord.as_string(styling), sep);
    }
    println!("{}", &chords[len - 1].as_string(styling));
}

pub fn scale_chords(steps: &Steps, chord_size: usize) -> Vec<Chord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in note_iter(0, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(0, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(Scale(chord).into_chord());
    }
    chords
}

pub fn rooted_scale_chords(steps: &Steps, tonic: Note, chord_size: usize) -> Vec<RootedChord>{
    let len = steps.len();
    let mut chords = Vec::new();
    for (i, _) in note_iter(tonic, &steps.0).enumerate().take(len){
        let mut chord = Vec::new();
        for note in note_iter(tonic, &steps.0).skip(i).step_by(2).take(chord_size){
            chord.push(note);
        }
        chords.push(RootedChord::from_scale(Scale(chord)));
    }
    chords
}

pub fn strs_scale_chords_roman(steps: &Steps, size: usize, styling: ChordStyling) -> Vec<String>{
    let chords = scale_chords(steps, size);
    let mut res = Vec::new();
    for (i, chord) in chords.iter().enumerate(){
        res.push(chord.quality(to_roman_num(i + 1), true, styling));
    }
    res
}

pub fn strs_scale_chords(steps: &Steps, tonic: Note, size: usize, styling: ChordStyling) -> Vec<String>{
    let chords = rooted_scale_chords(steps, tonic, size);
    let mut res = Vec::new();
    for chord in chords.iter(){
        res.push(chord.as_string(true, styling));
    }
    res
}

pub fn scale_subseq_chords(scale: Scale) -> Vec<RootedChord>{
    if scale.len() < 3 { return Vec::new(); }
    let steps = scale.to_steps();
    let root = scale.0[0];
    let mut sub_scales = HashSet::new();
    let slen = scale.len();
    for (i, _) in note_iter(root, &steps.0).enumerate().take(slen){
        let lscale = note_iter(root, &steps.0).skip(i).take(slen).collect::<Vec<_>>();
        let subchords = RootedChord::from_scale(Scale(lscale)).into_subseq_chords();
        sub_scales.extend(subchords.into_iter().map(|sc| sc.normalized()));
    }
    let mut sub_scales = sub_scales.into_iter().collect::<Vec<_>>();
    sub_scales.sort_by(|a,b| a.chord.len().cmp(&b.chord.len()).then(a.root.cmp(&b.root)).then(a.chord.cmp(&b.chord)));
    sub_scales
}

pub fn steps_subseq_chords(steps: Steps) -> Vec<Vec<Chord>>{
    let mut scale = steps.into_scale(0);
    scale.0.pop();
    let mut table = vec![0; 12];
    for (i,note) in scale.0.iter().enumerate(){
        table[(*note).max(0) as usize] = i;
    }
    let subs = scale_subseq_chords(scale.clone());
    let mut cells = vec![vec![]; scale.len()];
    for s in subs.into_iter(){
        let index = table[s.root.max(0) as usize];
        cells[index].push(s.chord);
    }
    cells
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_chords_strings(){
        assert_eq!(Chord::new(&[MAJOR_THIRD,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("X"));
        assert_eq!(Chord::new(&[MINOR_THIRD,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("x"));
        assert_eq!(Chord::new(&[MINOR_THIRD,DIMINISHED_FIFTH]).as_string(ChordStyling::Std), String::from("x°"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,DIMINISHED_FIFTH]).as_string(ChordStyling::Std), String::from("X[♮3♭5]"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,DIMINISHED_FIFTH]).as_string(ChordStyling::Extended), String::from("X°"));
        assert_eq!(Chord::new(&[MINOR_THIRD]).as_string(ChordStyling::Std), String::from("X[♭3]"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("Xsus2"));
        assert_eq!(Chord::new(&[PERFECT_FOURTH,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("Xsus4"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,AUGMENTED_FIFTH]).as_string(ChordStyling::Std), String::from("X+"));
        assert_eq!(Chord::new(&[MINOR_THIRD,AUGMENTED_FIFTH]).as_string(ChordStyling::Std), String::from("X[♭3♭6]"));
        assert_eq!(Chord::new(&[MINOR_THIRD,AUGMENTED_FIFTH]).as_string(ChordStyling::Extended), String::from("x+"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,PERFECT_FOURTH]).as_string(ChordStyling::Std), String::from("X[♮2♮4]"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,PERFECT_FOURTH]).as_string(ChordStyling::Extended), String::from("Xssus"));
        assert_eq!(Chord::new(&[MINOR_SECOND,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("Xphry"));
        assert_eq!(Chord::new(&[AUGMENTED_FOURTH,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("Xlyd"));
        assert_eq!(Chord::new(&[MINOR_SECOND,DIMINISHED_FIFTH]).as_string(ChordStyling::Std), String::from("Xloc2"));
        assert_eq!(Chord::new(&[PERFECT_FOURTH,DIMINISHED_FIFTH]).as_string(ChordStyling::Std), String::from("Xloc4"));
        assert_eq!(Chord::new(&[MINOR_SECOND,PERFECT_FOURTH,DIMINISHED_FIFTH]).as_string(ChordStyling::Std), String::from("X[♭2♮4♭5]"));
        assert_eq!(Chord::new(&[MINOR_SECOND,PERFECT_FOURTH,DIMINISHED_FIFTH]).as_string(ChordStyling::Extended), String::from("Xo"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,PERFECT_FIFTH,MAJOR_SIXTH]).as_string(ChordStyling::Std), String::from("X⁶"));
        assert_eq!(Chord::new(&[MINOR_THIRD,PERFECT_FIFTH,MAJOR_SIXTH]).as_string(ChordStyling::Std), String::from("x⁶"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,PERFECT_FIFTH,MAJOR_SEVENTH]).as_string(ChordStyling::Std), String::from("X∆"));
        assert_eq!(Chord::new(&[MINOR_THIRD,PERFECT_FIFTH,MINOR_SEVENTH]).as_string(ChordStyling::Std), String::from("x-"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,PERFECT_FIFTH,MINOR_SEVENTH]).as_string(ChordStyling::Std), String::from("X⁷"));
        assert_eq!(Chord::new(&[MINOR_THIRD,PERFECT_FIFTH,MAJOR_SEVENTH]).as_string(ChordStyling::Std), String::from("X-∆"));
        assert_eq!(Chord::new(&[MINOR_THIRD,DIMINISHED_FIFTH,MINOR_SEVENTH]).as_string(ChordStyling::Std), String::from("xø"));
        assert_eq!(Chord::new(&[MINOR_THIRD,DIMINISHED_FIFTH,DIMINISHED_SEVENTH]).as_string(ChordStyling::Std), String::from("x°⁷"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,AUGMENTED_FIFTH,MINOR_SEVENTH]).as_string(ChordStyling::Std), String::from("X+⁷"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,MAJOR_THIRD,PERFECT_FIFTH]).as_string(ChordStyling::Std), String::from("X[♮2♮3♮5]"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,MAJOR_THIRD,PERFECT_FIFTH]).as_string(ChordStyling::Extended), String::from("Xμ"));
        assert_eq!(Chord::new(&[MAJOR_THIRD,PERFECT_FIFTH,MAJOR_SIXTH,NINETH]).as_string(ChordStyling::Std), String::from("X6/9"));
        assert_eq!(Chord::new(&[MAJOR_SECOND,PERFECT_FIFTH,MAJOR_SEVENTH,FLAT_NINETH,SHARP_ELEVENTH]).as_string(ChordStyling::Std), String::from("X∆sus2(♭9♯11)"));
        assert_eq!(Chord::new(&[PERFECT_FOURTH,PERFECT_FIFTH,MINOR_SEVENTH,SHARP_NINETH,SHARP_THIRTEENTH]).as_string(ChordStyling::Std), String::from("X-sus4(♯9♯13)"));
    }
}
