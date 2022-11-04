rmdir c:\rbrust\lab-rs\rust_xlsxwriter-rs\target\debug\.fingerprint\rust_xlsxwriter-rs-feec119d6ad4ff93 /S /Q
cd c:\rbrust\lab-rs\rust_xlsxwriter-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\rust_xlsxwriter.rs\example.rs main.rs
cd ..
cargo build
cd target\debug\
copy rust_xlsxwriter-rs.exe example.exe
pause


