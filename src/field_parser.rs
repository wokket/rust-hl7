/// This module parses _within_ a field only.  It is expected that only a single field (i.e. the value between pipes) is passed at a time.
/// Note that is NOT indented for use with the MSH-2 field (escape character definition field)
pub struct FieldParser;

use super::*;

impl FieldParser {
    /// This method is expecting to receive a single Repeat worth of data only...
    /// If called with an empty repeat (ie "") an empty vec (ie []) is returned
    fn get_components_naiive(input: &str) -> Vec<String> {
        if input.len() == 0 {
            return Vec::<String>::new(); //empty, no-alloc
        }

        let result = input
            .split("^")
            .map(|e| e.to_string()) // copy slice to a brand new string, we need a seperate obj in order to return it.
            .collect();
        result
    }

    /// This method splits a field value (ie, the thing between the pipes) into a set of 0 or more repeats
    /// If called with an empty field value (ie "") an empty vec (ie []) is returned.
    fn get_repeats(input: &str) -> Vec<&str> {
        if input.len() == 0 {
            return Vec::<&str>::new();
        }

        let result = input.split("~").collect();
        result
    }

    pub fn parse_field(input: &str) -> Field {
        let mut repeats = Vec::new(); //TODO: Add reasonable minimum capacity

        for repeat_value in FieldParser::get_repeats(input) {
            let subcomponents = FieldParser::get_components_naiive(repeat_value);
            let repeat = Repeat {
                sub_components: subcomponents,
            };

            repeats.push(repeat);
        }

        Field { repeats: repeats }
    }

    pub fn parse_field_alt(input: &str) -> Field {
        let mut repeats = Vec::new(); //benchs faster assuming no value

        for repeat_value in FieldParser::get_repeats(input) {
            let subcomponents = FieldParser::get_components_naiive(repeat_value);
            let repeat = Repeat {
                sub_components: subcomponents,
            };

            repeats.push(repeat);
        }

        Field { repeats: repeats }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_splitting() {
        let mut result = FieldParser::get_components_naiive("test");
        assert_eq!(["test"], result.as_slice());

        result = FieldParser::get_components_naiive("test value");
        assert_eq!(["test value"], result.as_slice());

        result = FieldParser::get_components_naiive("test^value");
        assert_eq!(["test", "value"], result.as_slice());

        result = FieldParser::get_components_naiive("test^^value");
        assert_eq!(["test", "", "value"], result.as_slice());

        result = FieldParser::get_components_naiive("test^^value^");
        assert_eq!(["test", "", "value", ""], result.as_slice());

        result = FieldParser::get_components_naiive("PO BOX 23523^WELLINGTON^ON^98111");
        assert_eq!(
            ["PO BOX 23523", "WELLINGTON", "ON", "98111"],
            result.as_slice()
        );

        result = FieldParser::get_components_naiive("");
        assert_eq!([] as [&str; 0], result.as_slice());
    }

    #[test]
    fn test_repeat_splitting() {
        let mut result = FieldParser::get_repeats("test");
        assert_eq!(["test"], result.as_slice());

        result = FieldParser::get_repeats("test value");
        assert_eq!(["test value"], result.as_slice());

        result = FieldParser::get_repeats("test~value");
        assert_eq!(["test", "value"], result.as_slice());

        result = FieldParser::get_repeats("test^value~another^^value^");
        assert_eq!(["test^value", "another^^value^"], result.as_slice());

        result = FieldParser::get_repeats("");
        assert_eq!([] as [&str; 0], result.as_slice());
    }

    #[test]
    fn build_simple_field() {
        let input = "Test Value";
        let expected = Field {
            repeats: vec![Repeat {
                sub_components: vec!["Test Value".to_string()],
            }],
        };

        let actual = FieldParser::parse_field(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn build_simple_field_with_repeats() {
        let input = "Test Value~another Value";
        let expected = Field {
            repeats: vec![
                Repeat {
                    sub_components: vec!["Test Value".to_string()],
                },
                Repeat {
                    sub_components: vec!["another Value".to_string()],
                },
            ],
        };

        let actual = FieldParser::parse_field(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn build_actual_field_with_repeats_and_subcomponents() {
        let input = "260 GOODWIN CREST DRIVE^^BIRMINGHAM^AL^35 209^^M~NICKELL’S PICKLES^10000 W 100TH AVE^BIRMINGHAM^AL^35200^^O";
        let expected = Field {
            repeats: vec![
                Repeat {
                    sub_components: vec![
                        "260 GOODWIN CREST DRIVE".to_string(),
                        "".to_string(),
                        "BIRMINGHAM".to_string(),
                        "AL".to_string(),
                        "35 209".to_string(),
                        "".to_string(),
                        "M".to_string(),
                    ],
                },
                Repeat {
                    sub_components: vec![
                        "NICKELL’S PICKLES".to_string(),
                        "10000 W 100TH AVE".to_string(),
                        "BIRMINGHAM".to_string(),
                        "AL".to_string(),
                        "35200".to_string(),
                        "".to_string(),
                        "O".to_string(),
                    ],
                },
            ],
        };

        let actual = FieldParser::parse_field(input);
        assert_eq!(expected, actual);
    }

}
