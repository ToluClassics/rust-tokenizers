# Copyright 2018 The HuggingFace Inc. team.
# Copyright 2019 Guillaume Becquin
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#     http://www.apache.org/licenses/LICENSE-2.0
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import tempfile
from pathlib import Path
import pytest
from rust_tokenizers.rust_tokenizers import PySentencePieceTokenizer, PyXLMRobertaTokenizer
from transformers import AlbertTokenizer, T5Tokenizer, XLMRobertaTokenizer, XLNetTokenizer
from transformers.data.processors.glue import Sst2Processor
from transformers.file_utils import get_from_cache
from transformers.tokenization_bert import BertTokenizer
from transformers.tokenization_distilbert import DistilBertTokenizer
from transformers.tokenization_ctrl import CTRLTokenizer
from transformers.tokenization_gpt2 import GPT2Tokenizer
from transformers.tokenization_roberta import RobertaTokenizer
from transformers.tokenization_openai import OpenAIGPTTokenizer
from rust_tokenizers import PyBertTokenizer, PyCtrlTokenizer, PyGpt2Tokenizer, PyRobertaTokenizer, \
    PyOpenAiGptTokenizer, PyAlbertTokenizer, PyT5Tokenizer, PyXLNetTokenizer
from zipfile import ZipFile
import requests
import sentencepiece


@pytest.mark.slow
class TestTokenizationSST2:
    def setup_class(self):
        self.processor = Sst2Processor()
        self.test_dir = Path(tempfile.mkdtemp())
        sst2_url = 'https://firebasestorage.googleapis.com/v0/b/mtl-sentence-representations.appspot.com/o/data%2FSST-2.zip?alt=media&token=aabc5f6b-e466-44a2-b9b4-cf6337f84ac8'
        contents = requests.get(sst2_url)
        (self.test_dir / 'SST-2.zip').open('wb').write(contents.content)
        with ZipFile(self.test_dir / 'SST-2.zip', 'r') as zipObj:
            zipObj.extractall(self.test_dir)
        self.examples = self.processor.get_train_examples(self.test_dir / 'SST-2')
        sentence_piece_url = 'https://s3.amazonaws.com/models.huggingface.co/bert/xlnet-base-cased-spiece.model'
        contents = requests.get(sentence_piece_url)
        (self.test_dir / 'spiece.model').open('wb').write(contents.content)

    def test_tokenization_bert(self):
        # Given
        self.base_tokenizer = BertTokenizer.from_pretrained('bert-base-uncased',
                                                            do_lower_case=True,
                                                            cache_dir=self.test_dir)
        self.rust_tokenizer = PyBertTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['bert-base-uncased']),
            do_lower_case=True,
            strip_accents=True)
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f' Python {baseline["input_ids"]}'
            assert (rust.segment_ids == baseline['token_type_ids'])
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_distilbert(self):
        # Given
        self.base_tokenizer = DistilBertTokenizer.from_pretrained('distilbert-base-cased',
                                                                  do_lower_case=False,
                                                                  cache_dir=self.test_dir)
        self.rust_tokenizer = PyBertTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['distilbert-base-cased']),
            do_lower_case=False,
            strip_accents=False)
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f'Python {baseline["input_ids"]}'

    def test_tokenization_ctrl(self):
        # Given
        self.base_tokenizer = CTRLTokenizer.from_pretrained('ctrl',
                                                            do_lower_case=True,
                                                            cache_dir=self.test_dir)
        self.rust_tokenizer = PyCtrlTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['ctrl']),
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['merges_file']['ctrl']),
            do_lower_case=True
        )
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f'Python {baseline["input_ids"]}'
            assert (rust.segment_ids == baseline['token_type_ids'])
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_gpt2(self):
        # Given
        self.base_tokenizer = GPT2Tokenizer.from_pretrained('gpt2',
                                                            do_lower_case=True,
                                                            cache_dir=self.test_dir)
        self.rust_tokenizer = PyGpt2Tokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['gpt2']),
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['merges_file']['gpt2']), do_lower_case=True
        )
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f'Python {baseline["input_ids"]}'
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_roberta(self):
        # Given
        self.base_tokenizer = RobertaTokenizer.from_pretrained('roberta-base',
                                                               do_lower_case=True,
                                                               cache_dir=self.test_dir)
        self.rust_tokenizer = PyRobertaTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['roberta-base']),
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['merges_file']['roberta-base']),
            do_lower_case=True,
            add_prefix_space=False
        )
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   truncation='longest_first',
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f'Python {baseline["input_ids"]}'
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_openai_gpt(self):
        # Given
        self.base_tokenizer = OpenAIGPTTokenizer.from_pretrained('openai-gpt',
                                                                 do_lower_case=True,
                                                                 cache_dir=self.test_dir)
        self.rust_tokenizer = PyOpenAiGptTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['openai-gpt']),
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['merges_file']['openai-gpt']),
            do_lower_case=True
        )
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        output_rust = self.rust_tokenizer.encode_list([example.text_a for example in self.examples],
                                                      max_len=128,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            assert rust.token_ids == baseline[
                'input_ids'], f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                              f'Sentence a: {self.examples[idx].text_a} \n' \
                              f'Sentence b: {self.examples[idx].text_b} \n' \
                              f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n' \
                              f'Rust: {rust.token_ids} \n' \
                              f'Python {baseline["input_ids"]}'
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_sentence_piece(self):
        # Given
        self.base_tokenizer = sentencepiece.SentencePieceProcessor()
        self.base_tokenizer.Load(str(self.test_dir / 'spiece.model'))
        self.rust_tokenizer = PySentencePieceTokenizer(str(self.test_dir / 'spiece.model'), do_lower_case=False)
        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.EncodeAsIds(example.text_a))

        # When
        # Note: the original sentence piece tokenizer strips trailing spaces
        output_rust = self.rust_tokenizer.encode_list([example.text_a.strip() for example in self.examples],
                                                      max_len=256,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            if rust.token_ids != baseline:
                assert sum(self.base_tokenizer.get_score(baseline)) == \
                       sum(self.base_tokenizer.get_score(rust.token_ids)), \
                    f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n ' \
                    f'Sentence a: {self.examples[idx].text_a} \n' \
                    f'Sentence b: {self.examples[idx].text_b} \n' \
                    f'Token mismatch: {self.get_token_diff_sentence_piece(rust.token_ids, baseline)} \n' \
                    f'Rust: {rust.token_ids} \n' \
                    f'Python {baseline}'

    def test_tokenization_albert(self):
        # Given
        self.base_tokenizer = AlbertTokenizer.from_pretrained('albert-base-v2',
                                                              do_lower_case=True,
                                                              cache_dir=self.test_dir)
        self.rust_tokenizer = PyAlbertTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['albert-base-v2']),
            do_lower_case=True,
            strip_accents=True)

        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        # Note: the original sentence piece tokenizer strips trailing spaces
        output_rust = self.rust_tokenizer.encode_list([example.text_a.strip() for example in self.examples],
                                                      max_len=256,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            if rust.token_ids != baseline['input_ids']:
                for pos, (rust_id, baseline_id) in enumerate(zip(rust.token_ids, baseline['input_ids'])):
                    # This check is required a SentencePiece can also be ambiguous in very rare cases
                    # (e.g. "eee" -> "e, ee" or "ee, e" have the same score)
                    if rust_id != baseline_id:
                        if pos < len(baseline):
                            if (rust_id != baseline['input_ids'][pos + 1]) & \
                                    (rust_id != baseline['input_ids'][pos - 1]):
                                raise AssertionError(
                                    f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n '
                                    f'Sentence a: {self.examples[idx].text_a} \n'
                                    f'Sentence b: {self.examples[idx].text_b} \n'
                                    f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n'
                                    f'Rust: {rust.token_ids} \n'
                                    f'Python {baseline["input_ids"]}')
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_xlnet(self):
        # Given
        self.base_tokenizer = XLNetTokenizer.from_pretrained('xlnet-base-cased',
                                                             do_lower_case=False,
                                                             cache_dir=self.test_dir)
        self.rust_tokenizer = PyXLNetTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['xlnet-base-cased']),
            do_lower_case=False,
            strip_accents=True)

        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        # Note: the original sentence piece tokenizer strips trailing spaces
        output_rust = self.rust_tokenizer.encode_list([example.text_a.strip() for example in self.examples],
                                                      max_len=256,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            if rust.token_ids != baseline['input_ids']:
                for pos, (rust_id, baseline_id) in enumerate(zip(rust.token_ids, baseline['input_ids'])):
                    # This check is required a SentencePiece can also be ambiguous in very rare cases
                    # (e.g. "eee" -> "e, ee" or "ee, e" have the same score)
                    if rust_id != baseline_id:
                        if pos < len(baseline):
                            if (rust_id != baseline['input_ids'][pos + 1]) & \
                                    (rust_id != baseline['input_ids'][pos - 1]):
                                raise AssertionError(
                                    f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n '
                                    f'Sentence a: {self.examples[idx].text_a} \n'
                                    f'Sentence b: {self.examples[idx].text_b} \n'
                                    f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n'
                                    f'Rust: {rust.token_ids} \n'
                                    f'Python {baseline["input_ids"]}')
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_t5(self):
        # Given
        self.base_tokenizer = T5Tokenizer.from_pretrained('t5-base',
                                                          do_lower_case=False,
                                                          cache_dir=self.test_dir)
        self.rust_tokenizer = PyT5Tokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file']['t5-base']),
            do_lower_case=False)

        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        # Note: the original sentence piece tokenizer strips trailing spaces
        output_rust = self.rust_tokenizer.encode_list([example.text_a.strip() for example in self.examples],
                                                      max_len=256,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            if rust.token_ids != baseline['input_ids']:
                for pos, (rust_id, baseline_id) in enumerate(zip(rust.token_ids, baseline['input_ids'])):
                    # This check is required a SentencePiece can also be ambiguous in very rare cases
                    # (e.g. "eee" -> "e, ee" or "ee, e" have the same score)
                    if rust_id != baseline_id:
                        if pos < len(baseline):
                            if (rust_id != baseline['input_ids'][pos + 1]) & \
                                    (rust_id != baseline['input_ids'][pos - 1]):
                                raise AssertionError(
                                    f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n '
                                    f'Sentence a: {self.examples[idx].text_a} \n'
                                    f'Sentence b: {self.examples[idx].text_b} \n'
                                    f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n'
                                    f'Rust: {rust.token_ids} \n'
                                    f'Python {baseline["input_ids"]}')
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def test_tokenization_xlm_roberta(self):
        # Given
        self.base_tokenizer = XLMRobertaTokenizer.from_pretrained('xlm-roberta-large-finetuned-conll03-english',
                                                                  do_lower_case=False,
                                                                  cache_dir=self.test_dir)
        self.rust_tokenizer = PyXLMRobertaTokenizer(
            get_from_cache(self.base_tokenizer.pretrained_vocab_files_map['vocab_file'][
                               'xlm-roberta-large-finetuned-conll03-english']),
            do_lower_case=False)

        output_baseline = []
        for example in self.examples:
            output_baseline.append(self.base_tokenizer.encode_plus(example.text_a,
                                                                   add_special_tokens=True,
                                                                   return_overflowing_tokens=True,
                                                                   return_special_tokens_mask=True,
                                                                   max_length=128))

        # When
        # Note: the original sentence piece tokenizer strips trailing spaces
        output_rust = self.rust_tokenizer.encode_list([example.text_a.strip() for example in self.examples],
                                                      max_len=256,
                                                      truncation_strategy='longest_first',
                                                      stride=0)

        # Then
        for idx, (rust, baseline) in enumerate(zip(output_rust, output_baseline)):
            if rust.token_ids != baseline['input_ids']:
                for pos, (rust_id, baseline_id) in enumerate(zip(rust.token_ids, baseline['input_ids'])):
                    # This check is required a SentencePiece can also be ambiguous in very rare cases
                    # (e.g. "eee" -> "e, ee" or "ee, e" have the same score)
                    if rust_id != baseline_id:
                        if pos < len(baseline):
                            if (rust_id != baseline['input_ids'][pos + 1]) & \
                                    (rust_id != baseline['input_ids'][pos - 1]):
                                raise AssertionError(
                                    f'Difference in tokenization for {self.rust_tokenizer.__class__}: \n '
                                    f'Sentence a: {self.examples[idx].text_a} \n'
                                    f'Sentence b: {self.examples[idx].text_b} \n'
                                    f'Token mismatch: {self.get_token_diff(rust.token_ids, baseline["input_ids"])} \n'
                                    f'Rust: {rust.token_ids} \n'
                                    f'Python {baseline["input_ids"]}')
            assert (rust.special_tokens_mask == baseline['special_tokens_mask'])

    def get_token_diff(self, rust_tokens, python_tokens):
        last_index = 1
        first_index = 0
        max_index = min(len(rust_tokens), len(python_tokens))
        while rust_tokens[first_index] == python_tokens[first_index] and first_index < max_index - 1:
            first_index += 1
        first_index -= 1
        while rust_tokens[-last_index] == python_tokens[-last_index] and last_index < max_index - 1:
            last_index += 1
        last_index += 1
        python_last_index = len(python_tokens) + last_index
        rust_last_index = len(rust_tokens) + last_index
        rust_tokens_diff = rust_tokens[first_index:rust_last_index]
        python_token_diff = python_tokens[first_index:python_last_index]
        rust_decoded_tokens = self.base_tokenizer.convert_ids_to_tokens(rust_tokens_diff)
        python_decoded_tokens = self.base_tokenizer.convert_ids_to_tokens(python_token_diff)
        return rust_decoded_tokens, python_decoded_tokens

    def get_token_diff_sentence_piece(self, rust_tokens, python_tokens):
        last_index = 1
        first_index = 0
        max_index = min(len(rust_tokens), len(python_tokens))
        while rust_tokens[first_index] == python_tokens[first_index] and first_index < max_index - 1:
            first_index += 1
        first_index -= 1
        while rust_tokens[-last_index] == python_tokens[-last_index] and last_index < max_index - 1:
            last_index += 1
        last_index += 1
        python_last_index = len(python_tokens) + last_index
        rust_last_index = len(rust_tokens) + last_index
        rust_tokens_diff = rust_tokens[first_index:rust_last_index]
        python_token_diff = python_tokens[first_index:python_last_index]
        rust_decoded_tokens = self.base_tokenizer.DecodeIds(rust_tokens_diff)
        python_decoded_tokens = self.base_tokenizer.DecodeIds(python_token_diff)
        return rust_decoded_tokens, python_decoded_tokens
