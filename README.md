Reproducing slowness issues using rust-analyzer 0.2.240 with a diesel project.

- Running `rust-analyzer-linux analysis-stats .` in this repo will hang on processing `do_db_things` for a very long time.
- Making a change to `lib.rs`, e.g. adding some whitespace in `do_db_things` and then saving (with format on save enabled) then leads to hanging on "Saving 'lib.rs': Running 'rust-analyzer' Formatter".
- `rust-analyzer-linux` spawned by the extension ends up in the background using ~100% CPU consistently.

Sometimes manages to load the file initially and provide analysis, but then making changes causes hangs as described aboved. Especially making changes to the schema declarations seems to trigger persistent slowness afterwards.

https://github.com/rust-analyzer/rust-analyzer/issues/5344
https://github.com/rust-analyzer/rust-analyzer/issues/4186
