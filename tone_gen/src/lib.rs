use colored::Colorize;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::fmt::Display;

pub enum Note {
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

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        match rng.gen_range(0..=11) {
            0 => Note::Ab,
            1 => Note::A,
            2 => Note::Bb,
            3 => Note::B,
            4 => Note::C,
            5 => Note::Db,
            6 => Note::D,
            7 => Note::Eb,
            8 => Note::E,
            9 => Note::F,
            10 => Note::Gb,
            _ => Note::G,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered_note = match self {
            Note::Ab => "A♭".red(),
            Note::A => "A♮".bright_red(),
            Note::Bb => "B♭".yellow(),
            Note::B => "B♮".bright_yellow(),
            Note::C => "C♮".bright_green(),
            Note::Db => "D♭".blue(),
            Note::D => "D♮".bright_blue(),
            Note::Eb => "E♭".magenta(),
            Note::E => "E♮".bright_magenta(),
            Note::F => "F♮".bright_purple(),
            Note::Gb => "G♭".cyan(),
            Note::G => "G♮".bright_cyan(),
        };

        write!(f, "{}", rendered_note.bold().italic())
    }
}

pub enum Chord {
    Major(Note),
    Minor(Note),
    Seven(Note),
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

pub struct Sheet(pub Vec<Vec<RootString>>);

impl FromIterator<Vec<RootString>> for Sheet {
    fn from_iter<T: IntoIterator<Item = Vec<RootString>>>(iter: T) -> Self {
        Sheet(iter.into_iter().collect())
    }
}

impl From<(usize, usize)> for Sheet {
    fn from((total, length): (usize, usize)) -> Self {
        (0..total)
            .map(|_| {
                (0..length)
                    .map(|_| rand::random::<RootString>())
                    .collect::<Vec<RootString>>()
            })
            .collect::<Sheet>()
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🎸🎸🎸 Generating Chords 🎶🎶🎶\n").ok();

        self.0.iter().for_each(|line| {
            let run = line
                .iter()
                .map(|string| string.to_string())
                .collect::<Vec<String>>()
                .join(" -> ");

            writeln!(f, "🎼 {}\n", run).ok();
        });

        Ok(())
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
