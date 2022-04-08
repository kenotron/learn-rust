This is an dir walker, looking for package.json. Here are some parameters:

1. the target path is HARDCODED
2. the program has just one file
3. it is SYNCHRONOUS
4. it has no dependencies other than std lib

Things I've learned here:

1. There's a big world of difference between &str, String, OsString & PathBuf
2. Keep things on heap (String) if you are producing String's to be placed inside a Vec that was created before a loop
3. Rust std libs are SYNC; be careful!

Next steps:

1. learn about diffs between thread pooling, rayon, and async/await w/ tokio
2. structs, impl, traits
3. mod.rs, lib.rs, main.rs

My next project likely is: [rayon-walk](../rayon-walk)