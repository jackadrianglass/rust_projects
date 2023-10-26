use crate::generator::{Groove, Grouping, Structure};

const URL: &str = "https://www.mikeslessons.com/gscribe";
const DEFAULT_SNARE_NOTE: char = 'g';
const DEFAULT_HIGH_HAT_NOTE: char = 'x';
const DEFAULT_BASS_NOTE: char = 'o';

pub fn gen_groovescribe_link(groove: &Groove) -> String{
    let high_hat_string =
        gen_groovescribe_seq(&groove.high_hat, &groove.structure, DEFAULT_HIGH_HAT_NOTE);
    let bass_string = gen_groovescribe_seq(&groove.bass, &groove.structure, DEFAULT_BASS_NOTE);
    let snare_string = gen_groovescribe_seq(&groove.snare, &groove.structure, DEFAULT_SNARE_NOTE);
    let ts = &groove.structure.time_signature;

    let mut url: String = URL.to_string();
    url += &format!("?TimeSig={}/{}", ts.number, ts.divisions);

    url += &format!(
        "&Div={}",
        ts.divisions * groove.structure.grouping.num_beats() as i32
    );
    url += &format!(
        "&Tempo=80&Measures={}{}{}{}",
        groove.structure.bars,
        beat_to_url(&high_hat_string, &"H".to_string()),
        beat_to_url(&snare_string, &"S".to_string()),
        beat_to_url(&bass_string, &"K".to_string())
    );

    url
}

fn beat_to_url(beat: &String, letter: &String) -> String {
    return format!("&{}={}|", letter, beat);
}

fn gen_groovescribe_seq(sequence: &[i8], structure: &Structure, letter: char) -> String {
    let mut vec: Vec<String> = Vec::new();
    let mut num_displayed = 0;
    for beat in sequence {
        if num_displayed % structure.time_signature.number == 0 {
            vec.push("|".to_string());
        }

        vec.push(gen_groovescribe_beat(&beat, &structure.grouping, letter));
        num_displayed += 1;
    }
    return vec.join("");
}

fn gen_groovescribe_beat(beat: &i8, grouping: &Grouping, letter: char) -> String {
    let mut tick = 1 << (grouping.num_beats() - 1);
    let mut string = String::new();

    for _ in 0..grouping.num_beats() {
        if beat & tick == 0 {
            string.push('-');
        } else {
            string.push(letter);
        }
        tick = tick >> 1;
    }

    return string;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::TimeSignature;

    #[test]
    fn test_gen_groovescribe_beat() {
        assert_eq!(
            "xxxx",
            gen_groovescribe_beat(&15, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "xxx-",
            gen_groovescribe_beat(&14, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "xx-x",
            gen_groovescribe_beat(&13, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "xx--",
            gen_groovescribe_beat(&12, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "x-xx",
            gen_groovescribe_beat(&11, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "x-x-",
            gen_groovescribe_beat(&10, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "x--x",
            gen_groovescribe_beat(&9, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "x---",
            gen_groovescribe_beat(&8, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "-xxx",
            gen_groovescribe_beat(&7, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "-xx-",
            gen_groovescribe_beat(&6, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "-x-x",
            gen_groovescribe_beat(&5, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "-x--",
            gen_groovescribe_beat(&4, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "--xx",
            gen_groovescribe_beat(&3, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "--x-",
            gen_groovescribe_beat(&2, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "---x",
            gen_groovescribe_beat(&1, &Grouping::Quaternary, 'x')
        );
        assert_eq!(
            "----",
            gen_groovescribe_beat(&0, &Grouping::Quaternary, 'x')
        );
    }

    #[test]
    fn test_gen_groovescribe_seq() {
        let time_signature = TimeSignature {
            number: 4,
            divisions: 4,
        };
        let structure = Structure {
            time_signature: time_signature,
            grouping: Grouping::Quaternary,
            bars: 4,
        };
        let test_vec: Vec<i8> = vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let expected = vec![
            "|", "xxxx", "xxx-", "xx-x", "xx--", "|", "x-xx", "x-x-", "x--x", "x---", "|", "-xxx",
            "-xx-", "-x-x", "-x--", "|", "--xx", "--x-", "---x", "----",
        ]
        .join("");
        let actual = gen_groovescribe_seq(&test_vec, &structure, 'x');

        assert_eq!(actual, expected);
    }
}
