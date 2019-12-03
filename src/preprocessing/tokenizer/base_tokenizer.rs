use crate::preprocessing::vocab::base_vocab::Vocab;
use crate::preprocessing::tokenizer::tokenization_utils::{split_on_special_tokens, tokenize_cjk_chars, whitespace_tokenize, strip_accents, split_on_punct, clean_text};
use std::sync::Arc;
use rayon::prelude::*;
use itertools::Itertools;

pub trait Tokenizer<T: Vocab>
    where Self: std::marker::Sync {
    fn vocab(&self) -> &T;
    fn tokenize(&self, text: &str) -> Vec<String>;
    fn tokenize_list(&self, text_list: Vec<&str>) -> Vec<Vec<String>>;
    fn convert_tokens_to_ids(&self, tokens: &Vec<String>) -> Vec<i64> {
        tokens.iter().map(|v| self.vocab().token_to_id(v)).collect()
    }
    fn encode(&self, text: &str) -> Vec<i64> {
        self.convert_tokens_to_ids(&self.tokenize(text))
    }

    fn encode_list(&self, text_list: Vec<&str>) -> Vec<Vec<i64>> {
        text_list
            .par_iter()
            .map(|text| self.tokenize(text))
            .map(|tokens| self.convert_tokens_to_ids(&tokens))
            .collect()
    }
}

pub struct BaseTokenizer<T: Vocab> {
    vocab: Arc<T>
}

impl<T: Vocab + Sync + Send> BaseTokenizer<T> {
    pub fn from_file(path: &str) -> BaseTokenizer<T> {
        let vocab = T::from_file(path);
        BaseTokenizer { vocab: Arc::new(vocab) }
    }

    pub fn from_existing_vocab(vocab: Arc<T>) -> BaseTokenizer<T> {
        BaseTokenizer { vocab: vocab.clone() }
    }
}

impl<T: Vocab + Sync + Send> Tokenizer<T> for BaseTokenizer<T> {
    fn vocab(&self) -> &T {
        &self.vocab
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        let tokenized_text: Vec<String> = {
            let temp_text = split_on_special_tokens(text, self.vocab.as_ref());
            let temp_text: Vec<String> = temp_text.
                iter().
                map(|v| clean_text(v, true)).
                map(|v| tokenize_cjk_chars(&v)).
                collect();
            temp_text
        };

        let mut tokenized_text: Vec<String> = tokenized_text
            .iter()
            .map(|v| whitespace_tokenize(&v))
            .flatten()
            .map(|s| s.to_string())
            .collect();

        for string in tokenized_text.iter_mut() {
            if !self.vocab.as_ref().special_values().contains_key(string) {
                *string = string.to_lowercase();
                *string = strip_accents(string.to_owned());
            }
        }

        let tokenized_text: Vec<String> = tokenized_text
            .iter()
            .map(|v| split_on_punct(v.to_owned(), self.vocab.as_ref()))
            .flatten()
            .map(|s| s.to_string())
            .collect();

        let tokenized_text: String = tokenized_text.iter().join(" ");
        let tokenized_text: Vec<String> = whitespace_tokenize(&tokenized_text)
            .iter()
            .map(|s| s.to_string())
            .collect();
        tokenized_text
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> Vec<Vec<String>> {
        text_list.
            par_iter().
            map(|text| self.tokenize(text)).
            collect()
    }
}

//==============================
// Unit tests
//==============================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BertVocab;
    use std::collections::HashMap;

    fn generate_test_vocab() -> BertVocab {
        let values: HashMap<String, i64> = [
            ("hello".to_owned(), 0),
            ("world".to_owned(), 1),
            ("[UNK]".to_owned(), 2),
            ("!".to_owned(), 3),
            ("[CLS]".to_owned(), 4),
            ("[SEP]".to_owned(), 5),
            ("[MASK]".to_owned(), 6),
            ("[中]".to_owned(), 7),
            ("华".to_owned(), 8),
            ("人]".to_owned(), 9),
            ("[PAD]".to_owned(), 10),
            ("una".to_owned(), 10),
            ("##ffa".to_owned(), 10),
            ("##ble".to_owned(), 10)
        ].iter().cloned().collect();

        let special_values: HashMap<String, i64> = [
            ("[UNK]".to_owned(), 2),
            ("[CLS]".to_owned(), 4),
            ("[SEP]".to_owned(), 5),
            ("[MASK]".to_owned(), 6),
            ("[PAD]".to_owned(), 10)
        ].iter().cloned().collect();

        BertVocab { values, unknown_value: "[UNK]", special_values }
    }

    #[test]
    fn test_base_tokenizer() {
//        Given
        let vocab = Arc::new(generate_test_vocab());
        let base_tokenizer: BaseTokenizer<BertVocab> = BaseTokenizer::from_existing_vocab(vocab);
        let test_tuples = [
            (
                "Sentence with [MASK] token.",
                vec!("sentence", "with", "[MASK]", "token", ".")
            ),
            (
                "[CLS]Sentence with [MASK] token.",
                vec!("[CLS]", "sentence", "with", "[MASK]", "token", ".")
            ),
            (
                "[CLS]",
                vec!("[CLS]")
            ),
            (
                "[CLS] [PAD]",
                vec!("[CLS]", "[PAD]")
            ),
            (
                "[CLS]       [PAD]",
                vec!("[CLS]", "[PAD]")
            ),
            (
                "asdf[CLS]",
                vec!("asdf", "[CLS]")
            ),
            (
                "",
                vec!()
            ),
            (
                "Allons, Flipote, allons; que d'eux je me délivre.",
                vec!("allons", ",", "flipote", ",", "allons", ";", "que", "d", "\'", "eux", "je", "me", "delivre", ".")
            ),
            (
                "[UNK]中华人民共和国 [PAD] asdf",
                vec!("[UNK]", "中", "华", "人", "民", "共", "和", "国", "[PAD]", "asdf")
            )
        ];
        let source_texts: Vec<&str> = test_tuples.iter().map(|v| v.0).collect();
        let expected_results: Vec<Vec<&str>> = test_tuples.iter().map(|v| v.1.clone()).collect();

//        When & Then
        for (source_text, expected_result) in test_tuples.iter() {
            assert_eq!(base_tokenizer.tokenize(*source_text), *expected_result);
        }

        assert_eq!(base_tokenizer.tokenize_list(source_texts), expected_results);
    }
}
