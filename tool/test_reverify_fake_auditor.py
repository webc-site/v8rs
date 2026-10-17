#!/usr/bin/env python3
"""
tool/reverify_fake_auditor.py 规则检测单元测试
"""

import re
import unittest

class TestReverifyFakeAuditorPatterns(unittest.TestCase):
    def setUp(self):
        self.fake_ptr_regex = re.compile(r"\b(0x1234|0xdeadbeef|0xbeef|0xcafe_?[0-9a-fA-F]*)\b")
        self.stub_discard_regex = re.compile(
            r"pub\s+fn\s+[a-zA-Z0-9_]+\s*\([^)]*_[a-zA-Z0-9_]+:[^)]*\)\s*->[^{]+\{\s*(Ok\(\(\)\)|true|false|0|None|Default::default\(\))\s*\}"
        )
        self.fake_test_regex = re.compile(
            r"#\[test\]\s*(?:async\s+)?fn\s+[a-zA-Z0-9_]+\s*\(\)\s*\{\s*(?:assert!\(true\);?)?\s*\}"
        )

    def test_discard_param_stub_matches_fake_stubs(self):
        stub = "pub fn dummy_exec(_arg: i32) -> bool { true }"
        self.assertIsNotNone(self.stub_discard_regex.search(stub))

        stub_none = "pub fn get_slot(_ctx: usize) -> Option<Slot> { None }"
        self.assertIsNotNone(self.stub_discard_regex.search(stub_none))

    def test_discard_param_stub_does_not_match_real_fn(self):
        real_fn = "pub fn calculate(x: i32) -> i32 { x + 42 }"
        self.assertIsNone(self.stub_discard_regex.search(real_fn))

    def test_fake_pointer_literal(self):
        code_fake = "let ptr = 0xdeadbeef as *const u8;"
        self.assertIsNotNone(self.fake_ptr_regex.search(code_fake))

        code_real = "let normal_hex = 0x1000;"
        self.assertIsNone(self.fake_ptr_regex.search(code_real))

    def test_fake_test_detection(self):
        empty_test = "#[test]\nfn test_dummy() {\n}"
        self.assertIsNotNone(self.fake_test_regex.search(empty_test))

        tautology_test = "#[test]\nfn test_tautology() {\n  assert!(true);\n}"
        self.assertIsNotNone(self.fake_test_regex.search(tautology_test))

        real_test = "#[test]\nfn test_real() {\n  assert_eq!(2 + 2, 4);\n}"
        self.assertIsNone(self.fake_test_regex.search(real_test))

if __name__ == "__main__":
    unittest.main()
