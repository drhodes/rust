/*
Module: os

TODO: Restructure and document
*/

import ctypes::*;

export libc;
export libc_constants;
export pipe;
export fd_FILE;
export close;
export fclose;
export waitpid;
export getcwd;
export exec_suffix;
export target_os;
export dylib_filename;
export get_exe_path;
export fsync_fd;

// FIXME Somehow merge stuff duplicated here and macosx_os.rs. Made difficult
// by https://github.com/graydon/rust/issues#issue/268

#[link_name = ""]
#[abi = "cdecl"]
native mod libc {
    fn read(fd: fd_t, buf: *u8, count: size_t) -> ssize_t;
    fn write(fd: fd_t, buf: *u8, count: size_t) -> ssize_t;
    fn fread(buf: *u8, size: size_t, n: size_t, f: libc::FILE) -> size_t;
    fn fwrite(buf: *u8, size: size_t, n: size_t, f: libc::FILE) -> size_t;
    fn open(s: str::sbuf, flags: c_int, mode: unsigned) -> fd_t;
    fn close(fd: fd_t) -> c_int;
    type FILE;
    fn fopen(path: str::sbuf, mode: str::sbuf) -> FILE;
    fn fdopen(fd: fd_t, mode: str::sbuf) -> FILE;
    fn fclose(f: FILE);
    fn fflush(f: FILE) -> c_int;
    fn fsync(fd: fd_t) -> c_int;
    fn fdatasync(fd: fd_t) -> c_int;
    fn fileno(f: FILE) -> fd_t;
    fn fgetc(f: FILE) -> c_int;
    fn ungetc(c: c_int, f: FILE);
    fn feof(f: FILE) -> c_int;
    fn fseek(f: FILE, offset: long, whence: c_int) -> c_int;
    fn ftell(f: FILE) -> long;
    type dir;
    fn opendir(d: str::sbuf) -> dir;
    fn closedir(d: dir) -> c_int;
    type dirent;
    fn readdir(d: dir) -> dirent;
    fn getenv(n: str::sbuf) -> str::sbuf;
    fn setenv(n: str::sbuf, v: str::sbuf, overwrite: c_int) -> c_int;
    fn unsetenv(n: str::sbuf) -> c_int;
    fn pipe(buf: *mutable fd_t) -> c_int;
    fn waitpid(pid: pid_t, &status: c_int, options: c_int) -> pid_t;
    fn readlink(path: str::sbuf, buf: str::sbuf, bufsize: size_t) -> ssize_t;
    fn mkdir(path: str::sbuf, mode: c_int) -> c_int;
    fn rmdir(path: str::sbuf) -> c_int;
    fn chdir(path: str::sbuf) -> c_int;
}

mod libc_constants {
    const O_RDONLY: c_int = 0i32;
    const O_WRONLY: c_int = 1i32;
    const O_RDWR: c_int   = 2i32;
    const O_APPEND: c_int = 1024i32;
    const O_CREAT: c_int  = 64i32;
    const O_EXCL: c_int   = 128i32;
    const O_TRUNC: c_int  = 512i32;
    const O_TEXT: c_int   = 0i32;     // nonexistent in linux libc
    const O_BINARY: c_int = 0i32;     // nonexistent in linux libc

    const S_IRUSR: unsigned = 256u32;
    const S_IWUSR: unsigned = 128u32;
}

fn pipe() -> {in: fd_t, out: fd_t} {
    let fds = {mutable in: 0i32, mutable out: 0i32};
    assert (os::libc::pipe(ptr::mut_addr_of(fds.in)) == 0i32);
    ret {in: fds.in, out: fds.out};
}

fn fd_FILE(fd: fd_t) -> libc::FILE {
    ret str::as_buf("r", {|modebuf| libc::fdopen(fd, modebuf) });
}

fn close(fd: fd_t) -> c_int {
    libc::close(fd)
}

fn fclose(file: libc::FILE) {
    libc::fclose(file)
}

fn fsync_fd(fd: fd_t, level: io::fsync::level) -> c_int {
    alt level {
      io::fsync::fsync. | io::fsync::fullfsync. { ret libc::fsync(fd); }
      io::fsync::fdatasync. { ret libc::fdatasync(fd); }
    }
}

fn waitpid(pid: pid_t) -> i32 {
    let status = 0i32;
    assert (os::libc::waitpid(pid, status, 0i32) != -1i32);
    ret status;
}

#[abi = "cdecl"]
native mod rustrt {
    fn rust_getcwd() -> str;
}

fn getcwd() -> str { ret rustrt::rust_getcwd(); }

fn exec_suffix() -> str { ret ""; }

fn target_os() -> str { ret "linux"; }

fn dylib_filename(base: str) -> str { ret "lib" + base + ".so"; }

/// Returns the directory containing the running program
/// followed by a path separator
fn get_exe_path() -> option::t<fs::path> {
    let bufsize = 1023u;
    let path = str::unsafe_from_bytes(vec::init_elt(0u8, bufsize));
    ret str::as_buf("/proc/self/exe", { |proc_self_buf|
        str::as_buf(path, { |path_buf|
            if libc::readlink(proc_self_buf, path_buf, bufsize) != -1 {
                option::some(fs::dirname(path) + fs::path_sep())
            } else {
                option::none
            }
        })
    });
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
