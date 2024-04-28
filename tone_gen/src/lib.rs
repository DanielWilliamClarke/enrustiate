use colored::Colorize;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use simple_tones::{Note, NoteDuration, Player};

use std::fmt::Display;
use std::io::stdin;

pub enum RootNote {
    Ab,
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
}

impl Distribution<RootNote> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RootNote {
        match rng.gen_range(0..=11) {
            0 => RootNote::Ab,
            1 => RootNote::A,
            2 => RootNote::Bb,
            3 => RootNote::B,
            4 => RootNote::C,
            5 => RootNote::Db,
            6 => RootNote::D,
            7 => RootNote::Eb,
            8 => RootNote::E,
            9 => RootNote::F,
            10 => RootNote::Gb,
            _ => RootNote::G,
        }
    }
}

impl Display for RootNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered_note = match self {
            RootNote::Ab => "A♭".red(),
            RootNote::A => "A♮".bright_red(),
            RootNote::Bb => "B♭".yellow(),
            RootNote::B => "B♮".bright_yellow(),
            RootNote::C => "C♮".bright_green(),
            RootNote::Db => "D♭".blue(),
            RootNote::D => "D♮".bright_blue(),
            RootNote::Eb => "E♭".magenta(),
            RootNote::E => "E♮".bright_magenta(),
            RootNote::F => "F♮".bright_purple(),
            RootNote::Gb => "G♭".cyan(),
            RootNote::G => "G♮".bright_cyan(),
        };

        write!(f, "{}", rendered_note.bold().italic())
    }
}

pub enum Chord {
    Major(RootNote),
    Minor(RootNote),
    Seven(RootNote),
}

impl Distribution<Chord> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Chord {
        match rng.gen_range(0..=2) {
            0 => Chord::Major(rand::random()),
            1 => Chord::Minor(rand::random()),
            _ => Chord::Seven(rand::random()),
        }
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chord::Major(note) => write!(f, "{} {}", note, "Major".dimmed()),
            Chord::Minor(note) => write!(f, "{} {}", note, "Minor".dimmed()),
            Chord::Seven(note) => write!(f, "{} {}", note, "Seven".dimmed()),
        }
    }
}

pub enum RootString {
    Six(Chord),
    Five(Chord),
}

impl Distribution<RootString> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RootString {
        match rng.gen_range(0..=1) {
            0 => RootString::Six(rand::random()),
            _ => RootString::Five(rand::random()),
        }
    }
}

impl Display for RootString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootString::Six(chord) => write!(f, "{}: {}", "6".underline(), chord),
            RootString::Five(chord) => write!(f, "{}: {}", "5".underline(), chord),
        }
    }
}

impl From<&RootString> for Note {
    fn from(rs: &RootString) -> Self {
        // Guitar string octaves
        // https://music.stackexchange.com/questions/32715/what-do-the-terms-e2-a2-d3-g3-b3-e4-actually-mean#:~:text=The%20number%20indicates%20the%20note%27s%20octave.&text=So%2C%20E2%20(lowest%20of%20your,)%2C%20it%27s%20the%20same%20thing.
        let (chord, octave) = match rs {
            RootString::Six(chord) => (chord, 2),
            RootString::Five(chord) => (chord, 2)
        };

        let note = match chord {
            Chord::Major(root) => root,
            Chord::Minor(root) => root,
            Chord::Seven(root) => root
        };

        let note  = match note {
            RootNote::Ab => "Ab",
            RootNote::A => "A",
            RootNote::Bb => "Bb",
            RootNote::B => "B",
            RootNote::C => "C",
            RootNote::Db => "Db",
            RootNote::D => "D",
            RootNote::Eb => "Eb",
            RootNote::E => "E",
            RootNote::F => "F",
            RootNote::Gb => "Gb",
            RootNote::G => "G",
        };

        // https://docs.rs/simple_tones/0.1.0/simple_tones/
        let note = format!("{}{}", note, octave).parse().unwrap();

        Note::new(note, NoteDuration::Whole)
    }
}

pub struct Line(pub Vec<RootString>);

impl FromIterator<RootString> for Line {
    fn from_iter<T: IntoIterator<Item = RootString>>(iter: T) -> Self {
        Line(iter.into_iter().collect())
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let run = self.0
            .iter()
            .map(|string| string.to_string())
            .collect::<Vec<String>>()
            .join(" -> ");

        writeln!(f, "🎼 {}", run)
    }
}

impl Line {
    pub fn play (&self, bpm: u32) {
        let sheet = self.0
            .iter()
            .map(Note::from)
            .collect::<Vec<Note>>();

        let np = Player::from_bpm(bpm);
        np.play(sheet.iter());
    }
}

pub struct Sheet(pub Vec<Line>);

impl FromIterator<Line> for Sheet {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        Sheet(iter.into_iter().collect())
    }
}

impl From<(usize, usize)> for Sheet {
    fn from((total, length): (usize, usize)) -> Self {
        (0..total)
            .map(|_| {
                (0..length)
                    .map(|_| rand::random::<RootString>())
                    .collect::<Line>()
            })
            .collect::<Sheet>()
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🎸🎸🎸 Generating Chords 🎶🎶🎶\n").ok();

        self.0.iter().for_each(|line| {
            writeln!(f, "🎼 {}", line).ok();
        });

        Ok(())
    }
}

pub enum SheetPlayerOption {
    Play,
    Exit,
}

pub struct SheetPlayer {
    sheet: Sheet,
    index: usize,
    bpm: u32
}

impl From<Sheet> for SheetPlayer
{
    fn from(sheet: Sheet) -> SheetPlayer {
       SheetPlayer { sheet, index: 0, bpm: 300 }
    }
}

impl SheetPlayer {
    pub fn read_input (&mut self) -> Result<SheetPlayerOption, String> {
        println!("Please select option [r: repeat, n: next, p: prev, i: select index, s: change bpm, g: generate line, e: exit]");
        let mut input= String::new();

        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct option");

        match input.trim() {
            "r" => Ok(SheetPlayerOption::Play),
            "n" => {
                if self.index < self.sheet.0.len() - 1 {
                    self.index += 1;
                } else {
                    self.index = 0;
                };

                Ok(SheetPlayerOption::Play)
            }
            "p" => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.index = self.sheet.0.len() - 1;
                }

                Ok(SheetPlayerOption::Play)
            },
            "i" => {
                println!("Select an index between 1 and {}", self.sheet.0.len());
                let mut selected_index= String::new();
                stdin()
                    .read_line(&mut selected_index)
                    .expect("Did not enter an index");

                let input_result = match selected_index.trim().parse::<usize>() {
                    Ok(index) if index > 0 && index <= self.sheet.0.len() => Ok(index - 1),
                    Ok(index) => Err(format!("Invalid index: {}", index)),
                    Err(err) => Err(format!("Invalid index input: {}", err)),
                };

                match input_result {
                    Ok(index) =>{
                        self.index = index;
                        Ok(SheetPlayerOption::Play)
                    }
                    Err(err) => Err(err)
                }
            },
            "s" => {
                println!("Set a new BPM");
                let mut selected_bpm= String::new();
                stdin()
                    .read_line(&mut selected_bpm)
                    .expect("Did not enter an index");

                match selected_bpm.trim().parse::<u32>() {
                    Ok(bpm) => {
                        self.bpm = bpm;

                        Ok(SheetPlayerOption::Play)
                    }
                    Err(err) => Err(format!("Invalid bpm input: {}", err)),
                }
            },
            "g" => {
                println!("Set new line length");
                let mut selected_length= String::new();
                stdin()
                    .read_line(&mut selected_length)
                    .expect("Did not enter an index");

                match selected_length.trim().parse::<usize>() {
                    Ok(length) => {
                        let line = (0..length)
                            .map(|_| rand::random::<RootString>())
                            .collect::<Line>();

                        self.sheet.0.push(line);
                        // Set the index to the new index
                        self.index = self.sheet.0.len() - 1;

                        Ok(SheetPlayerOption::Play)
                    }
                    Err(err) => Err(format!("Invalid line length input: {}", err)),
                }
            },
            "e" => {
                Ok(SheetPlayerOption::Exit)
            }
            v => Err(format!("Invalid input!: {}", v))
        }
    }

    pub fn play (&self) {
        println!("Playing line: {} -> {}", self.index + 1, self.sheet.0[self.index]);
        self.sheet.0[self.index].play(self.bpm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_runs_of_length() {
        let total = 100;
        let length = 100;

        let result = Sheet::from((total, length));

        let flat = result
            .0
            .into_iter()
            .flat_map(|inner_vec| inner_vec)
            .collect::<Vec<RootString>>();

        assert_eq!(flat.len(), total * length);
    }
}
