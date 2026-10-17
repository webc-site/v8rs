// 由 tool/src/strings/owning-external-string-resource.h.js 从 v8/src/strings/owning-external-string-resource.h 生成; 修订就地改, 复跑覆盖
// 独占外部字符串资源骨架与实现

/// 独占式外部字符串资源 (原版 OwningExternalStringResourceImpl 模板)。
/// 存储策略分支 (原版 #ifdef V8_ENABLE_MEMORY_CORRUPTION_API):
///   关闭 → std::unique_ptr<CharT[]> (默认, x64/arm64 常规构建)
///   开启 → ExternalStringsCage 池分配 + Seal 封印
pub mod owning_resource_skel {
  /// 资源骨架: data() + length() 两访问器 (原版 override data/length)
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct OwningExternalStringResourceSkel<CharT> {
    /// 只读长度 (原版 const size_t length_)
    pub length: usize,
    /// 独占存储 (原版 const Storage storage_)
    pub storage: Box<[CharT]>,
  }

  impl<CharT: Default + Clone> OwningExternalStringResourceSkel<CharT> {
    /// 从切片拷贝构建外部资源
    #[inline]
    #[must_use]
    pub fn new(slice: &[CharT]) -> Self {
      let mut res = Self {
        length: slice.len(),
        storage: Box::<[CharT]>::from(slice),
      };
      res.seal_if_supported();
      res
    }

    /// 分配指定容量的外部资源
    #[inline]
    #[must_use]
    pub fn with_capacity(len: usize) -> Self {
      let mut res = Self {
        length: len,
        storage: Self::create_uninitialized_storage(len),
      };
      res.seal_if_supported();
      res
    }

    /// 未初始化存储分配 (原版 CreateUninitializedStorage, cage 关闭路径)
    #[inline]
    fn create_uninitialized_storage(len: usize) -> Box<[CharT]> {
      vec![CharT::default(); len].into_boxed_slice()
    }

    /// 构造后封印 (原版 SealIfSupported; cage 开启路径)
    #[inline]
    fn seal_if_supported(&mut self) {
      // 默认非 cage 保护模式下为 no-op
    }

    /// 数据切片借用 (原版 data())
    #[inline]
    #[must_use]
    pub fn data(&self) -> &[CharT] {
      &self.storage[..self.length]
    }

    /// 资源长度 (原版 length())
    #[inline]
    #[must_use]
    pub fn length(&self) -> usize {
      self.length
    }

    /// 是否为空
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
      self.length == 0
    }
  }

  /// 一字节别名 (原版 OwningExternalOneByteStringResource: char/char/uint8_t)
  pub type OwningExternalOneByteStringResource = OwningExternalStringResourceSkel<u8>;
  /// 双字节别名 (原版 OwningExternalStringResource: uc16/char16_t/uc16)
  pub type OwningExternalStringResource = OwningExternalStringResourceSkel<u16>;

  /// 字宽断言对数守卫 (原版 static_assert ×2)
  pub const WIDTH_ASSERTS: usize = 2;
}
