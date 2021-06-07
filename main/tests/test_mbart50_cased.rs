mod test_utils;

use rust_tokenizers::tokenizer::{
    MBart50Tokenizer, MultiThreadedTokenizer, Tokenizer, TruncationStrategy,
};
use rust_tokenizers::{Offset, TokenizedInput};
use test_utils::download_file_to_cache;

#[test]
fn test_mbart50_tokenization() -> anyhow::Result<()> {
    let vocab_path = download_file_to_cache(
        "https://huggingface.co/facebook/mbart-large-50-many-to-many-mmt/resolve/main/sentencepiece.bpe.model",
        "mbart50_spiece.model",
    )?;

    let mbart_tokenizer = MBart50Tokenizer::from_file(vocab_path.to_str().unwrap(), false)?;

    let original_strings = [
        "en_XX This is a sample sentence to be tokénized",
        "en_XX Wondering how this will get tokenized 🤔 ?",
        "fr_XX İs th!s 𩸽 Ϻ Šœ Ugljšić dấu nặng",
        "hi_IN   İs th!s    𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
        "lt_LT � İs th!s �� 𩸽 Ϻ Šœ   Ugljšić  dấu nặng     ",
    ];

    let expected_results = [
        TokenizedInput {
            token_ids: vec![
                250004, 3293, 83, 10, 121413, 149357, 47, 186, 25636, 2746, 29367, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 5, end: 10 }),
                Some(Offset { begin: 10, end: 13 }),
                Some(Offset { begin: 13, end: 15 }),
                Some(Offset { begin: 15, end: 22 }),
                Some(Offset { begin: 22, end: 31 }),
                Some(Offset { begin: 31, end: 34 }),
                Some(Offset { begin: 34, end: 37 }),
                Some(Offset { begin: 37, end: 41 }),
                Some(Offset { begin: 41, end: 44 }),
                Some(Offset { begin: 44, end: 48 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                250004, 76648, 214, 3642, 903, 1221, 2046, 47, 1098, 29367, 6, 243691, 705, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 5, end: 12 }),
                Some(Offset { begin: 12, end: 15 }),
                Some(Offset { begin: 15, end: 19 }),
                Some(Offset { begin: 19, end: 24 }),
                Some(Offset { begin: 24, end: 29 }),
                Some(Offset { begin: 29, end: 33 }),
                Some(Offset { begin: 33, end: 36 }),
                Some(Offset { begin: 36, end: 39 }),
                Some(Offset { begin: 39, end: 43 }),
                Some(Offset { begin: 43, end: 44 }),
                Some(Offset { begin: 44, end: 45 }),
                Some(Offset { begin: 45, end: 47 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                250008, 63770, 5675, 38, 7, 6, 3, 6, 3, 3608, 52908, 345, 11016, 170, 36277, 39973,
                55315, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 5, end: 8 }),
                Some(Offset { begin: 8, end: 11 }),
                Some(Offset { begin: 11, end: 12 }),
                Some(Offset { begin: 12, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 28 }),
                Some(Offset { begin: 28, end: 32 }),
                Some(Offset { begin: 32, end: 37 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                250010, 63770, 5675, 38, 7, 6, 6, 6, 6, 3, 6, 3, 3608, 52908, 6, 6, 345, 11016,
                170, 36277, 6, 39973, 55315, 6, 6, 6, 6, 6, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 7, end: 10 }),
                Some(Offset { begin: 10, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 16, end: 17 }),
                Some(Offset { begin: 17, end: 18 }),
                Some(Offset { begin: 18, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 29 }),
                Some(Offset { begin: 29, end: 31 }),
                Some(Offset { begin: 31, end: 32 }),
                Some(Offset { begin: 32, end: 35 }),
                Some(Offset { begin: 35, end: 36 }),
                Some(Offset { begin: 36, end: 40 }),
                Some(Offset { begin: 40, end: 45 }),
                Some(Offset { begin: 45, end: 46 }),
                Some(Offset { begin: 46, end: 47 }),
                Some(Offset { begin: 47, end: 48 }),
                Some(Offset { begin: 48, end: 49 }),
                Some(Offset { begin: 49, end: 50 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
        TokenizedInput {
            token_ids: vec![
                250015, 6, 63770, 5675, 38, 7, 6, 6, 3, 6, 3, 3608, 52908, 6, 6, 345, 11016, 170,
                36277, 6, 39973, 55315, 6, 6, 6, 6, 6, 2,
            ],
            segment_ids: vec![],
            special_tokens_mask: vec![],
            overflowing_tokens: vec![],
            num_truncated_tokens: 0,
            token_offsets: vec![
                None,
                Some(Offset { begin: 5, end: 6 }),
                Some(Offset { begin: 7, end: 10 }),
                Some(Offset { begin: 10, end: 13 }),
                Some(Offset { begin: 13, end: 14 }),
                Some(Offset { begin: 14, end: 15 }),
                Some(Offset { begin: 15, end: 16 }),
                Some(Offset { begin: 18, end: 19 }),
                Some(Offset { begin: 19, end: 20 }),
                Some(Offset { begin: 20, end: 21 }),
                Some(Offset { begin: 21, end: 22 }),
                Some(Offset { begin: 22, end: 24 }),
                Some(Offset { begin: 24, end: 25 }),
                Some(Offset { begin: 25, end: 26 }),
                Some(Offset { begin: 26, end: 27 }),
                Some(Offset { begin: 27, end: 29 }),
                Some(Offset { begin: 29, end: 31 }),
                Some(Offset { begin: 31, end: 32 }),
                Some(Offset { begin: 32, end: 35 }),
                Some(Offset { begin: 35, end: 36 }),
                Some(Offset { begin: 36, end: 40 }),
                Some(Offset { begin: 40, end: 45 }),
                Some(Offset { begin: 45, end: 46 }),
                Some(Offset { begin: 46, end: 47 }),
                Some(Offset { begin: 47, end: 48 }),
                Some(Offset { begin: 48, end: 49 }),
                Some(Offset { begin: 49, end: 50 }),
                None,
            ],
            reference_offsets: vec![],
            mask: vec![],
        },
    ]
    .to_vec();

    let output = MultiThreadedTokenizer::encode_list(
        &mbart_tokenizer,
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
                        mbart_tokenizer.decode(vec!(predicted.token_ids[idx]), false, false),
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
