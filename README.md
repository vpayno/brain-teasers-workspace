# brain-teasers-workspace

Just read `Rust Brain Teasers` and I needed a place to play with the code.
Also using this workspace to try the brain teasers in other languages.

## Brain Teasers

### Three and a Bit

```text
$ cargo run --bin three_and_a_bit
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/three_and_a_bit three_and_a_bit`
Wanted: 3.4028236
   Got: 3.4028237
```

### Non-Standard Input

```text
$ cargo run --bin non-std-input
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/non-std-input`

Type your answer and press enter.
What is 3+2?
5

Response: "5\n"
Trimmed: "5"

Correct!

$ cargo run --bin non-std-input
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/non-std-input`

Type your answer and press enter.
What is 3+2?
7

Response: "7\n"
Trimmed: "7"

Incorrect!
```

### Type Conversion

```text
$ cargo run --bin type-conversion
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/type-conversion`
x1(4294967296) doesn't equal y1(0)
x2(4294967296) equals y2(4294967296)
thread 'main' panicked at type-conversion/src/main.rs:21:33:
Failed to convert from u64 to u32: TryFromIntError(())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ RUST_BACKTRACE=1 cargo run --bin type-conversion
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/type-conversion`
x1(4294967296) doesn't equal y1(0)
x2(4294967296) equals y2(4294967296)
thread 'main' panicked at type-conversion/src/main.rs:21:33:
Failed to convert from u64 to u32: TryFromIntError(())
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:647:5
   1: core::panicking::panic_fmt
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:72:14
   2: core::result::unwrap_failed
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/result.rs:1649:5
   3: core::result::Result<T,E>::expect
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/result.rs:1030:23
   4: type_conversion::main
             at ./type-conversion/src/main.rs:21:19
   5: core::ops::function::FnOnce::call_once
             at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ RUST_BACKTRACE=full cargo run --bin type-conversion
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/type-conversion`
x1(4294967296) doesn't equal y1(0)
x2(4294967296) equals y2(4294967296)
thread 'main' panicked at type-conversion/src/main.rs:21:33:
Failed to convert from u64 to u32: TryFromIntError(())
stack backtrace:
   0:     0x55c00e69a236 - std::backtrace_rs::backtrace::libunwind::trace::h6e4a662bea54ccfc
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/../../backtrace/src/backtrace/libunwind.rs:104:5
   1:     0x55c00e69a236 - std::backtrace_rs::backtrace::trace_unsynchronized::hb42b4eb2797d9c0e
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x55c00e69a236 - std::sys_common::backtrace::_print_fmt::h2bc261f3223f4e4d
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:68:5
   3:     0x55c00e69a236 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9cca0343d66d16a8
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x55c00e6b8ee0 - core::fmt::rt::Argument::fmt::h8b666c45176be671
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/fmt/rt.rs:142:9
   5:     0x55c00e6b8ee0 - core::fmt::write::h4311bce0ee536615
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/fmt/mod.rs:1120:17
   6:     0x55c00e69841f - std::io::Write::write_fmt::h0685c51539d0a0cd
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/io/mod.rs:1846:15
   7:     0x55c00e69a014 - std::sys_common::backtrace::_print::h25f19b1d64e81f86
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x55c00e69a014 - std::sys_common::backtrace::print::h2fb8f70628a241ed
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x55c00e69b577 - std::panicking::default_hook::{{closure}}::h05093fe2e3ef454d
  10:     0x55c00e69b2d9 - std::panicking::default_hook::h5ac38aa38e0086d2
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:292:9
  11:     0x55c00e69ba08 - std::panicking::rust_panic_with_hook::hed79743dc8b4b969
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:781:13
  12:     0x55c00e69b8e2 - std::panicking::begin_panic_handler::{{closure}}::ha437b5d58f431abf
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:659:13
  13:     0x55c00e69a736 - std::sys_common::backtrace::__rust_end_short_backtrace::hd98e82d5b39ec859
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:171:18
  14:     0x55c00e69b634 - rust_begin_unwind
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:647:5
  15:     0x55c00e67e0e5 - core::panicking::panic_fmt::hc69c4d258fe11477
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/panicking.rs:72:14
  16:     0x55c00e67e553 - core::result::unwrap_failed::hff299ec748d62aab
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/result.rs:1649:5
  17:     0x55c00e67ed40 - core::result::Result<T,E>::expect::h0973832121af6a60
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/result.rs:1030:23
  18:     0x55c00e67f176 - type_conversion::main::h986510cef192d252
                               at /home/vpayno/git_vpayno/brain-teasers-workspace/type-conversion/src/main.rs:21:19
  19:     0x55c00e67ec4b - core::ops::function::FnOnce::call_once::h86b1730ea98a684d
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:250:5
  20:     0x55c00e67ebce - std::sys_common::backtrace::__rust_begin_short_backtrace::hce82ff96ca4e6e30
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/sys_common/backtrace.rs:155:18
  21:     0x55c00e67f3d1 - std::rt::lang_start::{{closure}}::h595a572b5b246a07
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:166:18
  22:     0x55c00e695be1 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h8c00837184d6f522
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/core/src/ops/function.rs:284:13
  23:     0x55c00e695be1 - std::panicking::try::do_call::hfd8273c3b0a89311
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:554:40
  24:     0x55c00e695be1 - std::panicking::try::h389092b34a7cd1de
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:518:19
  25:     0x55c00e695be1 - std::panic::catch_unwind::hd784b6d233abd2d5
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panic.rs:142:14
  26:     0x55c00e695be1 - std::rt::lang_start_internal::{{closure}}::h8d693c96d9aec4f8
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:148:48
  27:     0x55c00e695be1 - std::panicking::try::do_call::h62b442e92648318b
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:554:40
  28:     0x55c00e695be1 - std::panicking::try::h3dd33ffb0232e2bf
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panicking.rs:518:19
  29:     0x55c00e695be1 - std::panic::catch_unwind::h388a168fe640f06a
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/panic.rs:142:14
  30:     0x55c00e695be1 - std::rt::lang_start_internal::hdaf8b62dc8f7de54
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:148:20
  31:     0x55c00e67f3aa - std::rt::lang_start::h9334eecc88da7978
                               at /rustc/7cf61ebde7b22796c69757901dd346d0fe70bd97/library/std/src/rt.rs:165:17
  32:     0x55c00e67f36e - main
  33:     0x7f5feab9224a - __libc_start_call_main
                               at ./csu/../sysdeps/nptl/libc_start_call_main.h:58:16
  34:     0x7f5feab92305 - __libc_start_main_impl
                               at ./csu/../csu/libc-start.c:360:3
  35:     0x55c00e67e811 - _start
  36:                0x0 - <unknown>
```
