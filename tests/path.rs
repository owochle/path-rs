use path_rs::Path;
use rstest::rstest;

#[test]
fn create() {
    let p = Path::new("/home/hello");

    assert_eq!(p, "/home/hello");
}

#[rstest]
#[case("/home/hello.txt", Some("hello.txt"))]
#[case("/home/hello.tar.gz", Some("hello.tar.gz"))]
#[case("/home/hello/", Some("hello"))]
#[case("/home/hello", Some("hello"))]
#[case("", None)]
#[case("/", None)]
#[case("..", None)]
#[case(".", None)]
fn file_name(#[case] path: &str, #[case] expected: Option<&str>) {
    let p = Path::new(path);

    assert_eq!(p.file_name(), expected);
}

#[rstest]
#[case("/home/hello.txt", Some("hello"))]
#[case("/home/hello.tar.gz", Some("hello.tar"))]
#[case("/home/hello/", Some("hello"))]
#[case("/home/hello", Some("hello"))]
#[case("", None)]
#[case("/", None)]
#[case("..", None)]
#[case(".", None)]
fn file_stem(#[case] path: &str, #[case] expected: Option<&str>) {
    let p = Path::new(path);

    assert_eq!(p.file_stem(), expected);
}

#[rstest]
#[case("/home/hello.txt", Some("hello"))]
#[case("/home/hello.tar.gz", Some("hello"))]
#[case("/home/hello/", Some("hello"))]
#[case("/home/hello", Some("hello"))]
#[case("", None)]
#[case("/", None)]
#[case("..", None)]
#[case(".", None)]
fn file_prefix(#[case] path: &str, #[case] expected: Option<&str>) {
    let p = Path::new(path);

    assert_eq!(p.file_prefix(), expected)
}

#[rstest]
#[case("/home/hello.tar.gz", Some("gz"))]
#[case("/home/hello.txt", Some("txt"))]
#[case("/home/hello.bonjour/hello.txt", Some("txt"))]
#[case("/home/hello", None)]
#[case("/home/.bashrc", None)]
#[case("", None)]
#[case("/", None)]
#[case("..", None)]
#[case(".", None)]
fn extension(#[case] path: &str, #[case] expected: Option<&str>) {
    let p = Path::new(path);

    assert_eq!(p.extension(), expected)
}

#[rstest]
#[case("/home/hello", "/home/hello", true)]
#[case("/home//hello", "/home/hello", true)]
#[case("./home////hello", "../home/hello", false)]
#[case("../home/hello", "/home/hello", false)]
#[case("/home/././hello", "/home/hello", true)]
#[case("/home/..////./etc/../usr/share/./fonts", "/usr/share/fonts", true)]
fn equality(#[case] path1: &str, #[case] path2: &str, #[case] expected: bool) {
    let p1 = Path::new(path1);
    let p2 = Path::new(path2);

    assert_eq!(p1.eq(p2), expected);
}

#[rstest]
#[case("/home/hello", true)]
#[case("./home/hello", false)]
#[case("/", true)]
#[case("..", false)]
#[case("././.././hello", false)]
#[case("/./hello", true)]
fn is_absolute(#[case] path: &str, #[case] expected: bool) {
    let p = Path::new(path);

    assert_eq!(p.is_absolute(), expected);
}