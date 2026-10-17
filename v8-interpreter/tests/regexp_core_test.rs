//! C-2b 拆家第一刀回归钉: `RegExpCore` 规范化编译与 `regex_split_parts` 从宿主逐字搬入,
//! 本测锁搬移前后行为一致 (口径来源: 门禁 68/70 的 node 对拍形与 value.rs 原实现注释)

use v8_interpreter::{RegExpCore, regex_split_parts};

#[test]
fn regexp_core_normalization_and_compile() {
  // i/m 折 (?i)(?m) 前缀
  let re = RegExpCore::new("abc", "im").unwrap();
  assert!(re.compiled.is_match("ABC").unwrap());
  assert!(re.ignore_case && re.multiline && !re.global);
  // 未闭合字符类: 规范化原样保留 `[`, fancy-regex 编译报错 (宿主包 Invalid RegExp)
  assert!(RegExpCore::new("[a", "").is_err());
  // `[[]` 内层 `[` 转义
  let re = RegExpCore::new("[[]", "").unwrap();
  assert!(re.compiled.is_match("[").unwrap());
  // \xHH >= 0x80 改写 \u{HH}
  let re = RegExpCore::new("\\xff", "").unwrap();
  assert!(re.compiled.is_match("\u{ff}").unwrap());
  // 非法模式报错透传 fancy-regex 原文 (宿主负责包 Invalid RegExp 前缀)
  assert!(RegExpCore::new("(", "").is_err());
}

#[test]
fn regexp_core_flag_surface() {
  let re = RegExpCore::new("a", "gy").unwrap();
  assert!(re.tracks_last_index() && re.sticky());
  let [g, i, m, y, u, d] = re.boolean_flags();
  assert!(g.1 && !i.1 && !m.1 && y.1 && !u.1 && !d.1);
  assert_eq!(i.0, "ignoreCase");
  assert_eq!(d.0, "dotAll");
  // 非 g/y 不推进 lastIndex
  let re = RegExpCore::new("a", "u").unwrap();
  assert!(!re.tracks_last_index() && re.boolean_flags()[4].1);
}

#[test]
fn regex_split_parts_pinned_shapes() {
  let re = RegExpCore::new("[12]", "g").unwrap();
  assert_eq!(
    regex_split_parts("a1b2c3", &re, None)
      .into_iter()
      .map(Option::unwrap)
      .collect::<Vec<_>>(),
    ["a", "b", "c3"]
  );
  // 捕获组逐个入列 (门禁 68: "(x)(y)" 形)
  let re = RegExpCore::new("(a)(b)", "").unwrap();
  let parts: Vec<Option<String>> = regex_split_parts("xxabyyab", &re, None);
  assert_eq!(
    parts.iter().map(|p| p.as_deref()).collect::<Vec<_>>(),
    [
      Some("xx"),
      Some("a"),
      Some("b"),
      Some("yy"),
      Some("a"),
      Some("b"),
      Some("")
    ]
  );
  // 未参与组吐 None
  let re = RegExpCore::new("(r)", "").unwrap();
  assert_eq!(regex_split_parts("q", &re, None), vec![Some("q".into())]);
  // limit 截断 + 零宽空匹配前进不死循环
  let re = RegExpCore::new("", "").unwrap();
  let parts = regex_split_parts("ab", &re, Some(2));
  assert_eq!(parts.len(), 2);
  let re = RegExpCore::new(",", "").unwrap();
  assert_eq!(
    regex_split_parts("1,2", &re, None)
      .into_iter()
      .map(Option::unwrap)
      .collect::<Vec<_>>(),
    ["1", "2"]
  );
}
