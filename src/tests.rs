use crate::scheme::{ColorId, get_lynx_color, get_panther_color};

#[test]
fn panther_names() {
    assert_eq!(get_panther_color(ColorId::N0).name, "n0");
    assert_eq!(get_panther_color(ColorId::N1).name, "n1");
    assert_eq!(get_panther_color(ColorId::N2).name, "n2");
    assert_eq!(get_panther_color(ColorId::N3).name, "n3");
    assert_eq!(get_panther_color(ColorId::N4).name, "n4");
    assert_eq!(get_panther_color(ColorId::T0).name, "t0");
    assert_eq!(get_panther_color(ColorId::T1).name, "t1");
    assert_eq!(get_panther_color(ColorId::T2).name, "t2");
    assert_eq!(get_panther_color(ColorId::T3).name, "t3");
    assert_eq!(get_panther_color(ColorId::Red).name, "red");
    assert_eq!(get_panther_color(ColorId::Orange).name, "orange");
    assert_eq!(get_panther_color(ColorId::Yellow).name, "yellow");
    assert_eq!(get_panther_color(ColorId::Green).name, "green");
    assert_eq!(get_panther_color(ColorId::NeonGreen).name, "neon-green");
    assert_eq!(get_panther_color(ColorId::Blue).name, "blue");
    assert_eq!(get_panther_color(ColorId::Cyan).name, "cyan");
    assert_eq!(get_panther_color(ColorId::Purple).name, "purple");
    assert_eq!(get_panther_color(ColorId::Pink).name, "pink");
}

#[test]
fn lynx_names() {
    assert_eq!(get_lynx_color(ColorId::N0).name, "n0");
    assert_eq!(get_lynx_color(ColorId::N1).name, "n1");
    assert_eq!(get_lynx_color(ColorId::N2).name, "n2");
    assert_eq!(get_lynx_color(ColorId::N3).name, "n3");
    assert_eq!(get_lynx_color(ColorId::N4).name, "n4");
    assert_eq!(get_lynx_color(ColorId::T0).name, "t0");
    assert_eq!(get_lynx_color(ColorId::T1).name, "t1");
    assert_eq!(get_lynx_color(ColorId::T2).name, "t2");
    assert_eq!(get_lynx_color(ColorId::T3).name, "t3");
    assert_eq!(get_lynx_color(ColorId::Red).name, "red");
    assert_eq!(get_lynx_color(ColorId::Orange).name, "orange");
    assert_eq!(get_lynx_color(ColorId::Yellow).name, "yellow");
    assert_eq!(get_lynx_color(ColorId::Green).name, "green");
    assert_eq!(get_lynx_color(ColorId::NeonGreen).name, "neon-green");
    assert_eq!(get_lynx_color(ColorId::Blue).name, "blue");
    assert_eq!(get_lynx_color(ColorId::Cyan).name, "cyan");
    assert_eq!(get_lynx_color(ColorId::Purple).name, "purple");
    assert_eq!(get_lynx_color(ColorId::Pink).name, "pink");
}
