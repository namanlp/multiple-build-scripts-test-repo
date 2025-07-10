#!/bin/bash

for n in {1..15}; 
do
   rustup run nightly ../cargo/target/release/cargo clean
   echo $n $n | rustup run nightly ../cargo/target/release/cargo run
   echo $n
done
