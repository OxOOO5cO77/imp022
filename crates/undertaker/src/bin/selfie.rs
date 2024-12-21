use std::collections::HashMap;
use std::env;
use std::io::ErrorKind;

#[derive(Default)]
struct Extents {
    right: usize,
    bottom: usize,
}

impl Extents {
    fn extend(&mut self, other: Extents) {
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut args = env::args();
    args.next().ok_or(ErrorKind::Other)?; // discard name
    let filename = args.next().ok_or(ErrorKind::Other)?;
    let layout_file = std::fs::read_to_string(&filename)?;

    let mut max_extents = Extents::default();

    let mut type_map = HashMap::new();

    for line in layout_file.lines() {
        let mut chars = line.chars();
        let (kind, remain) = (chars.next().ok_or(ErrorKind::Other)?, chars.as_str());
        let extents = parse_extents(remain);

        if let Some(extents) = extents {
            max_extents.extend(extents);
        }

        let entry = type_map.entry(kind).or_insert(0);
        *entry += 1;
    }

    print!("[{}] {}x{}: ", filename, max_extents.right, max_extents.bottom);
    print!(" {} defines", type_map.get(&'$').unwrap_or(&0));
    print!(" {} layouts", type_map.get(&'/').unwrap_or(&0));
    print!(" {} shapes", type_map.get(&'#').unwrap_or(&0));
    print!(" {} images", type_map.get(&'*').unwrap_or(&0));
    print!(" {} text", type_map.get(&'&').unwrap_or(&0));
    print!(" {} input", type_map.get(&'?').unwrap_or(&0));
    println!();
    Ok(())
}

fn make_extents(size: &str, pos: &str) -> Option<Extents> {
    let (width, height) = size.split_once(',')?;
    let w = width.parse::<usize>().ok()?;
    let h = height.parse::<usize>().ok()?;

    let mut components = pos.split(',');
    let x = components.next()?.parse::<usize>().ok()?;
    let y = components.next()?.parse::<usize>().ok()?;

    let extents = Extents {
        right: x + w,
        bottom: y + h,
    };

    Some(extents)
}

fn parse_extents(remain: &str) -> Option<Extents> {
    let (at_left, at_right) = remain.split_once('@')?;
    let (_, size) = at_left.split_once('%')?;
    let (pos, _) = at_right.split_once('!').unwrap_or((at_right, ""));

    make_extents(size, pos)
}
