use crate::generator::{Grouping, Structure, Groove};

const SPACING: &str = "  ";

fn display_ascii_notes(structure: &Structure) {
    print!("  ");

    let ts = &structure.time_signature;
    let num_beats = ts.number * structure.bars as i32;

    let top_bar = match structure.grouping {
        Grouping::Binary => " _ ",
        Grouping::Ternary => " _3_ ",
        Grouping::Quaternary => " _____ ",
    };

    for num in 0..num_beats {
        if num % ts.number == 0 {
            print!("  ");
        }
        print!(" {}{}", top_bar, SPACING);
    }
    print!("\n  ");

    let mid_bar = match structure.grouping {
        Grouping::Binary => "| |",
        Grouping::Ternary => "| | |",
        Grouping::Quaternary => "|-|-|-|",
    };

    for num in 0..num_beats {
        if num % ts.number == 0 {
            print!("| ");
        }
        print!(" {}{}", mid_bar, SPACING);
    }
}

fn display_beat(beat: &i8, grouping: &Grouping, letter: char) {
    let mut tick = 1 << (grouping.num_beats() - 1);
    for _ in 0..grouping.num_beats() {
        if beat & tick == 0 {
            print!(" .");
        } else {
            print!(" {}", letter);
        }
        tick = tick >> 1;
    }
    print!("{}", SPACING);
}

fn display_seq(sequence: &[i8], structure: &Structure, letter: char) {
    let mut num_displayed = 0;
    for beat in sequence {
        // When you cross the bar line, display a bar
        if num_displayed % structure.time_signature.number == 0 {
            print!("| ");
        }
        display_beat(&beat, &structure.grouping, letter);
        num_displayed += 1;
    }
}

pub fn display_groove(groove: &Groove) {
    display_ascii_notes(&groove.structure);

    let ts = &groove.structure.time_signature;
    if ts.number > 9 {
        print!("\n{}", ts.number);
    } else {
        // note the trailing whitespace
        print!("\n{} ", ts.number);
    }
    display_seq(&groove.high_hat, &groove.structure, 'H');

    print!("\n--");
    display_seq(&groove.snare, &groove.structure, 'S');

    if ts.divisions > 9 {
        print!("\n{}", ts.divisions);
    } else {
        print!("\n{} ", ts.divisions);
    }
    display_seq(&groove.bass, &groove.structure, 'B');
    print!("\n");
}

