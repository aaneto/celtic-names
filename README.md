# Celtic Names
This is a simple CLI tool to generate a celtic name using
the markov chain method.

The data used to create the markov chain is scrapped from the
celtic personal names of roman britain, [CPNRB](https://www.asnc.cam.ac.uk/personalnames/)

[![Build Status](https://travis-ci.org/aaneto/celtic-names.svg?branch=master)](https://travis-ci.org/aaneto/celtic-names)
## Using the Script

To get an overview of the parameters, use --help:

```bash
$ cargo run -- --help

Celtic Markov Names 0.1.0
Adilson Neto <almeidneto@gmail.com>
Generate a random celtic name using markov chains

USAGE:
    celtic_names [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                Prints help information
    -m, --use_crate_markov    Check to use the markov crate instead of this tool custom implementation
    -V, --version             Prints version information

OPTIONS:
    -c, --chain_size <chain_size>              Set the order of the markov chain to use on name generation
    -n, --number_of_names <number_of_names>    Set the number of names to generate
    -s, --size_of_names <size_of_names>        Set the size of the names to generate
```

Running the script without parameters will run the defaults:

```bash
$ cargo run --release

Running with parameters: 
number_of_names: 10
size_of_names: 7
chain_size: 3
use_crate_markov: false

Fetching names...
Name(1): Uorix
Name(2): Ccianus
Name(3): Nibelia
Name(4): Utiustu
Name(5): Meco
Name(6): Ntokoxo
Name(7): Ecaratu
Name(8): Thianus
Name(9): Ecionus
Name(10): Esus
```
