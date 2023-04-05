#![allow(dead_code)]
use crate::Media;
use std::collections::BTreeMap;

use {
    bitcoin::{
        blockdata::{
            opcodes,
            script::{self, Instruction, Instructions},
        },
        taproot::TAPROOT_ANNEX_PREFIX,
        ScriptBuf, Transaction, Witness,
        script::PushBytes,
    },
    std::{iter::Peekable, str},
};

const PROTOCOL_ID: &[u8] = b"ord";

const BODY_TAG: &[u8] = &[];
const CONTENT_TYPE_TAG: &[u8] = &[1];

#[derive(Debug, PartialEq, Clone)]
pub struct Inscription {
    body: Option<Vec<u8>>,
    content_type: Option<Vec<u8>>,
}

impl Inscription {
    #[cfg(test)]
    pub(crate) fn new(content_type: Option<Vec<u8>>, body: Option<Vec<u8>>) -> Self {
        Self { content_type, body }
    }

    pub fn from_transaction(tx: &Transaction) -> Option<Inscription> {
        InscriptionParser::parse(&tx.input.get(0)?.witness).ok()
    }

    // pub(crate) fn from_file(_network: Network, path: impl AsRef<Path>) -> Result<Self, Error> {
    //     let path = path.as_ref();

    //     let body =
    //         fs::read(path).with_context(|| format!("io error reading {}", path.display()))?;

    //     // if let Some(limit) = chain.inscription_content_size_limit() {
    //     //     let len = body.len();
    //     //     if len > limit {
    //     //         bail!("content size of {len} bytes exceeds {limit} byte limit for {chain} inscriptions");
    //     //     }
    //     // }

    //     let content_type = Media::content_type_for_path(path)?;

    //     Ok(Self {
    //         body: Some(body),
    //         content_type: Some(content_type.into()),
    //     })
    // }

    fn append_reveal_script_to_builder(&self, mut builder: script::Builder) -> script::Builder {
        let protocol_push_bytes: &PushBytes = <&PushBytes>::try_from(PROTOCOL_ID).unwrap();

        builder = builder
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(protocol_push_bytes);

        let content_tag_push_bytes: &PushBytes = <&PushBytes>::try_from(CONTENT_TYPE_TAG).unwrap();

        if let Some(content_type) = &self.content_type {
            builder = builder
                .push_slice(content_tag_push_bytes);

            content_type.iter().for_each(|t| {
                //let content_type_push_bytes = PushBytesBuf::try_from(t).unwrap();
                let xt = &[*t];
                let content_type_push_bytes: &PushBytes = <&PushBytes>::try_from(xt).unwrap();
                builder = builder.clone().push_slice(content_type_push_bytes);
            });
        }

        if let Some(body) = &self.body {
            let body_tag_push_bytes: &PushBytes = <&PushBytes>::try_from(BODY_TAG).unwrap();

            builder = builder.push_slice(body_tag_push_bytes);
            body.iter().for_each(|chunk| {
                let xchunk = &[*chunk];
                let chunk_push_bytes: &PushBytes = <&PushBytes>::try_from(xchunk).unwrap();
                builder = builder.clone().push_slice(chunk_push_bytes);
            });
        }

        builder.push_opcode(opcodes::all::OP_ENDIF)
    }
    #[allow(dead_code)]
    pub(crate) fn append_reveal_script(&self, builder: script::Builder) -> ScriptBuf {
        self.append_reveal_script_to_builder(builder).into_script()
    }

    pub fn media(&self) -> Media {
        if self.body.is_none() {
            return Media::Unknown;
        }

        let Some(content_type) = self.content_type() else {
      return Media::Unknown;
    };

        content_type.parse().unwrap_or(Media::Unknown)
    }

    pub fn body(&self) -> Option<&[u8]> {
        Some(self.body.as_ref()?)
    }

    pub fn into_body(self) -> Option<Vec<u8>> {
        self.body
    }

    pub fn content_length(&self) -> Option<usize> {
        Some(self.body()?.len())
    }

    pub fn content_type(&self) -> Option<&str> {
        str::from_utf8(self.content_type.as_ref()?).ok()
    }

    #[cfg(test)]
    pub(crate) fn to_witness(&self) -> Witness {
        let builder = script::Builder::new();

        let script = self.append_reveal_script(builder);

        let mut witness = Witness::new();

        witness.push(script);
        witness.push([]);

        witness
    }
}

#[derive(Debug, PartialEq)]
enum InscriptionError {
    EmptyWitness,
    InvalidInscription,
    KeyPathSpend,
    NoInscription,
    ScriptBuf(script::Error),
    UnrecognizedEvenField,
}

type Result<T, E = InscriptionError> = std::result::Result<T, E>;

struct InscriptionParser<'a> {
    instructions: Peekable<Instructions<'a>>,
}

impl<'a> InscriptionParser<'a> {
    fn parse(witness: &Witness) -> Result<Inscription> {
        if witness.is_empty() {
            return Err(InscriptionError::EmptyWitness);
        }

        if witness.len() == 1 {
            return Err(InscriptionError::KeyPathSpend);
        }

        let annex = witness
            .last()
            .and_then(|element| element.first().map(|byte| *byte == TAPROOT_ANNEX_PREFIX))
            .unwrap_or(false);

        if witness.len() == 2 && annex {
            return Err(InscriptionError::KeyPathSpend);
        }

        let script = witness
            .iter()
            .nth(if annex {
                witness.len() - 1
            } else {
                witness.len() - 2
            })
            .unwrap();

        InscriptionParser {
            instructions: ScriptBuf::from(Vec::from(script)).instructions().peekable(),
        }
        .parse_script()
    }

    fn parse_script(mut self) -> Result<Inscription> {
        loop {
            let next = self.advance()?;
            let no_push_bytes: &PushBytes = <&PushBytes>::try_from(&[]).unwrap();

            if next == Instruction::PushBytes(no_push_bytes) {
                if let Some(inscription) = self.parse_inscription()? {
                    return Ok(inscription);
                }
            }
        }
    }

    fn advance(&mut self) -> Result<Instruction<'a>> {
        self.instructions
            .next()
            .ok_or(InscriptionError::NoInscription)?
            .map_err(InscriptionError::ScriptBuf)
    }

    fn parse_inscription(&mut self) -> Result<Option<Inscription>> {
        if self.advance()? == Instruction::Op(opcodes::all::OP_IF) {
            let protocol_push_bytes: &PushBytes = <&PushBytes>::try_from(PROTOCOL_ID).unwrap();
            if !self.accept(Instruction::PushBytes(protocol_push_bytes))? {
                return Err(InscriptionError::NoInscription);
            }

            let mut fields = BTreeMap::new();
            let body_tag_push_bytes: &PushBytes = <&PushBytes>::try_from(BODY_TAG).unwrap();

                loop {
                    match self.advance()? {
                    Instruction::PushBytes(tag) => {
                        match tag {
                            _ if tag == body_tag_push_bytes => {
                                let mut body = Vec::new();
                                while !self.accept(Instruction::Op(opcodes::all::OP_ENDIF))? {
                                    body.extend_from_slice(self.expect_push()?);
                                } 
                                fields.insert(body_tag_push_bytes, body);
                                break;
                            }
                            _ => {
                                if fields.contains_key(tag) {
                                    return Err(InscriptionError::InvalidInscription);
                                }
                                fields.insert(tag, self.expect_push()?.to_vec());
                            }
                        }
                    }
                    Instruction::Op(opcodes::all::OP_ENDIF) => break,
                    _ => return Err(InscriptionError::InvalidInscription),
                }
            }

            let body = fields.remove(body_tag_push_bytes);
            let content_tag_push_bytes: &PushBytes = <&PushBytes>::try_from(CONTENT_TYPE_TAG).unwrap();
            let content_type = fields.remove(content_tag_push_bytes);

            for tag in fields.keys() {
                if let Some(lsb) = tag.as_bytes().first() {
                    if lsb % 2 == 0 {
                        return Err(InscriptionError::UnrecognizedEvenField);
                    }
                }
            }

            return Ok(Some(Inscription { body, content_type }));
        }

        Ok(None)
    }

    fn expect_push(&mut self) -> Result<&'a [u8]> {
        match self.advance()? {
            Instruction::PushBytes(bytes) => Ok(bytes.as_bytes()),
            _ => Err(InscriptionError::InvalidInscription),
        }
    }

    fn accept(&mut self, instruction: Instruction) -> Result<bool> {
        match self.instructions.peek() {
            Some(Ok(next)) => {
                if *next == instruction {
                    self.advance()?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Some(Err(err)) => Err(InscriptionError::ScriptBuf(*err)),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::{OutPoint, Sequence, TxIn};

    fn envelope(payload: &[&[u8]]) -> Witness {
        let mut builder = script::Builder::new()
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF);

        for data in payload {
            builder = builder.push_slice(data);
        }

        let script = builder.push_opcode(opcodes::all::OP_ENDIF).into_script();

        Witness::from_vec(vec![script.into_bytes(), Vec::new()])
    }

    #[test]
    fn empty() {
        assert_eq!(
            InscriptionParser::parse(&Witness::new()),
            Err(InscriptionError::EmptyWitness)
        );
    }

    #[test]
    fn ignore_key_path_spends() {
        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![Vec::new()])),
            Err(InscriptionError::KeyPathSpend),
        );
    }

    #[test]
    fn ignore_key_path_spends_with_annex() {
        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![Vec::new(), vec![0x50]])),
            Err(InscriptionError::KeyPathSpend),
        );
    }

    #[test]
    fn ignore_unparsable_scripts() {
        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![vec![0x01], Vec::new()])),
            Err(InscriptionError::ScriptBuf(script::Error::EarlyEndOfScript)),
        );
    }

    #[test]
    fn no_inscription() {
        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![
                ScriptBuf::new().into_bytes(),
                Vec::new()
            ])),
            Err(InscriptionError::NoInscription),
        );
    }

    #[test]
    fn duplicate_field() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[1],
                b"text/plain;charset=utf-8",
                &[],
                b"ord",
            ])),
            Err(InscriptionError::InvalidInscription),
        );
    }

    #[test]
    fn valid() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[],
                b"ord",
            ])),
            Ok(inscription("text/plain;charset=utf-8", "ord")),
        );
    }

    #[test]
    fn valid_with_unknown_tag() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[3],
                b"bar",
                &[],
                b"ord",
            ])),
            Ok(inscription("text/plain;charset=utf-8", "ord")),
        );
    }

    #[test]
    fn no_content_tag() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[1], b"text/plain;charset=utf-8"])),
            Ok(Inscription {
                content_type: Some(b"text/plain;charset=utf-8".to_vec()),
                body: None,
            }),
        );
    }

    #[test]
    fn no_content_type() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[], b"foo"])),
            Ok(Inscription {
                content_type: None,
                body: Some(b"foo".to_vec()),
            }),
        );
    }

    #[test]
    fn valid_body_in_multiple_pushes() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[],
                b"foo",
                b"bar"
            ])),
            Ok(inscription("text/plain;charset=utf-8", "foobar")),
        );
    }

    #[test]
    fn valid_body_in_zero_pushes() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[1], b"text/plain;charset=utf-8", &[]])),
            Ok(inscription("text/plain;charset=utf-8", "")),
        );
    }

    #[test]
    fn valid_body_in_multiple_empty_pushes() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[],
                &[],
                &[],
                &[],
                &[],
                &[],
            ])),
            Ok(inscription("text/plain;charset=utf-8", "")),
        );
    }

    #[test]
    fn valid_ignore_trailing() {
        let script = script::Builder::new()
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(b"ord")
            .push_slice(&[1])
            .push_slice(b"text/plain;charset=utf-8")
            .push_slice(&[])
            .push_slice(b"ord")
            .push_opcode(opcodes::all::OP_ENDIF)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();

        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![script.into_bytes(), Vec::new()])),
            Ok(inscription("text/plain;charset=utf-8", "ord")),
        );
    }

    #[test]
    fn valid_ignore_preceding() {
        let script = script::Builder::new()
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(b"ord")
            .push_slice(&[1])
            .push_slice(b"text/plain;charset=utf-8")
            .push_slice(&[])
            .push_slice(b"ord")
            .push_opcode(opcodes::all::OP_ENDIF)
            .into_script();

        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![script.into_bytes(), Vec::new()])),
            Ok(inscription("text/plain;charset=utf-8", "ord")),
        );
    }

    #[test]
    fn valid_ignore_inscriptions_after_first() {
        let script = script::Builder::new()
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(b"ord")
            .push_slice(&[1])
            .push_slice(b"text/plain;charset=utf-8")
            .push_slice(&[])
            .push_slice(b"foo")
            .push_opcode(opcodes::all::OP_ENDIF)
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice(b"ord")
            .push_slice(&[1])
            .push_slice(b"text/plain;charset=utf-8")
            .push_slice(&[])
            .push_slice(b"bar")
            .push_opcode(opcodes::all::OP_ENDIF)
            .into_script();

        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![script.into_bytes(), Vec::new()])),
            Ok(inscription("text/plain;charset=utf-8", "foo")),
        );
    }

    #[test]
    fn invalid_utf8_does_not_render_inscription_invalid() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[
                b"ord",
                &[1],
                b"text/plain;charset=utf-8",
                &[],
                &[0b10000000]
            ])),
            Ok(inscription("text/plain;charset=utf-8", [0b10000000])),
        );
    }

    #[test]
    fn no_endif() {
        let script = script::Builder::new()
            .push_opcode(opcodes::OP_FALSE)
            .push_opcode(opcodes::all::OP_IF)
            .push_slice("ord".as_bytes())
            .into_script();

        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![script.into_bytes(), Vec::new()])),
            Err(InscriptionError::NoInscription)
        );
    }

    #[test]
    fn no_op_false() {
        let script = script::Builder::new()
            .push_opcode(opcodes::all::OP_IF)
            .push_slice("ord".as_bytes())
            .push_opcode(opcodes::all::OP_ENDIF)
            .into_script();

        assert_eq!(
            InscriptionParser::parse(&Witness::from_vec(vec![script.into_bytes(), Vec::new()])),
            Err(InscriptionError::NoInscription)
        );
    }

    #[test]
    fn empty_envelope() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[])),
            Err(InscriptionError::NoInscription)
        );
    }

    #[test]
    fn wrong_magic_number() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"foo"])),
            Err(InscriptionError::NoInscription),
        );
    }

    #[test]
    fn extract_from_transaction() {
        let tx = Transaction {
            version: 0,
            lock_time: bitcoin::LockTime(0),
            input: vec![TxIn {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::new(),
                sequence: Sequence(0),
                witness: envelope(&[b"ord", &[1], b"text/plain;charset=utf-8", &[], b"ord"]),
            }],
            output: Vec::new(),
        };

        assert_eq!(
            Inscription::from_transaction(&tx),
            Some(inscription("text/plain;charset=utf-8", "ord")),
        );
    }

    #[test]
    fn do_not_extract_from_second_input() {
        let tx = Transaction {
            version: 0,
            lock_time: bitcoin::LockTime(0),
            input: vec![
                TxIn {
                    previous_output: OutPoint::null(),
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence(0),
                    witness: Witness::new(),
                },
                TxIn {
                    previous_output: OutPoint::null(),
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence(0),
                    witness: inscription("foo", [1; 1040]).to_witness(),
                },
            ],
            output: Vec::new(),
        };

        assert_eq!(Inscription::from_transaction(&tx), None);
    }

    #[test]
    fn do_not_extract_from_second_envelope() {
        let mut builder = script::Builder::new();
        builder = inscription("foo", [1; 100]).append_reveal_script_to_builder(builder);
        builder = inscription("bar", [1; 100]).append_reveal_script_to_builder(builder);

        let witness = Witness::from_vec(vec![builder.into_script().into_bytes(), Vec::new()]);

        let tx = Transaction {
            version: 0,
            lock_time: bitcoin::LockTime(0),
            input: vec![TxIn {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::new(),
                sequence: Sequence(0),
                witness,
            }],
            output: Vec::new(),
        };

        assert_eq!(
            Inscription::from_transaction(&tx),
            Some(inscription("foo", [1; 100]))
        );
    }

    #[test]
    fn inscribe_png() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[1], b"image/png", &[], &[1; 100]])),
            Ok(inscription("image/png", [1; 100])),
        );
    }

    #[test]
    fn reveal_script_chunks_data() {
        assert_eq!(
            inscription("foo", [])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            7
        );

        assert_eq!(
            inscription("foo", [0; 1])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            8
        );

        assert_eq!(
            inscription("foo", [0; 520])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            8
        );

        assert_eq!(
            inscription("foo", [0; 521])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            9
        );

        assert_eq!(
            inscription("foo", [0; 1040])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            9
        );

        assert_eq!(
            inscription("foo", [0; 1041])
                .append_reveal_script(script::Builder::new())
                .instructions()
                .count(),
            10
        );
    }

    #[test]
    fn chunked_data_is_parsable() {
        let mut witness = Witness::new();

        witness.push(&inscription("foo", [1; 1040]).append_reveal_script(script::Builder::new()));

        witness.push([]);

        assert_eq!(
            InscriptionParser::parse(&witness).unwrap(),
            inscription("foo", [1; 1040]),
        );
    }

    #[test]
    fn round_trip_with_no_fields() {
        let mut witness = Witness::new();

        witness.push(
            &Inscription {
                content_type: None,
                body: None,
            }
            .append_reveal_script(script::Builder::new()),
        );

        witness.push([]);

        assert_eq!(
            InscriptionParser::parse(&witness).unwrap(),
            Inscription {
                content_type: None,
                body: None,
            }
        );
    }

    #[test]
    fn unknown_odd_fields_are_ignored() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[3], &[0]])),
            Ok(Inscription {
                content_type: None,
                body: None,
            }),
        );
    }

    #[test]
    fn unknown_even_fields_are_invalid() {
        assert_eq!(
            InscriptionParser::parse(&envelope(&[b"ord", &[2], &[0]])),
            Err(InscriptionError::UnrecognizedEvenField),
        );
    }

    pub(crate) fn inscription(content_type: &str, body: impl AsRef<[u8]>) -> Inscription {
        Inscription::new(Some(content_type.into()), Some(body.as_ref().into()))
    }
}
