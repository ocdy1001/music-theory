extern crate music_gen;
use music_gen::tones::*;
use music_gen::theory::*;

fn main(){
    test2();
}

fn test2(){
    print_notes(&chord_from_intervals(0, &MAJOR_TRIAD), ", ");
    print_notes(&chord_from_intervals(A4, &MINOR_TRIAD), ", ");
    print_notes(&chord_from_scale(A4, &ionian_scale_steps(), &NINETH_DEGREES) ,", ");
    println!("{}", NamedChord::from_chord(&chord_from_intervals(A4, &MAJOR_SEVENTH_TETRAD)).as_string());
    for mode in 0..7{
        //print_chords(&scale_chords(&mode_of_scale(ionian_scale_steps(), mode), 4), ",\t");
        print_strings(&strs_scale_chords_roman(&mode_of_scale(ionian_scale_steps(), mode), 3), ",\t");
    }
    print_chords(&scale_chords(&mode_of_scale(satie_scale(), 0), 3), ",\t");
    print_chords(&scale_chords(&mode_of_scale(greek_dorian_chromatic(), 0), 3), ",\t");
    print_chords(&scale_chords(&mode_of_scale(greek_dorian(), 0), 3), ",\t");
}

fn test1(){
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let tonef = &spread(6, 1.003, 0.0, sine_sample);
    let volf = &hit_lin_quot_quad(40.0,0.2, 1.0, 2);
    let hzf = &arg_id;
    let passf = &smooth_pass(10.0);
    
    let mut score = Score::new();
    score.new_staff();
    score.new_bar(0, Bar::new(Key::std_key(), 120.0, TimeSig::new(1, 1.0)));
    score.add_note(barnote(NamedNote::A(4).to_note(), 1.0), false, 0);
    score.add_note(barnote(NamedNote::A(4).to_note(), 1.0), false, 0);
    score.add_note(barnote(NamedNote::Cs(4).to_note(), 1.0), true, 0);
    score.add_note(barnote(NamedNote::E(4).to_note(), 1.0), true, 0);
    
    println!("{}", score.as_string(0));

    score.render_to_track_stereo(0, &mut track, 3.0, 1.0, 0.0, tonef, volf, hzf, passf);
    track.trim_end(0.001);
    track.normalize(0.99);
    track.render("test.wav");
}

fn test0(){
    //let scale = ionian_mode(NamedNote::A(4).to_note(), AEOLIAN);
    let scale = notes_of_mode(NamedNote::A(3).to_note(), satie_scale(), 0);
    print_notes(&scale, "\t");
    let sr = 44100;
    let mut track = music_gen::tones::Track::new(sr, 2);
    let volf = &hit_lin_quot_quad(40.0,0.2, 1.0, 2);
    let mut time = 0;
    for note in scale{
        let hz = to_pitch(note);
        //tone_to_track(&mut track, time, sr * 3, 1.0, 0.0, 0.0, hz, &sine_sample, &arg_id, volf, &arg_id);
        tone_to_track_stereo(&mut track, time, sr * 3 as usize, 1.0, 0.0, hz, &spread(6, 1.003, 0.0, sine_sample), volf, &arg_id, &smooth_pass(10.0));
        time += sr/2;
    }
    track.trim_end(0.001);
    track.normalize(0.99);
    track.render("test.wav");
}
