# Producers consumers

Create a program that will launch multiple threads in rust.
N producers and M consumers (they are all threads).
The producers create random numbers in a loop sleeping for 1 milli second after
creating each number. Use "rand::Rng".
The consumers will consume these random numbers and will write them to "/dev/null".

Use [this](https://docs.rs/crossbeam-channel/latest/crossbeam_channel/)
to pass messages between the producers and the consumers.
