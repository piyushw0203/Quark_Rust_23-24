

#!/bin/bash

folder_name="Quark_Rust_23-24"
#this checks whether the folder is already present`
if [ -d "$folder_name" ]; then
  echo "The folder '$folder_name' already exists."
else
  git clone https://github.com/apoorvapendse/Quark_Rust_23-24 "$folder_name"
fi

cd "./Quark_Rust_23-24/Apoorva's Tasks/task2/src/"


# Run the cargo command
# this itself checks whether all the deps are installed 
# i have assumed that the user already has github ssh token and rust compiler
cargo run

