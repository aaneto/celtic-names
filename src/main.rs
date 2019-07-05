mod markov_chain;
mod name_generator;
mod name_scraper;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Celtic Markov Names")
        .version("0.1.5")
        .author("Adilson Neto <almeidneto@gmail.com>")
        .about("Generate a random celtic name using markov chains")
        .arg(
            Arg::with_name("number_of_names")
                .short("n")
                .long("number_of_names")
                .help("Set the number of names to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size_of_names")
                .short("s")
                .long("size_of_names")
                .help("Set the size of the names to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("chain_size")
                .short("c")
                .long("chain_size")
                .help("Set the order of the markov chain to use on name generation")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("use_crate_markov")
                .short("m")
                .long("use_crate_markov")
                .help("Check to use the markov crate instead of this tool custom implementation"),
        )
        .get_matches();

    let number_of_names = matches
        .value_of("number_of_names")
        .and_then(|num_string| num_string.parse().ok())
        .unwrap_or(10);

    let size_of_names = matches
        .value_of("size_of_names")
        .and_then(|size_string| size_string.parse().ok())
        .unwrap_or(7);

    let chain_size = matches
        .value_of("chain_size")
        .and_then(|chain_string| chain_string.parse().ok())
        .unwrap_or(3);

    let use_crate_markov = matches.is_present("use_crate_markov");

    println!("Running with parameters: ");
    println!("number_of_names: {}", number_of_names);
    println!("size_of_names: {}", size_of_names);
    println!("chain_size: {}", chain_size);
    println!("use_crate_markov: {}", use_crate_markov);

    let mut generator = new_name_generator(use_crate_markov, chain_size);

    println!();
    println!("Fetching names...");
    for name in name_scraper::find_names_in_page() {
        generator.feed(name);
    }

    for i in 0..number_of_names {
        let name = generator.generate(size_of_names);

        println!("Name({}): {}", 1 + i, name);
    }

    dont_disappear::any_key_to_continue::default();
}

fn new_name_generator(
    use_crate_markov: bool,
    order: usize,
) -> Box<dyn name_generator::NameGenerator> {
    if use_crate_markov {
        Box::new(markov::Chain::of_order(order))
    } else {
        Box::new(markov_chain::MarkovChain::new(order))
    }
}
