#!/bin/bash
cargo b --release 

#strip out debugging constants etc from the library. reduces *.nix library from 4Mb back to (a windows equalling) 1.4Mb
strip ./target/release/librusthl7.so

cp ./target/release/librusthl7.so ./dotnet 
sudo dotnet run -c release --project ./dotnet/ConsoleApp1.csproj
