use std::fmt::{Debug, Display};
use std::str::FromStr;

pub fn check_display_parse_equivalence<T>(text: &str, value: T) -> anyhow::Result<()>
where
    T: PartialEq + Display + Debug + FromStr<Err = anyhow::Error>,
{
    let actualtext = value.to_string();
    assert_eq!(text, actualtext);
    let actualvalue = actualtext.parse()?;
    assert_eq!(&value, &actualvalue);
    // Juuuust in case, though I cannot currently think of how to make this fail:
    let actualtext2 = actualvalue.to_string();
    assert_eq!(text, actualtext2);
    Ok(())
}
