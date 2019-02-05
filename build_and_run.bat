cargo b --release 
copy .\target\release\rusthl7.dll .\dotnet /y 
dotnet run -c release --project .\dotnet\ConsoleApp1.csproj
