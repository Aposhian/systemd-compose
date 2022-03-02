use systemd_compose::{Compose, convert};
use systemd_schema::prelude::*;

// https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust
macro_rules! convert_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            let compose: Compose = serde_yaml::from_str(&input).unwrap();
            let services = convert(compose).unwrap();
            assert_eq!(expected, serde_ini::to_string(&services[0]).unwrap());
        }
    )*
    }
}

convert_tests! {
    simple: (
"
services:
  nop:
    image: busybox
    command: cat /etc/hostname
",
"[Unit]
WantedBy=multi-user.target

[Service]
Environment=PODMAN_SYSTEMD_UNIT=%n
ExecStart=/usr/bin/podman start
Type=simple
ExecStart=
[Install]
"
    ),
}