use de::_from_nvlist;
use razorzfsnvpair as libnvpair;
use ser::_to_nvlist;

mod de;
mod ser;

pub type Result<T> = std::result::Result<T, libnvpair::NvListError>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn struct_u8() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: u8,
            b: u8,
            c: u8,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u16() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: u16,
            b: u16,
            c: u16,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u32() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: u32,
            b: u32,
            c: u32,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_u64() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: u64,
            b: u64,
            c: u64,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i8() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i8,
            b: i8,
            c: i8,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i16() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i16,
            b: i16,
            c: i16,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i32() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i32,
            b: i32,
            c: i32,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_i64() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i64,
            b: i64,
            c: i64,
        }

        let test_struct = Test { a: 3, b: 5, c: 7 };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_string() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }

        let test_struct = Test {
            a: "a".to_string(),
            b: "b".to_string(),
            c: "c".to_string(),
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_bool() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: bool,
            b: bool,
            c: bool,
        }

        let test_struct = Test {
            a: true,
            b: false,
            c: true,
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix_basic() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            e: i8,
            f: i16,
            g: i32,
            h: i64,
            i: String,
            j: bool,
        }

        let test_struct = Test {
            a: 3,
            b: 5,
            c: 7,
            d: 11,
            e: 13,
            f: 17,
            g: 19,
            h: 23,
            i: "test".to_string(),
            j: false,
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u8() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u8>,
            c: Vec<u8>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u16() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u16>,
            b: Vec<u16>,
            c: Vec<u16>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u32() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u32>,
            b: Vec<u32>,
            c: Vec<u32>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_u64() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u64>,
            b: Vec<u64>,
            c: Vec<u64>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i8() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<i8>,
            b: Vec<i8>,
            c: Vec<i8>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i16() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<i16>,
            b: Vec<i16>,
            c: Vec<i16>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i32() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<i32>,
            b: Vec<i32>,
            c: Vec<i32>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_i64() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<i64>,
            b: Vec<i64>,
            c: Vec<i64>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_string() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<String>,
            b: Vec<String>,
            c: Vec<String>,
        }

        let test_struct = Test {
            a: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            b: vec![
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
                "j".to_string(),
            ],
            c: vec![
                "k".to_string(),
                "l".to_string(),
                "m".to_string(),
                "n".to_string(),
                "o".to_string(),
            ],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_vec_bool() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<bool>,
            b: Vec<bool>,
            c: Vec<bool>,
        }

        let test_struct = Test {
            a: vec![true, true, true, true, true],
            b: vec![false, false, false, false, false],
            c: vec![true, true, false, true, true],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix_vec() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u16>,
            c: Vec<u32>,
            d: Vec<u64>,
            e: Vec<i8>,
            f: Vec<i16>,
            g: Vec<i32>,
            h: Vec<i64>,
            i: Vec<String>,
            j: Vec<bool>,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
            d: vec![16, 17, 18, 19, 20],
            e: vec![21, 22, 23, 24, 25],
            f: vec![26, 27, 28, 29, 30],
            g: vec![31, 32, 33, 34, 35],
            h: vec![36, 37, 38, 39, 40],
            i: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            j: vec![true, true, false, true, true],
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_mix() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: Vec<u8>,
            b: Vec<u16>,
            c: Vec<u32>,
            d: Vec<u64>,
            e: Vec<i8>,
            f: Vec<i16>,
            g: Vec<i32>,
            h: Vec<i64>,
            i: Vec<String>,
            j: Vec<bool>,
            k: u8,
            l: u16,
            m: u32,
            n: u64,
            o: i8,
            p: i16,
            q: i32,
            r: i64,
            s: String,
            t: bool,
        }

        let test_struct = Test {
            a: vec![1, 2, 3, 4, 5],
            b: vec![6, 7, 8, 9, 10],
            c: vec![11, 12, 13, 14, 15],
            d: vec![16, 17, 18, 19, 20],
            e: vec![21, 22, 23, 24, 25],
            f: vec![26, 27, 28, 29, 30],
            g: vec![31, 32, 33, 34, 35],
            h: vec![36, 37, 38, 39, 40],
            i: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            j: vec![true, true, false, true, true],
            k: 3,
            l: 5,
            m: 7,
            n: 11,
            o: 13,
            p: 17,
            q: 19,
            r: 23,
            s: "test".to_string(),
            t: false,
        };
        let mut nvlist = _to_nvlist(&test_struct).unwrap();

        assert_eq!(test_struct, _from_nvlist(&mut nvlist).unwrap());
    }

    #[test]
    fn struct_nested_depth_two() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Nested {
            a: u16,
            b: u16,
            c: u16,
            d: Vec<u16>,
        }
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            a: Nested,
            b: u16,
            c: u16,
            d: Vec<u16>,
        }

        let expected = Test {
            a: Nested {
                a: 3,
                b: 5,
                c: 7,
                d: vec![1, 2, 3],
            },
            b: 3,
            c: 5,
            d: vec![1, 2, 3],
        };

        let mut nvlist = _to_nvlist(&expected).unwrap();
        assert_eq!(expected, _from_nvlist(&mut nvlist).unwrap());
    }
}
