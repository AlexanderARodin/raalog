#[derive(PartialEq)]
pub enum LinePosition {
    NULL,
    Single,
    First,
    Middle,
    Last,
}

pub fn convert_to_lines(src: Option<&str>) -> Vec<(&str, LinePosition)> {
    let mut result = Vec::new();
    if let Some(s) = src {
        for line in s.lines() {
            result.push((line, LinePosition::Middle));
        }
    }else{
        result.push(("", LinePosition::NULL));
        return result;
    }

    let n = result.len();
    match n {
        0 => {
            result.push(("", LinePosition::Single));
        },
        1 => {
            result[0].1 = LinePosition::Single;
        },
        _ => {
            result[0].1 = LinePosition::First;
            result[n-1].1 = LinePosition::Last;
        },
    }
    result
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic_validation {
    use super::*;

    #[test]
    fn null_line() {
        let parsed = convert_to_lines(None);

        assert!("" == parsed[0].0);
        assert!(LinePosition::NULL == parsed[0].1);
    }
    #[test]
    fn empty_line() {
        let src = String::from("");
        let parsed = convert_to_lines(Some(&src));

        assert!(src == parsed[0].0);
        assert!(LinePosition::Single == parsed[0].1);
    }
    #[test]
    fn single_line() {
        let src = String::from("12345");
        let parsed = convert_to_lines(Some(&src));

        assert!(src == parsed[0].0);
        assert!(LinePosition::Single == parsed[0].1);
    }
    #[test]
    fn two_line() {
        let src = String::from("123\n45");
        let parsed = convert_to_lines(Some(&src));

        assert!( "123" == parsed[0].0);
        assert!(LinePosition::First == parsed[0].1);
        assert!( "45" == parsed[1].0);
        assert!(LinePosition::Last == parsed[1].1);
    }
    #[test]
    fn three_line() {
        let src = String::from("1\n23\n45");
        let parsed = convert_to_lines(Some(&src));

        assert!( "1" == parsed[0].0);
        assert!(LinePosition::First == parsed[0].1);
        assert!( "23" == parsed[1].0);
        assert!(LinePosition::Middle == parsed[1].1);
        assert!( "45" == parsed[2].0);
        assert!(LinePosition::Last == parsed[2].1);
    }
}
