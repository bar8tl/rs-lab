rmdir c:\rbrust\lab-rs\xlscalamine-rs\target\debug\.fingerprint\xlscalamine-rs-d72ae7e9a2d45b14 /S /Q
cd c:\rbrust\lab-rs\xlscalamine-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\xlscalamine.rs\xlsread.rs main.rs
cd ..
cargo build
cd target\debug\
copy xlscalamine-rs.exe xlsread.exe
pause


