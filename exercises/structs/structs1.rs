// structs1.rs
// Address all the TODOs to make the tests pass!
use std::fmt;

struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}

struct ColorTupleStruct<'a>(&'a str, &'a str);

struct ColorUnitStruct;

impl fmt::Debug for ColorUnitStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitStruct")
    }
}

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct { name: "green", hex: "#00FF00" };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let unit_struct = ColorUnitStruct; 
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
