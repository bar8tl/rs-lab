rmdir c:\rbrust\lab-rs\xlsxwriter-rs\target\debug\.fingerprint\xlsxwriter-rs-cab7861e02a3898e /S /Q
cd c:\rbrust\lab-rs\xlsxwriter-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\xlsxwriter-rs\example1.rs main.rs
cd ..
cargo build
cd target\debug\
copy xlsxwriter-rs.exe example1.exe
pause



