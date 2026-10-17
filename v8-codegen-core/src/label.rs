use crate::error::{Error, Result};

/// 标号跳转距离 (近跳转 8 位, 远跳转 32 位)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distance {
  Near,
  Far,
}

/// 标号 (对应原版 v8::internal::Label)
///
/// 内部通过 pos_ 与 near_link_pos_ 记录标号状态：
/// - pos_ < 0: 已绑定 (bound)，pos() 返回绑定位置 -pos_ - 1
/// - pos_ == 0 且 near_link_pos_ == 0: 未使用 (unused)
/// - pos_ > 0: 远跳转链接 (linked)，pos() 返回最后一次引用的位置 pos_ - 1
/// - near_link_pos_ > 0: 近跳转链接 (near_linked)，near_link_pos() 返回最后引用位置 near_link_pos_ - 1
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Label {
  pos_: i32,
  near_link_pos_: i32,
}

impl Label {
  #[inline]
  pub const fn new() -> Self {
    Self {
      pos_: 0,
      near_link_pos_: 0,
    }
  }

  #[inline]
  pub const fn is_bound(&self) -> bool {
    self.pos_ < 0
  }

  #[inline]
  pub const fn is_unused(&self) -> bool {
    self.pos_ == 0 && self.near_link_pos_ == 0
  }

  #[inline]
  pub const fn is_linked(&self) -> bool {
    self.pos_ > 0
  }

  #[inline]
  pub const fn is_near_linked(&self) -> bool {
    self.near_link_pos_ > 0
  }

  #[inline]
  pub fn pos(&self) -> Result<i32> {
    if self.pos_ < 0 {
      Ok(-self.pos_ - 1)
    } else if self.pos_ > 0 {
      Ok(self.pos_ - 1)
    } else {
      Err(Error::LabelUnused)
    }
  }

  #[inline]
  pub fn near_link_pos(&self) -> Result<i32> {
    if self.near_link_pos_ > 0 {
      Ok(self.near_link_pos_ - 1)
    } else {
      Err(Error::LabelUnused)
    }
  }

  #[inline]
  pub fn unuse(&mut self) {
    self.pos_ = 0;
  }

  #[inline]
  pub fn unuse_near(&mut self) {
    self.near_link_pos_ = 0;
  }

  #[inline]
  pub fn bind_to(&mut self, pos: i32) -> Result<()> {
    if self.is_bound() {
      return Err(Error::LabelAlreadyBound(self.pos()?));
    }
    self.pos_ = -pos - 1;
    Ok(())
  }

  #[inline]
  pub fn link_to(&mut self, pos: i32, distance: Distance) {
    match distance {
      Distance::Near => {
        self.near_link_pos_ = pos + 1;
      }
      Distance::Far => {
        self.pos_ = pos + 1;
      }
    }
  }
}
