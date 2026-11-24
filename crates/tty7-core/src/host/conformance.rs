use std::io;
use std::path::Path;

use super::{Host, MTime};

pub type Case = fn(h: &dyn Host, sandbox: &dyn Sandbox);

pub trait Sandbox {
    fn path(&self) -> &Path;

    fn symlink(&self, target: &Path, link: &Path) -> Option<io::Result<()>> {
        let _ = (target, link);
        None
    }
}

#[macro_export]
macro_rules! for_each_host_case {
    ($cb:ident $(, $extra:tt)*) => {
        $cb! {
            $($extra)*
            @cases
            read_dir_lists_and_sorts,
            read_dir_includes_hidden,
            read_dir_missing_is_not_found,
            read_dir_on_a_file_errors,
            read_dir_symlink_to_dir_is_dir,
            stat_reports_len_and_mtime,
            stat_missing_is_not_found,
            exists_matches_stat,
            canonicalize_resolves_dotdot,
            read_file_roundtrips_bytes,
            read_file_over_max_bytes_errors,
            read_file_on_a_dir_errors,
            join_uses_host_separator,
            shells_are_named_and_have_a_default,
            is_connected_is_true_when_healthy,
            id_is_stable_across_calls,
            separator_matches_hello,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __host_case_table {
    (@cases $($name:ident),* $(,)?) => {
                                pub const CASES: &[(&str, $crate::host::conformance::Case)] = &[
            $((stringify!($name), $name as $crate::host::conformance::Case)),*
        ];
    };
}

crate::for_each_host_case!(__host_case_table);

#[doc(hidden)]
#[macro_export]
macro_rules! __host_case_tests {
    (($factory:expr) @cases $($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let (host, sandbox) = ($factory)();
                $crate::host::conformance::$name(&*host, &sandbox);
            }
        )*
    };
}

#[macro_export]
macro_rules! host_conformance_suite {
    ($modname:ident, $factory:expr) => {
        mod $modname {
            #[allow(unused_imports)]
            use super::*;
            use $crate::__host_case_tests;

            $crate::for_each_host_case!(__host_case_tests, ($factory));
        }
    };
}

fn write(p: &Path, body: &str) {
    std::fs::write(p, body.as_bytes()).unwrap_or_else(|e| panic!("write {}: {e}", p.display()));
}

fn put(p: &Path, bytes: &[u8]) {
    std::fs::write(p, bytes).unwrap_or_else(|e| panic!("write {}: {e}", p.display()));
}

fn mkdir(p: &Path) {
    std::fs::create_dir_all(p).unwrap_or_else(|e| panic!("mkdir {}: {e}", p.display()));
}

fn names(entries: &[super::Entry]) -> Vec<&str> {
    entries.iter().map(|e| e.name.as_str()).collect()
}

pub fn read_dir_lists_and_sorts(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    mkdir(&h.join(sandbox, "src"));
    mkdir(&h.join(sandbox, "Assets"));
    write(&h.join(sandbox, "main.rs"), "");
    write(&h.join(sandbox, "Cargo.toml"), "");
    write(&h.join(sandbox, "zeta.txt"), "");

    let listed = h.read_dir(sandbox).unwrap();
    assert_eq!(
        names(&listed),
        vec!["Assets", "src", "Cargo.toml", "main.rs", "zeta.txt"],
        "directories first, then case-insensitive by name"
    );
}

pub fn read_dir_includes_hidden(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    write(&h.join(sandbox, ".hidden"), "");
    write(&h.join(sandbox, "visible"), "");
    let listed = h.read_dir(sandbox).unwrap();
    assert!(names(&listed).contains(&".hidden"), "{:?}", names(&listed));
}

pub fn read_dir_missing_is_not_found(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let err = h.read_dir(&h.join(sandbox, "nope")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound, "{err}");
}

pub fn read_dir_on_a_file_errors(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let f = h.join(sandbox, "file.txt");
    write(&f, "hi");
    assert!(h.read_dir(&f).is_err());
}

pub fn read_dir_symlink_to_dir_is_dir(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let target = h.join(sandbox, "real");
    let link = h.join(sandbox, "link");
    mkdir(&target);
    let Some(made) = sb.symlink(&target, &link) else {
        return;
    };
    if made.is_err() {
        return;
    }

    let listed = h.read_dir(sandbox).unwrap();
    let l = listed
        .iter()
        .find(|e| e.name == "link")
        .expect("link listed");
    assert!(l.is_dir, "a link to a directory resolves as a directory");
    assert!(l.is_symlink, "and still reports as a link");
}

pub fn stat_reports_len_and_mtime(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let f = h.join(sandbox, "sized.txt");
    write(&f, "hello");
    let m = h.stat(&f).unwrap();
    assert_eq!(m.len, 5);
    assert!(!m.is_dir);
    assert!(!m.is_symlink);
    let mtime = m.mtime.expect("a real filesystem has mtimes");
    assert!(mtime > MTime { secs: 0, nanos: 0 }, "{mtime:?}");

    let d = h.join(sandbox, "adir");
    mkdir(&d);
    assert!(h.stat(&d).unwrap().is_dir);
}

pub fn stat_missing_is_not_found(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let err = h.stat(&h.join(sandbox, "ghost")).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound, "{err}");
}

pub fn exists_matches_stat(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let f = h.join(sandbox, "there.txt");
    write(&f, "x");
    assert_eq!(h.exists(&f), h.stat(&f).is_ok());
    assert!(h.exists(&f));

    let missing = h.join(sandbox, "not-there");
    assert_eq!(h.exists(&missing), h.stat(&missing).is_ok());
    assert!(!h.exists(&missing));

    let nested = h.join(&f, "child");
    assert_eq!(h.exists(&nested), h.stat(&nested).is_ok());
}

pub fn canonicalize_resolves_dotdot(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let a = h.join(sandbox, "a");
    let b = h.join(sandbox, "b");
    mkdir(&a);
    mkdir(&b);

    let round_about = h.join(&h.join(&a, ".."), "b");
    let canon = h.canonicalize(&round_about).unwrap();
    let direct = h.canonicalize(&b).unwrap();
    assert_eq!(canon, direct, "a/../b is b");
}

pub fn read_file_roundtrips_bytes(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let f = h.join(sandbox, "bytes.bin");
    let mut body: Vec<u8> = vec![0x00, 0xff, 0xfe, b'a', 0x00, 0x80];
    put(&f, &body);
    assert_eq!(h.read_file(&f, 1024).unwrap(), body);

    let big = h.join(sandbox, "big.bin");
    body = (0..10 * 1024 * 1024u32).map(|i| (i % 251) as u8).collect();
    put(&big, &body);
    let back = h.read_file(&big, 32 * 1024 * 1024).unwrap();
    assert_eq!(back.len(), body.len());
    assert!(back == body, "10MB body round-tripped byte for byte");
}

pub fn read_file_over_max_bytes_errors(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let f = h.join(sandbox, "fat.bin");
    put(&f, &vec![b'x'; 4096]);
    let err = h.read_file(&f, 1024).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::FileTooLarge, "{err}");
    assert_eq!(h.read_file(&f, 4096).unwrap().len(), 4096);
}

pub fn read_file_on_a_dir_errors(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let d = h.join(sandbox, "adir");
    mkdir(&d);
    assert!(h.read_file(&d, 1024 * 1024).is_err());
}

pub fn join_uses_host_separator(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let sep = h.separator();
    let joined = h.join(sandbox, "child");
    let text = joined.to_string_lossy();
    assert!(text.ends_with(&format!("{sep}child")), "{text}");
    assert!(
        text.starts_with(&*sandbox.to_string_lossy()),
        "{text} should extend {}",
        sandbox.display()
    );
    let deep = h.join(&joined, "grand");
    assert!(
        deep.to_string_lossy()
            .ends_with(&format!("child{sep}grand"))
    );
}

pub fn shells_are_named_and_have_a_default(h: &dyn Host, _sb: &dyn Sandbox) {
    let inv = h.shells().expect("a host can list its shells");
    assert!(
        !inv.default_name.trim().is_empty(),
        "no default shell name to tag the menu with"
    );
    let mut seen = std::collections::HashSet::new();
    for shell in &inv.shells {
        assert!(!shell.label.trim().is_empty(), "a shell row with no label");
        assert!(
            !shell.program.trim().is_empty(),
            "shell {:?} has nothing to spawn",
            shell.label
        );
        assert!(
            seen.insert(shell.label.clone()),
            "{:?} is listed twice",
            shell.label
        );
    }
}

pub fn is_connected_is_true_when_healthy(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    assert!(h.is_connected());
    let _ = h.read_dir(sandbox).unwrap();
    assert!(h.is_connected());
}

pub fn id_is_stable_across_calls(h: &dyn Host, _sb: &dyn Sandbox) {
    let first = h.id();
    for _ in 0..8 {
        assert_eq!(h.id(), first);
    }
}

pub fn separator_matches_hello(h: &dyn Host, sb: &dyn Sandbox) {
    let sandbox = sb.path();
    let sep = h.separator();
    assert_eq!(sep, h.separator());
    assert!(
        h.join(sandbox, "x").to_string_lossy().contains(sep),
        "join must use the reported separator"
    );
}

#[cfg(test)]
mod tests {
    use super::CASES;

    #[test]
    fn every_case_is_registered() {
        let src = include_str!("conformance.rs");
        let defined: Vec<&str> = src
            .lines()
            .filter_map(|l| l.strip_prefix("pub fn "))
            .filter(|l| l.contains("&dyn Sandbox"))
            .filter_map(|l| l.split('(').next())
            .collect();

        let registered: Vec<&str> = CASES.iter().map(|(n, _)| *n).collect();
        for name in &defined {
            assert!(
                registered.contains(name),
                "case `{name}` is defined but missing from for_each_host_case!"
            );
        }
        for name in &registered {
            assert!(
                defined.contains(name),
                "case `{name}` is registered but has no `pub fn`"
            );
        }
        assert_eq!(defined.len(), registered.len(), "duplicate registration");
    }
}
