// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    // ???

    // Pattern Matching
    /*
    let val = match time_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    };
    val
    */
    
    // Using if 
    if time_of_day < 22 {
        return Some(5);
    } else if time_of_day <= 23 {
        return Some(0);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?

        // https://stackoverflow.com/questions/64996954/how-can-i-pull-data-out-of-an-option-for-independent-use
        // If you can guarantee that it's impossible for the value to be None, then you can use:
        // let origin = resp.get("origin").unwrap();
        // Or:
        // let origin = resp.get("origin").expect("This shouldn't be possible!");

        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
