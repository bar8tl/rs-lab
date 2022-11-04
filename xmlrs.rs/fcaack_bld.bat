rmdir C:\rbrust\lab-rs\xmlrs-rs\target\debug\.fingerprint\xml-rs-6c08e03bdbeecbbb /S /Q
cd c:\rbrust\lab-rs\xmlrs-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\xmlrs.rs\fcaack.rs main.rs
cd ..
cargo build
cd target\debug\
copy xmlrs-rs.exe fcaack.exe
pause
