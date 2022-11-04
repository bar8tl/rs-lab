rmdir c:\rbrust\lab-rs\simple_excel_writer-rs\target\debug\.fingerprint\cimple_excel_xwriter-rs-feec119d6ad4ff93 /S /Q
cd c:\rbrust\lab-rs\simple_excel_writer-rs\src\
copy c:\c-portab\01-rb\pgmfiles\lab.rs\simple_excel_writer.rs\example.rs main.rs
cd ..
cargo build
cd target\debug\
copy simple_excel_writer-rs.exe example.exe
pause


