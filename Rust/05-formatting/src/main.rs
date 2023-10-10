use std::fmt::{Alignment, Debug, Display, Formatter, Result};

struct Point2D {
    x: i32,
    y: i32,
}

impl Debug for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Point2D")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

struct ShowFormatterSettings;

impl Display for ShowFormatterSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let filler = f.fill();
        let alignment = if let Some(a) = f.align() {
            match a {
                Alignment::Left => "left",
                Alignment::Right => "right",
                Alignment::Center => "center",
            }
        } else {
            "<none>"
        };
        let width = if let Some(w) = f.width() {
            w.to_string()
        } else {
            "<none>".to_string()
        };
        let precision = if let Some(p) = f.precision() {
            p.to_string()
        } else {
            "<none>".to_string()
        };
        let plus_flag = f.sign_plus();
        let minus_flag = f.sign_minus();
        let hash_flag = f.alternate();
        let zero_flag = f.sign_aware_zero_pad();

        writeln!(f, "Formatter {{")?;
        writeln!(f, "    filler: '{filler}'")?;
        writeln!(f, "    alignment: {alignment}")?;
        writeln!(f, "    width: {width}")?;
        writeln!(f, "    precision: {precision}")?;
        writeln!(f, "    '+' flag: {plus_flag}")?;
        writeln!(f, "    '-' flag: {minus_flag}")?;
        writeln!(f, "    '#' flag: {hash_flag}")?;
        writeln!(f, "    '0' flag: {zero_flag}")?;
        write!(f, "}}")
    }
}

fn main() {
    let p = Point2D { x: -1, y: 3 };
    let q = ShowFormatterSettings {};

    println!("p = {p:#?}");
    println!("{q}");
}
