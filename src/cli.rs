// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use clap::Parser;

/// Lorem Ipsum Generator - Generate placeholder text
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(disable_version_flag = true)]
#[command(arg_required_else_help = false)]
pub struct CliArgs {
    /// Start with 'Lorem ipsum' (1 for true, 0 for false)
    #[arg(short = 's', long, default_value = "1", value_parser = clap::value_parser!(u8).range(0..=1))]
    pub start_with_lorem: u8,

    /// Number of paragraphs
    #[arg(short = 'p', long)]
    pub paragraphs: Option<usize>,

    /// Maximum sentences per paragraph
    #[arg(short = 'm', long = "ms")]
    pub max_sentences: Option<usize>,

    /// Maximum words per sentence
    #[arg(short = 'w', long)]
    pub max_words: Option<usize>,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    _version: Option<bool>,

}
