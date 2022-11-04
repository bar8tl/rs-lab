rmdir c:\rbrust\lab-rs\rusqlite-rs\target\debug\.fingerprint\rusqlite-rs-0055f24ea961e12f /S /Q
cd c:\rbrust\lab-rs\rusqlite-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\rusqlite.rs\myseek.rs main.rs
cd ..
copy c:\c-portab\01-rb\pgmfiles\lab.rs\rusqlite.rs\cargo.toml .
cargo build
cd target\debug\
copy rusqlite-rs.exe myseek.exe
pause


