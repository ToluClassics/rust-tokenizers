mod test_utils;
use rust_tokenizers::tokenizer::{
    MultiThreadedTokenizer, Tokenizer, TruncationStrategy, XLMRobertaTokenizer,
};
use rust_tokenizers::{Offset, TokenizedInput};
use test_utils::download_file_to_cache;

#[test]
fn test_xlm_roberta_tokenization() -> anyhow::Result<()> {
    let vocab_path = download_file_to_cache("https://cdn.huggingface.co/xlm-roberta-large-finetuned-conll03-english-sentencepiece.bpe.model", )?;

    let xlm_roberta_tokenizer = XLMRobertaTokenizer::from_file(vocab_path, false)?;

    let original_strings = [
        "…",
        "This is a sample sentence to be tokénized",
        "Wondering how this will get tokenized 🤔 ?",
        "İs th!s 𩸽 Ϻ Šœ Ugljšić dấu nặng",
        "   İs th!s    𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
        " � İs th!s �� 𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
    ];

    let expected_results = [
        TokenizedInput {
            token_ids: vec![0, 153, 2],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![None, Some(Offset { begin: 0, end: 1 }), None],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                0, 3293, 83, 10, 121413, 149357, 47, 186, 25636, 2746, 29367, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 0, end: 4 }),
                Some(Offset { begin: 4, end: 7 }),
                Some(Offset { begin: 7, end: 9 }),
                Some(Offset { begin: 9, end: 16 }),
                Some(Offset { begin: 16, end: 25 }),
                Some(Offset { begin: 25, end: 28 }),
                Some(Offset { begin: 28, end: 31 }),
                Some(Offset { begin: 31, end: 35 }),
                Some(Offset { begin: 35, end: 38 }),
                Some(Offset { begin: 38, end: 42 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                0, 76648, 214, 3642, 903, 1221, 2046, 47, 1098, 29367, 6, 243691, 705, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 0, end: 6 }),
                Some(Offset { begin: 6, end: 9 }),
                Some(Offset { begin: 9, end: 13 }),
                Some(Offset { begin: 13, end: 18 }),
                Some(Offset { begin: 18, end: 23 }),
                Some(Offset { begin: 23, end: 27 }),
                Some(Offset { begin: 27, end: 30 }),
                Some(Offset { begin: 30, end: 33 }),
                Some(Offset { begin: 33, end: 37 }),
                Some(Offset { begin: 37, end: 38 }),
                Some(Offset { begin: 38, end: 39 }),
                Some(Offset { begin: 39, end: 41 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                0, 63770, 5675, 38, 7, 6, 3, 6, 3, 3608, 52908, 345, 11016, 170, 36277, 39973,
                55315, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 0, end: 2 }),
                Some(Offset { begin: 2, end: 5 }),
                Some(Offset { begin: 5, end: 6 }),
                Some(Offset { begin: 6, end: 7 }),
                Some(Offset { begin: 7, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 11, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 16 }),
                Some(Offset { begin: 16, end: 18 }),
                Some(Offset { begin: 18, end: 19 }),
                Some(Offset { begin: 19, end: 22 }),
                Some(Offset { begin: 22, end: 26 }),
                Some(Offset { begin: 26, end: 31 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                0, 6, 6, 63770, 5675, 38, 7, 6, 6, 6, 6, 3, 6, 3, 3608, 52908, 6, 6, 345, 11016,
                170, 36277, 6, 39973, 55315, 6, 6, 6, 6, 6, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 1, end: 2 }),
                Some(Offset { begin: 2, end: 5 }),
                Some(Offset { begin: 5, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 11, end: 12 }),
                Some(Offset { begin: 12, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 30 }),
                Some(Offset { begin: 30, end: 31 }),
                Some(Offset { begin: 31, end: 35 }),
                Some(Offset { begin: 35, end: 40 }),
                Some(Offset { begin: 40, end: 41 }),
                Some(Offset { begin: 41, end: 42 }),
                Some(Offset { begin: 42, end: 43 }),
                Some(Offset { begin: 43, end: 44 }),
                Some(Offset { begin: 44, end: 45 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                0, 6, 63770, 5675, 38, 7, 6, 6, 3, 6, 3, 3608, 52908, 6, 6, 345, 11016, 170, 36277,
                6, 39973, 55315, 6, 6, 6, 6, 6, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 0, end: 1 }),
                Some(Offset { begin: 2, end: 5 }),
                Some(Offset { begin: 5, end: 8 }),
                Some(Offset { begin: 8, end: 9 }),
                Some(Offset { begin: 9, end: 10 }),
                Some(Offset { begin: 10, end: 11 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 30 }),
                Some(Offset { begin: 30, end: 31 }),
                Some(Offset { begin: 31, end: 35 }),
                Some(Offset { begin: 35, end: 40 }),
                Some(Offset { begin: 40, end: 41 }),
                Some(Offset { begin: 41, end: 42 }),
                Some(Offset { begin: 42, end: 43 }),
                Some(Offset { begin: 43, end: 44 }),
                Some(Offset { begin: 44, end: 45 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
    ]
    .to_vec();

    let output = MultiThreadedTokenizer::encode_list(
        &xlm_roberta_tokenizer,
        &original_strings,
        128,
        &TruncationStrategy::LongestFirst,
        0,
    );

    for (_idx, (predicted, expected)) in output.iter().zip(expected_results.iter()).enumerate() {
        let original_sentence_chars: Vec<char> = original_strings[_idx].chars().collect();
        for (idx, offset) in predicted.token_offsets.iter().enumerate() {
            match offset {
                Some(offset) => {
                    let (start_char, end_char) = (offset.begin as usize, offset.end as usize);
                    let text: String = original_sentence_chars[start_char..end_char]
                        .iter()
                        .collect();
                    println!(
                        "{:<2?} | {:<10} | {:<10} | {:<10?}",
                        offset,
                        text,
                        xlm_roberta_tokenizer.decode(&[predicted.token_ids[idx]], false, false),
                        predicted.mask[idx]
                    )
                }
                None => continue,
            }
        }

        assert_eq!(predicted.token_ids, expected.token_ids);
        assert_eq!(predicted.token_offsets, expected.token_offsets);
    }
    Ok(())
}
