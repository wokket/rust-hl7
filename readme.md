## An experimental HL7 library ##

Totally nothing like production ready!

Basic usage:

```
    let input = "PID|Field1|Component1^Component2|Component1^Sub-Component1&Sub-Component2^Component3|Repeat1~Repeat2\r";
    let message = pipe_parser::message_parser::MessageParser::parse_message(input);

    println!("message: {:?}", message);
```

This first cut is intended to parse from a multiline text blob into a tree of strings, representing all the different components.

Interpreting this components (type conversion, determing which fields they represent etc) is a future problem.


Early (apples to oranges) benchmarks this library at 12us to parse the same message nHAPI takes 350us (it creates a fully populated, strongly typed object, we just have Vec's of &str's everywhere....)