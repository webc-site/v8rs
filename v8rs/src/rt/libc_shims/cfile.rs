// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use nix::{
  errno::Errno,
  fcntl::open as nix_open,
  sys::stat::Mode,
  unistd::{Whence, lseek, read, write},
};

use crate::rt::{ByteRepr, FdRegistry, cpp2rust_errno};

pub struct CFile {
  pub fd: i32,
  pub eof: bool,
  pub err: bool,
}

impl CFile {
  pub fn new(fd: i32) -> Self {
    CFile {
      fd,
      eof: false,
      err: false,
    }
  }

  pub fn open(path: &str, mode: &str) -> Option<CFile> {
    use nix::fcntl::OFlag;
    let mut chars = mode.chars();
    let mut flags = match chars.next() {
      Some('r') => OFlag::O_RDONLY,
      Some('w') => OFlag::O_WRONLY.union(OFlag::O_CREAT).union(OFlag::O_TRUNC),
      Some('a') => OFlag::O_WRONLY.union(OFlag::O_CREAT).union(OFlag::O_APPEND),
      _ => panic!("fopen: unsupported mode {:?}", mode),
    };
    for c in chars {
      match c {
        'b' => {}
        '+' => {
          flags.remove(OFlag::O_WRONLY);
          flags.insert(OFlag::O_RDWR);
        }
        'x' => flags.insert(OFlag::O_EXCL),
        'e' => flags.insert(OFlag::O_CLOEXEC),
        _ => panic!("fopen: unsupported mode {:?}", mode),
      }
    }
    match nix_open(path, flags, Mode::from_bits_truncate(0o666)) {
      Ok(ofd) => Some(CFile::new(FdRegistry::register(ofd))),
      Err(e) => {
        cpp2rust_errno().write(e as i32);
        None
      }
    }
  }

  pub fn read(&mut self, buf: &mut [u8]) -> usize {
    let mut n = 0;
    while n < buf.len() {
      match FdRegistry::with_fd(self.fd, |b| read(b, &mut buf[n..])) {
        Ok(0) => {
          self.eof = true;
          break;
        }
        Ok(k) => n += k,
        Err(Errno::EINTR) => {}
        Err(e) => {
          self.err = true;
          cpp2rust_errno().write(e as i32);
          break;
        }
      }
    }
    n
  }

  pub fn write(&mut self, buf: &[u8]) -> usize {
    let mut n = 0;
    while n < buf.len() {
      match FdRegistry::with_fd(self.fd, |b| write(b, &buf[n..])) {
        Ok(0) => {
          self.err = true;
          break;
        }
        Ok(k) => n += k,
        Err(Errno::EINTR) => {}
        Err(e) => {
          self.err = true;
          cpp2rust_errno().write(e as i32);
          break;
        }
      }
    }
    n
  }

  pub fn seek(&mut self, offset: i64, whence: i32) -> i64 {
    let w = match whence {
      0 => Whence::SeekSet,
      1 => Whence::SeekCur,
      2 => Whence::SeekEnd,
      other => panic!("fseek: unsupported whence {}", other),
    };
    match FdRegistry::with_fd(self.fd, |b| lseek(b, offset, w)) {
      Ok(off) => {
        self.eof = false;
        off
      }
      Err(e) => {
        cpp2rust_errno().write(e as i32);
        -1
      }
    }
  }

  pub fn tell(&self) -> i64 {
    match FdRegistry::with_fd(self.fd, |b| lseek(b, 0, Whence::SeekCur)) {
      Ok(off) => off,
      Err(e) => {
        cpp2rust_errno().write(e as i32);
        -1
      }
    }
  }

  pub fn getc(&mut self) -> i32 {
    let mut b = [0u8; 1];
    match self.read(&mut b) {
      1 => b[0] as i32,
      _ => -1,
    }
  }

  pub fn close(&self) -> i32 {
    FdRegistry::close(self.fd)
  }
}

impl ByteRepr for CFile {}
