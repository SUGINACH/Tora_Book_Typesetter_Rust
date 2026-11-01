#![deny(clippy::unwrap_used)]

fn main() {
    let _value = Some("intentional quality-gate probe").unwrap();
}
