use anyhow::{ensure, Result};
use std::{
    collections::BTreeMap,
    env,
    iter::Peekable,
    process,
    slice::{self},
};

fn usage() -> ! {
    eprintln!("usage: judgefmt --name name -l0 p1 ... pk ... -ln p1 ... pk");
    process::exit(2);
}

fn parse_level(
    levels: &mut BTreeMap<i32, Vec<String>>,
    level: i32,
    args: &mut Peekable<slice::Iter<&str>>,
) -> Result<()> {
    loop {
        match args.peek() {
            Some(s) => {
                if s.starts_with("-l") {
                    return Ok(());
                }
                levels
                    .entry(level)
                    .or_insert(Vec::new())
                    .push(args.next().unwrap().to_string());
            }
            None => {
                return Ok(());
            }
        }
    }
}

fn parse_args(
    levels: &mut BTreeMap<i32, Vec<String>>,
    args: &mut Peekable<slice::Iter<&str>>,
) -> Result<()> {
    loop {
        match args.peek() {
            Some(s) => {
                ensure!(s.starts_with("-l"), "level doesn't start with -l");
                let level: i32 = args.next().unwrap().trim_start_matches("-l").parse()?;
                levels
                    .entry(level)
                    .or_insert(Vec::new());
                parse_level(levels, level, args)?;
            }
            None => {
                return validate_map(levels);
            }
        }
    }
}

fn validate_map(levels: &mut BTreeMap<i32, Vec<String>>) -> Result<()> {
    ensure!(!levels.is_empty(), "empty args");
    for i in 0..levels.len() as i32 {
        ensure!(levels.contains_key(&i), "missing arguments");
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let str_args: Vec<_> = args.iter().map(|s| s.as_str()).collect();
    let (name, rest) = match &str_args[..] {
        ["--name", name, ..] => (name, &str_args[2..]),
        _ => usage(),
    };
    let mut levels = BTreeMap::new();
    let mut it = rest.iter().peekable();
    parse_args(&mut levels, &mut it).unwrap_or_else(|err| {
        eprintln!("{}", err);
        usage();
    });
    let mut lev_lengths = Vec::new();
    let mut min_total = 0;
    for vc in levels.values() {
        let length = vc.iter().fold(0, |sum, s| sum + s.chars().count());
        if vc.len() != 0 {
            min_total = min_total.max(length + 4 * (vc.len() - 1));
        }
        lev_lengths.push(length);
    }
    let line = "-".repeat(min_total + 2);
    let padding = " ".repeat(name.len() + 1);
    let middle_row = (levels.len() + (levels.len() - 1)) / 2;
    let mut row = 0;
    for (level, vc) in &levels {
        if *level != 0 {
            if row == middle_row {
                print!("{name}:");
            } else {
                print!("{padding}");
            }
            println!("{line}");
            row += 1;
        }
        if row == middle_row {
            print!("{name}:");
        } else {
            print!("{padding}");
        }
        if vc.len() == 0 {
            println!();
            row += 1;
            continue;
        }
        let spaces = min_total - lev_lengths[*level as usize];
        let quo = spaces / if vc.len() == 1 { 2 } else { vc.len() - 1 };
        let rem = spaces % if vc.len() == 1 { 2 } else { vc.len() - 1 };
        let seg = " ".repeat(quo);
        if vc.len() == 1 {
            println!(" {}{}{}{} ", seg, " ".repeat(rem), vc[0], seg);
            row += 1;
        } else {
            for (i, s) in vc.iter().enumerate() {
                if i == 0 {
                    print!(" {}{}{}", s, seg, " ".repeat(rem));
                } else if i != vc.len() - 1 {
                    print!("{}{}", s, seg);
                } else {
                    println!("{} ", s);
                    row += 1;
                }
            }
        }
    }
}
