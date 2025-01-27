#!/usr/bin/env bash

if [ $# -ne 1 ]
  then
    echo "Need exactly one argument, the problem number."
    exit
fi


(touch "src/Problem_$1.rs" &&
echo -e 'struct Solution {}\n\nimpl Solution {\n\n}\n\n\nfn main() {
    println!("{}", Solution::);
}' >> "src/Problem_$1.rs" ) &&
echo -e "\n[[bin]]
name = \"Problem_$1\"
path = \"src/Problem_$1.rs\"" >> Cargo.toml ||
echo "Failed to generate file"
