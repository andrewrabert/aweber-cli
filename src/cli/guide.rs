use clap::{error::ErrorKind, Arg, ArgAction, ArgMatches, Command};

const GUIDE: &str = include_str!("../../GUIDE.md");

pub(crate) fn command() -> Command {
    Command::new("guide")
        .about("Read the built-in usage guide")
        .arg(
            Arg::new("list")
                .long("list")
                .action(ArgAction::SetTrue)
                .conflicts_with("section")
                .help("List section names"),
        )
        .arg(Arg::new("section").help("Print one section by name"))
}

fn sections() -> impl Iterator<Item = (&'static str, &'static str)> {
    GUIDE
        .split("\n## ")
        .skip(1)
        .filter_map(|section| section.lines().next().map(|name| (name, section)))
}

pub(crate) fn run(matches: &ArgMatches) {
    if matches.get_flag("list") {
        for (name, _) in sections() {
            println!("{name}");
        }
    } else if let Some(requested) = matches.get_one::<String>("section") {
        if let Some((_, section)) =
            sections().find(|(name, _)| name.eq_ignore_ascii_case(requested))
        {
            print!("## {section}");
        } else {
            let names = sections().map(|(name, _)| name).collect::<Vec<_>>();
            clap::Error::raw(
                ErrorKind::InvalidValue,
                format!(
                    "unknown guide section {requested:?}; valid sections: {}",
                    names.join(", ")
                ),
            )
            .exit();
        }
    } else {
        print!("{GUIDE}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_top_level_command_has_a_guide_section() {
        let names = sections().map(|(name, _)| name).collect::<HashSet<_>>();
        assert_eq!(names.len(), sections().count(), "duplicate guide section");
        for command in crate::commands::build_command_tree().get_subcommands() {
            assert!(
                names.contains(command.get_name()),
                "missing section for {}",
                command.get_name()
            );
        }
    }
}
