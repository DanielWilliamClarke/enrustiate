use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::fmt::Display;

pub enum Note {
    A,
    Ab,
    B,
    C,
    Cb,
    D,
    Db,
    E,
    F,
    Fb,
    G,
    Gb,
}

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        match rng.gen_range(0..=11) {
            0 => Note::A,
            1 => Note::Ab,
            2 => Note::B,
            3 => Note::C,
            4 => Note::Cb,
            5 => Note::D,
            6 => Note::Db,
            7 => Note::E,
            8 => Note::F,
            9 => Note::Fb,
            10 => Note::G,
            _ => Note::Gb,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered_note = match self {
            Note::A => "A ",
            Note::Ab => "A♭",
            Note::B => "B ",
            Note::C => "C ",
            Note::Cb => "C♭",
            Note::D => "D ",
            Note::Db => "D♭",
            Note::E => "E ",
            Note::F => "F ",
            Note::Fb => "F♭",
            Note::G => "G ",
            Note::Gb => "G♭",
        };

        write!(f, "{}", rendered_note)
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
            Chord::Major(note) => write!(f, "{} Major", note),
            Chord::Minor(note) => write!(f, "{} Minor", note),
            Chord::Seven(note) => write!(f, "{} Seven", note),
        }
    }
}

pub struct Sheet(pub Vec<Vec<Chord>>);

impl FromIterator<Vec<Chord>> for Sheet {
    fn from_iter<T: IntoIterator<Item = Vec<Chord>>>(iter: T) -> Self {
        Sheet(iter.into_iter().collect())
    }
}

impl From<(usize, usize)> for Sheet {
    fn from((total, length): (usize, usize)) -> Self {
        (0..total)
            .map(|_| {
                (0..length)
                    .map(|_| rand::random::<Chord>())
                    .collect::<Vec<Chord>>()
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
                .map(|chord| chord.to_string())
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
            .collect::<Vec<Chord>>();

        assert_eq!(flat.len(), total * length);
    }
}
