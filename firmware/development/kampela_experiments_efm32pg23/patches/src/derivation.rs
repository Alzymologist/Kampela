//! This should have been a regex line, but for `no_std`.

use alloc::vec::Vec;
use sp_core::crypto::DeriveJunction;

enum ItemDraft {
    None,
    Soft { start: usize, len: usize },
    Hard { start: usize, len: usize },
}

pub fn cut_derivations(input: &str) -> Result<Vec<DeriveJunction>, ErrorDerivation> {
    let mut out: Vec<DeriveJunction> = Vec::new();

    let mut item_draft = ItemDraft::None;

    for (i, ch) in input.chars().enumerate() {
        if ch == '/' {
            item_draft = match item_draft {
                ItemDraft::None => ItemDraft::Soft {
                    start: i + 1,
                    len: 0,
                },
                ItemDraft::Soft { start, len } => {
                    if len == 0 {
                        ItemDraft::Hard {
                            start: i + 1,
                            len: 0,
                        }
                    } else {
                        out.push(DeriveJunction::soft(&input[start..start + len]));
                        ItemDraft::Soft {
                            start: i + 1,
                            len: 0,
                        }
                    }
                }
                ItemDraft::Hard { start, len } => {
                    if len == 0 {
                        return Err(ErrorDerivation::Pwd);
                    } else {
                        out.push(DeriveJunction::hard(&input[start..start + len]));
                        ItemDraft::Soft {
                            start: i + 1,
                            len: 0,
                        }
                    }
                }
            };
        } else {
            item_draft = match item_draft {
                ItemDraft::None => return Err(ErrorDerivation::NoDerivation),
                ItemDraft::Soft { start, len } => ItemDraft::Soft {
                    start,
                    len: len + 1,
                },
                ItemDraft::Hard { start, len } => ItemDraft::Hard {
                    start,
                    len: len + 1,
                },
            };
        }
    }

    match item_draft {
        ItemDraft::None => {}
        ItemDraft::Soft { start, len } => {
            if len == 0 {
                return Err(ErrorDerivation::EmptySoft);
            } else {
                out.push(DeriveJunction::soft(&input[start..start + len]))
            }
        }
        ItemDraft::Hard { start, len } => {
            if len == 0 {
                return Err(ErrorDerivation::EmptyHard);
            } else {
                out.push(DeriveJunction::hard(&input[start..start + len]))
            }
        }
    }

    Ok(out)
}

#[derive(Debug)]
pub enum ErrorDerivation {
    EmptyHard,
    EmptySoft,
    NoDerivation,
    Pwd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_errors() {
        assert!(cut_derivations("//").is_err());
        assert!(cut_derivations("/").is_err());
        assert!(cut_derivations("///").is_err());
        assert!(cut_derivations("abc").is_err());
        assert!(cut_derivations("abc/").is_err());
        assert!(cut_derivations("/abc/").is_err());
    }

    #[test]
    fn test_cut_derivations() {
        let set = cut_derivations("//hard/soft//0/1").unwrap();
        assert_eq!(
            set,
            [
                DeriveJunction::hard("hard"),
                DeriveJunction::soft("soft"),
                DeriveJunction::hard("0"),
                DeriveJunction::soft("1")
            ]
        );
    }
}
