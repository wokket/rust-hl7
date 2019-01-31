cargo b --release && cp ./target/release/librusthl7.so ./dotnet && dotnet run -c release --project ./dotnet/ConsoleApp1.csproj
