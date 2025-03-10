//! Header parsing.

use winnow::{
    ascii::*,
    combinator::*,
    error::{StrContext, StrContextValue},
    prelude::*,
    token::*,
};

use crate::error::{Error, Result};

/// References to the strings of the file where the header is and it's values.
#[derive(Debug, Clone, PartialEq)]
pub struct Header<'a> {
    /// The part following the '#' until the '=' symbol.
    pub name: &'a str,
    /// Comma separated values following the first '=' symbol of the line.
    pub values: Vec<&'a str>,
}

impl<'a> Header<'a> {
    /// Create a tuple from the name and the values.
    pub fn decompose(self) -> (&'a str, Vec<&'a str>) {
        (self.name, self.values)
    }

    /// Parse a header string.
    ///
    /// The string shouldn't contain a '#' character at the begin nor a newline
    /// at the end.
    fn from_str(input: &mut &'a str) -> winnow::Result<Self> {
        // Take the hash symbol at the beginning and the whitespace
        ('#', space0)
            .context(StrContext::Label("header hash"))
            .parse_next(input)?;

        // Take the name
        let name = alphanumeric1
            .context(StrContext::Label("header name"))
            .parse_next(input)?;

        // Ignore the '=' symbol, although it is required
        (space0, '=', space0)
            .context(StrContext::Label("header equality symbol"))
            .parse_next(input)?;

        // Get the comma-space separated values
        let mut values: Vec<&str> = separated(
            0..,
            // Get until the ',' character and trim the spaces
            delimited(space0, take_till(1.., (',', '\r', '\n')), space0)
                .context(StrContext::Label("header value")),
            ',',
        )
        .context(StrContext::Label("header values"))
        .context(StrContext::Expected(StrContextValue::Description(
            "comma separated list of values",
        )))
        .parse_next(input)?;

        // Remove empty values
        if values == [""] {
            values.clear();
        }

        // Take the newline
        (space0, take_while(0.., ('\n', '\r'))).parse_next(input)?;

        Ok(Self { name, values })
    }
}

/// Parse the headers of the GEF file.
///
/// Return the parsed headers and a reference to the rest of the file.
pub(crate) fn parse_headers(mut gef: &'_ str) -> Result<(&'_ str, Vec<Header<'_>>)> {
    parse_headers_impl
        .parse_next(&mut gef)
        // Convert the winnow error to our own error type
        .map_err(|err| Error::Parsing(err.to_string()))
}

/// Parse the list of headers implementation.
pub(crate) fn parse_headers_impl<'a>(
    input: &mut &'a str,
) -> winnow::Result<(&'a str, Vec<Header<'a>>)> {
    // Skip initial whitespace
    multispace0.parse_next(input)?;

    // Loop over all sequences starting with # until the newline character
    let (values, _) = repeat_till(0.., Header::from_str, not('#'))
        .context(StrContext::Label("headers"))
        .parse_next(input)?;

    Ok((input, values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_header() {
        assert_eq!(
            Header::from_str(&mut "#A= 1").unwrap(),
            Header {
                name: "A",
                values: vec!["1"]
            }
        );

        assert_eq!(
            Header::from_str(&mut "#A = 1").unwrap(),
            Header {
                name: "A",
                values: vec!["1"]
            }
        );

        assert_eq!(
            Header::from_str(&mut "#A= 1, 2").unwrap(),
            Header {
                name: "A",
                values: vec!["1", "2"]
            }
        );
    }

    #[test]
    fn test_empty_header() {
        assert_eq!(
            Header::from_str(&mut "#A=").unwrap(),
            Header {
                name: "A",
                values: vec![]
            }
        );

        let (rest, headers) = parse_headers("#A=\nrest").unwrap();
        assert_eq!(
            headers,
            vec![Header {
                name: "A",
                values: vec![]
            },]
        );
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_header_names() {
        let (rest, headers) = parse_headers("#A= 1\n#B= 2\nrest").unwrap();

        assert_eq!(rest, "rest");
        assert_eq!(
            headers,
            vec![
                Header {
                    name: "A",
                    values: vec!["1"]
                },
                Header {
                    name: "B",
                    values: vec!["2"]
                }
            ]
        );
    }

    #[test]
    fn test_header_empty_lines() {
        let (rest, headers) = parse_headers("#A= 1\n\n\n#B= 2\nrest").unwrap();

        assert_eq!(rest, "rest");
        assert_eq!(
            headers,
            vec![
                Header {
                    name: "A",
                    values: vec!["1"]
                },
                Header {
                    name: "B",
                    values: vec!["2"]
                }
            ]
        );
    }

    #[test]
    fn test_whitespace_prefix() {
        let (rest, headers) = parse_headers("\n\t\n  \n#A= 1\n#B= 2\nrest").unwrap();

        assert_eq!(rest, "rest");
        assert_eq!(
            headers,
            vec![
                Header {
                    name: "A",
                    values: vec!["1"]
                },
                Header {
                    name: "B",
                    values: vec!["2"]
                }
            ]
        );
    }

    #[test]
    fn test_carriage_return() {
        let (data, _) = parse_headers(" #BLA= bla\r\n#EOH=\r\n   data data\r\n   data ").unwrap();

        assert_eq!(data, "   data data\r\n   data ");
    }

    #[test]
    fn test_gef_file() {
        let (csv, _) = parse_headers(
            "
#GEFID = 1,1,0
#COLUMNTEXT = 1, aan
#COLUMNSEPARATOR = ;
#RECORDSEPARATOR = !
#FILEOWNER = DINO
#FILEDATE = 2010,9,1
#PROJECTID = DINO-BOR
#COLUMN = 9
#COLUMNINFO = 1, m, Diepte bovenkant laag, 1
#COLUMNINFO = 2, m, Diepte onderkant laag, 2
#COLUMNINFO = 3, mm, Zandmediaan, 8
#COLUMNINFO = 4, mm, Grindmediaan, 9
#COLUMNINFO = 5, %, Lutum percentage, 3
#COLUMNINFO = 6, %, Silt percentage, 4
#COLUMNINFO = 7, %, Zand percentage, 5
#COLUMNINFO = 8, %, Grind percentage, 6
#COLUMNINFO = 9, %, Organische stof percentage, 7
#COLUMNVOID = 1, -9999.99
#COLUMNVOID = 2, -9999.99
#COLUMNVOID = 3, -9999.99
#COLUMNVOID = 4, -9999.99
#COLUMNVOID = 5, -9999.99
#COLUMNVOID = 6, -9999.99
#COLUMNVOID = 7, -9999.99
#COLUMNVOID = 8, -9999.99
#COLUMNVOID = 9, -9999.99
#LASTSCAN = 44
#REPORTCODE = GEF-BORE-Report,1,0,0
#MEASUREMENTCODE = Onbekend
#TESTID = B25G0304
#XYID = 31000,120870,483400
#ZID = 31000,2.0
#MEASUREMENTVAR = 19, 1, -, aantal peilbuizen
#EOH =
0.00;1.20;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;'Zgh2';'TGR \
             GE';'ZMFO';'CA3';!
1.20;3.10;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;'Zg';'ON';'ZMGO';'FN2';'\
             CA2';!
3
",
        )
        .unwrap();

        assert_eq!(
            csv,
            "0.00;1.20;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;'Zgh2';'TGR \
             GE';'ZMFO';'CA3';!
1.20;3.10;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;-9999.99;'Zg';'ON';'ZMGO';'FN2';'\
             CA2';!
3
"
        );
    }

    #[test]
    fn test_horrible_format() {
        let (csv, headers) = parse_headers(
            "
#GEFID  = 1,0,0
#COLUMN = 2
#COLUMNINFO   = 1, m, Sondeerlengte, 1
#COLUMNINFO   = 2, MPa, Conusweerstand, 2
#COMPANYID   = REDACT, 1, 0
#FILEOWNER   = REDACT
#PROJECTNAME = REDACT ED FILE
#PROJECTID   = 0
#TESTID      = REDACT sondering
#FILEDATE     =2021,3,10
#STARTDATE    =2021,3,10
#STARTTIME    =14,17,45
#LASTSCAN    =769
#PROCEDURECODE = GEF-CPT-Report, 1, 0, 0, -
#ZID = 31000,-2.94, 0.1
#XYID = 31000,0,0, 0.1, 0.1
#EOH=
0.000  0.36
0.033  0.43
0.065  0.64
0.099  0.71
",
        )
        .unwrap();

        assert_eq!(
            headers[0],
            Header {
                name: "GEFID",
                values: ["1", "0", "0"].to_vec(),
            }
        );
        assert_eq!(
            headers[1],
            Header {
                name: "COLUMN",
                values: ["2"].to_vec(),
            }
        );

        assert_eq!(
            csv,
            "0.000  0.36
0.033  0.43
0.065  0.64
0.099  0.71
"
        );

        let (csv, _) = parse_headers(
            "
#GEFID          = 1,1,0
#FILEDATE       = 2021, 04, 12
#REPORTCODE     = GEF-CPT-Report,1,1,0
#COLUMNSEPARATOR= |
#RECORDSEPARATOR= !

#COMMENT=*****************     ADMINISTRATIVE TEST DATA      ********************
#FILEOWNER      = Unknown

#COMMENT=******************    REMARKS ADDED AFTER TEST     *********************

#COMMENT=****************** REFERENCE LEVEL AND COORDINATES *********************
#MEASUREMENTTEXT= 9, Ground Level, fixed horizontal level
#ZID            = 00000,  0.10

#COMMENT=*****************       MEASURED PARAMETERS         ********************
#MEASUREMENTVAR = 20,0.00, MPa    , zero measurement cone before test

#COMMENT=*****************        COLUMN PARAMETERS         *********************
#COLUMN         = 4
#COLUMNVOID     = 4, -99999
#LASTSCAN       = 1184
#EOH            =
  0.00|   0.22|   0.00279|   0.00|!
  0.01|   0.34|   0.00333|   0.00|!
  0.02|   0.47|   0.00387|   0.00|!
  0.03|   0.59|   0.00443|   0.00|!
  0.04|   0.70|   0.00504|   0.00|!
  0.05|   0.80|   0.00574|   0.00|!",
        )
        .unwrap();

        assert_eq!(
            csv,
            "  0.00|   0.22|   0.00279|   0.00|!
  0.01|   0.34|   0.00333|   0.00|!
  0.02|   0.47|   0.00387|   0.00|!
  0.03|   0.59|   0.00443|   0.00|!
  0.04|   0.70|   0.00504|   0.00|!
  0.05|   0.80|   0.00574|   0.00|!"
        );
    }
}
