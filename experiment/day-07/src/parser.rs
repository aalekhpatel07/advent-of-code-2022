use std::hash::Hash;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::*,
    combinator::{map_res, map, cut},
    sequence::{preceded, separated_pair},
    branch::alt, error::context, multi::many1,
};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct File {
    pub name: String,
    pub size: usize
}

impl Hash for File {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write(self.name.as_bytes());
        hasher.finish();
    }
}


#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum FileLike {
    File(File),
    Dir(String)
}


impl FileLike {
    pub fn from_str(s: &str) -> IResult<&str, Self> {
        alt((
            map(
                context("directory", preceded(tag("dir "), cut(alpha1))),
                |dir_name: &str| Self::Dir(dir_name.into())
            ),
            map(
                context(
                "file",
                        separated_pair(
                            map_res(digit1, str::parse::<usize>), space1,  many1(anychar)
                        )
                    )
                ,
                |(num, file_name)| Self::File(File { name: file_name.iter().collect::<String>(), size: num })
            )
        ))(s)
    }
}


#[derive(Debug, PartialEq)]
pub enum Operation {
    Ls,
    Cd(String)
}

impl Operation {
    pub fn from_str(s: &str) -> IResult<&str, Self> {
        let (input, _) = tag("$ ")(s)?;
        alt((
            map(
                context("cd", preceded(tag("cd "), cut(alt((alphanumeric1, tag("/"), tag("..")))))),
                |sym_string: &str| Self::Cd(sym_string.to_owned())
            ),
            map(tag("ls"), |_| Self::Ls))
        )(input)
    }
}


impl<'a> TryFrom<&'a str> for Operation {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Operation::from_str(value).map(|(_, op)| op)
    }
}


impl<'a> TryFrom<&'a str> for FileLike {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        FileLike::from_str(value).map(|(_, fl)| fl)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("$ cd /", Operation::Cd("/".to_owned()) ; "Change directory to root.")]
    #[test_case("$ cd ..", Operation::Cd("..".to_owned()) ; "Change directory to previous.")]
    #[test_case("$ cd ddgtnw", Operation::Cd("ddgtnw".to_owned()) ; "Change directory to some alphanumeric.")]
    #[test_case("$ ls", Operation::Ls ; "Ls")]
    fn parse_operation(text: &str, operation: Operation) {
        let result = Operation::from_str(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().1, operation);
    }

    #[test]
    fn operation_try_from_str() {
        let operation: Operation = "$ cd /".try_into().unwrap();
        assert_eq!(operation, Operation::Cd("/".to_owned()));
    }

    #[test_case("dir ddgtnw", FileLike::Dir("ddgtnw".to_owned()) ; "Dir check")]
    #[test_case("57336 tbq.wvz", FileLike::File(File { name: "tbq.wvz".to_owned(), size: 57336 }) ; "File check")]
    fn parse_filelike(text: &str, file_like: FileLike) {
        let result = FileLike::from_str(text);
        let result = result.unwrap();
        // assert!(result.is_ok());
        assert_eq!(result.1, file_like);
    }
}