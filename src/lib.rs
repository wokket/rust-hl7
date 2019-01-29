#![feature(test)]

extern crate itertools;
extern crate libc;
extern crate test;

mod field_parser;
pub mod message_parser;
mod segment_parser;

use itertools::Itertools;
use libc::c_char;
use std::ffi::{CStr, CString};

/// A repeat of a field is a set of 0 or more sub component values.
/// Currently all values are stored as their original string representations.  Methods to convert
/// the values to their HL7-spec types is outside the scope of the parser.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Repeat {
    pub sub_components: Vec<String>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Field {
    pub repeats: Vec<Repeat>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Segment {
    pub fields: Vec<Field>,
}

/// A Message is an entire HL7 message parsed into it's consitituent segments, fields, repeats and subcomponents
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Message {
    /// The source string that was parsed to form this message.
    /// We need our own copy to ensure the &str's are referencing a string that lives long enough in an FFI scenario
    input: String,
    pub segments: Vec<Segment>,
}

impl Repeat {
    pub fn get_as_string(&self) -> String {
        if self.sub_components.len() == 0 {
            return "".to_string();
        } else {
            return self.sub_components.join("^");
        }
    }
}

impl Field {
    /// Returns a single String built from all the repeats.segment_parser
    /// This value includes HL7 delimiter values between repeats, components and sub components.segment_parser
    /// A copy  of the oringla data is made here (rather than a &str) as the value is expected to be returned to external callers who
    /// shouldn't have to keep the entire source message alive
    pub fn get_all_as_string(&self) -> String {
        if self.repeats.len() == 0 {
            return "".to_string();
        }

        self.repeats.iter().map(|r| r.get_as_string()).join("~")
    }
}

impl Message {
    pub fn new(input: String) -> Message {
        Message {
            input: input,
            segments: Vec::new(),
        }
    }

    pub fn build_from_input(&mut self) {
        let iter = self.input.split('\r');

        for segment_value in iter {
            if segment_value.len() == 0 {
                //we've hit the end-of-message blank line delimnter, proceed no further
                break;
            }

            let segment = segment_parser::SegmentParser::parse_segment(segment_value);
            self.segments.push(segment);
        }
    }

    pub fn get_segments(&self, segment_type: &str) -> Vec<&Segment> {
        self.segments
            .iter()
            .filter(|segment| segment.fields[0].get_all_as_string() == segment_type)
            .collect()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TestStruct {
    pub count: i32,
}

impl TestStruct {
    #[no_mangle]
    pub extern "C" fn free_string(s: *mut c_char) {
        unsafe {
            if s.is_null() {
                return;
            }
            CString::from_raw(s)
        };
    }

    #[no_mangle]
    pub extern "C" fn free_message(msg_ptr: *mut Message) {
        unsafe {
            if msg_ptr.is_null() {
                return;
            }

            let obj = Box::from_raw(msg_ptr);
            println!("Freeing message: {:?}", obj);
        };
    }

    #[no_mangle]
    pub extern "C" fn build_message(s: *const c_char) -> *mut Message {
        println!("Into build_message...");

        let c_str = unsafe {
            assert!(!s.is_null());
            CStr::from_ptr(s)
        };

        let r_str = c_str.to_str().unwrap().to_string();

        println!("Building message from string value: {}", r_str);

        let m = message_parser::MessageParser::parse_message(r_str);

        println!("Message init to: {:?}", m);

        let return_ptr = Box::into_raw(Box::new(m)); //box onto the heap for stability, then get a raw ptr we can pass outside.

        return_ptr
    }

    #[no_mangle]
    pub extern "C" fn get_field(ptr: *const Message) -> *mut c_char {
        println!("Into get_field()");
        let obj = unsafe { &*ptr }; // unsafe { Box::from_raw(ptr) };
        println!("unboxed obj");
        println!("Obj = {:?}", obj);

        let result = obj.segments[0].fields[0].get_all_as_string();

        println!("Returning field value: {}", result);

        let c_string = CString::new(result).unwrap();
        c_string.into_raw()
    }
}

impl TestStruct {
    #[no_mangle]
    pub extern "C" fn new_test_struct_ref() -> *mut TestStruct {
        let s = TestStruct { count: 5 };
        let b = Box::new(s);
        Box::into_raw(b)
    }

    #[no_mangle]
    pub extern "C" fn free_struct(ptr: *mut TestStruct) {
        println!("Into free...");
        unsafe {
            let b = Box::from_raw(ptr); //this return value falling out of scope free's the memory
            println!("Freeing struct with counter: {}", b.count);
        }
    }

    #[no_mangle]
    pub extern "C" fn new_test_struct() -> TestStruct {
        TestStruct { count: 5 }
    }

    #[no_mangle]
    pub extern "C" fn add_to_struct(ptr: *mut TestStruct, x: i32) -> i32 {
        println!("into Add");

        let mut obj = unsafe { Box::from_raw(ptr) };

        println!("Calling add on {:?}", obj);

        obj.count += x;
        let result = obj.count;

        //we need to tell rust not to drop the struct again
        let temp = Box::into_raw(obj);
        if temp != ptr {
            println!("New ptr != passed pointer!  Future use of ptr is prob invalid!");
        }

        result
    }

    #[no_mangle]
    pub extern "C" fn add_to_struct_val(mut self, x: i32) -> TestStruct {
        println!("Calling add on {:?}", self);
        self.count += x;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_get_all_as_string_single_simple_value() {
        let r = Repeat {
            sub_components: vec!["Simple Repeat".to_string()],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Simple Repeat");
    }

    #[test]
    fn repeat_get_all_as_string_multi_components() {
        let r = Repeat {
            sub_components: vec!["Multiple".to_string(), "Components".to_string()],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Multiple^Components");
    }

    #[test]
    fn field_get_all_as_string_single_simple_value() {
        let f = Field {
            repeats: vec![Repeat {
                sub_components: vec!["Simple Repeat".to_string()],
            }],
        };

        let actual = f.get_all_as_string();
        assert_eq!(actual, "Simple Repeat");
    }

    #[test]
    fn field_get_all_as_string_multiple_repeats() {
        let f = Field {
            repeats: vec![
                Repeat {
                    sub_components: vec!["Repeat 1".to_string()],
                },
                Repeat {
                    sub_components: vec!["Repeat 2".to_string()],
                },
            ],
        };

        let actual = f.get_all_as_string();
        assert_eq!(actual, "Repeat 1~Repeat 2");
    }

    #[test]
    fn test_segment_lookup() {
        let msg =
            message_parser::MessageParser::parse_message("MSH|fields\rOBR|segment\r".to_string()); //note the trailing \r
                                                                                                   /*let expected = Message {
                                                                                                       segments: vec![
                                                                                                           Segment {
                                                                                                               fields: vec![
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["test"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["fields"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                               ],
                                                                                                           },
                                                                                                           Segment {
                                                                                                               fields: vec![
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["another"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["segment"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                               ],
                                                                                                           },
                                                                                                       ],
                                                                                                   };*/

        let expected = Segment {
            fields: vec![
                Field {
                    repeats: vec![Repeat {
                        sub_components: vec!["OBR".to_string()],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        sub_components: vec!["segment".to_string()],
                    }],
                },
            ],
        };

        let result = msg.get_segments("OBR");
        assert!(result.len() == 1);
        assert_eq!(expected, *result[0]);
    }
}
