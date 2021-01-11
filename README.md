# AdventOfCode_2020_Rust
Going through the Advent of code 2020 challenges using Rust. Each individual 
day is contained in a simple binary package. I solved most problems without
using any external libraries. For some problems however I used the regex-crate
to ease parsing and the fnv-crate to replace the rather slow HashMap in std.

# Building/Running
Assuming Rust/Cargo is installed, simply clone this repository, copy your
input-file to the individual day's root-directory, rename the input-file to
`input` and run `cargo run --release` inside the directory of the desired
day. 

# A word on timing
When measuring the runtime of each individual part of each day, I generally
tried to time each part seperatly without timing the parsing-code. However for
some days it made no sense to time each part seperatly and sometimes the action
of parsing the input into a suitable data-structure mostly solved the first part
of that day. In these cases I then merely timed the total-time or included
parsing-code in the measurements. Furthermore I merely used std::time::Instant
to time my code, which leads to fairly inconsitant measurements. To conclude:
the runtime-measurements are somewhat inconsistant and should be taken with a
grain of salt.

