use owo_colors::OwoColorize;

const ASCII_ART: &str = r#" ██████╗███████╗███████╗██████╗
██╔════╝██╔════╝██╔════╝██╔══██╗
╚█████╗ █████╗  █████╗  ██║  ██║
 ╚═══██╗██╔══╝  ██╔══╝  ██║  ██║
██████╔╝███████╗███████╗██████╔╝
╚═════╝ ╚══════╝╚══════╝╚═════╝ "#;

/// Prints the header.
pub fn print_header() {
    println!();
    for line in ASCII_ART.lines() {
        let split_at = line
            .char_indices()
            .nth(8)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let (s_letter, rest) = line.split_at(split_at);

        println!("{}{}", s_letter.bright_red().bold(), rest.dimmed());
    }
    println!(
        "{}",
        format_args!(
            "v{}{}{}",
            env!("CARGO_PKG_VERSION"),
            " - ",
            env!("CARGO_PKG_DESCRIPTION")
        )
        .dimmed()
    );
}
