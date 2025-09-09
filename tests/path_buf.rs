use rstest::rstest;

#[rstest]
#[cfg(feature = "alloc")]
#[case("/home/hello", "./join", "/home/hello/join")]
#[case("/home/hello", "../user", "/home/user")]
#[case("/home/hello", "/usr/bin", "/usr/bin")]
fn join(#[case] path1: path_rs::PathBuf, #[case] path2: path_rs::PathBuf, #[case] expected: path_rs::PathBuf) {
    assert_eq!(path1.join(path2), expected);
}
