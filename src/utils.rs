use std::{collections::HashMap, usize};
use utf8_slice::slice;

use crate::Args;
use aecir::style::{Color, ColorName, Format};

pub fn display_sorted(data: HashMap<String, usize>, args: Args) {
    let mut sorted: Vec<(String, usize)> = data.into_iter().collect::<Vec<(String, usize)>>();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let total = sorted.iter().fold(0, |acc, x| acc + x.1);
    let max = sorted[0].1;
    let command_indentation = max.to_string().len();

    let (list, limitted) = if let Some(limit) = args.count {
        (&sorted[0..limit], true)
    } else {
        (&sorted[0..], false)
    };

    for item in list {
        print_command(
            &item.0,
            item.1,
            item.1 as f32 * 100. / total as f32,
            max,
            command_indentation,
            &args,
        );
    }
    if limitted {
        let remaining_total = total - list.iter().fold(0, |acc, x| acc + x.1);
        let remaining_percentage = (remaining_total as f32 * 100. / total as f32) as usize;
        println!(
            "{gray} ...  {remaining_total} (~{remaining_percentage}%) others {reset}",
            gray = if args.pretty {
                Color::Fg(ColorName::LightBlack).to_string()
            } else {
                "".to_string()
            },
            reset = if args.pretty {
                aecir::style::reset_colors()
            } else {
                "".to_string()
            }
        );
    }
}

pub fn print_command(
    command: &str,
    invocations: usize,
    percentage: f32,
    max: usize,
    command_indentation: usize,
    args: &Args,
) {
    let bar: String = format!(
        "{: <10}",
        args.bar
            .to_string()
            .repeat(((invocations as f32 / max as f32) * 10.) as usize)
    );
    if args.pretty {
        println!(
            "{opening_char}{red}{bar_first: <2}{yellow}{bar_second: <3}{green}{bar_third: <5}{reset}{closing_char} \
            {percentage: >5.2}% {gray}{invocations:command_indentation$}{reset}\
            {bold} {command} {reset_style}",
            opening_char = args.bar_open,
            red = Color::Fg(ColorName::Red),
            bar_first = slice(&bar, 0, 2),
            yellow = Color::Fg(ColorName::Yellow),
            bar_second = slice(&bar, 2, 5),
            green = Color::Fg(ColorName::Green),
            bar_third = slice(&bar, 5, 10),
            reset = aecir::style::reset_colors(),
            closing_char = args.bar_close,
            gray = Color::Fg(ColorName::LightBlack),
            bold = Format::Bold,
            reset_style = aecir::style::reset_all(),
        );
    } else {
        println!(
            "{opening_char}{bar}{closing_char} {percentage: >5.2}% {invocations:command_indentation$} {command}",
            opening_char = args.bar_open,
            closing_char = args.bar_close,
        );
    }
}
