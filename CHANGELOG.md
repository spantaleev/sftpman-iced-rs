# 2025-01-06 (Version 2.0.0)

Initial release.

This application supersedes the [sftman-gtk](https://github.com/spantaleev/sftpman-gtk) GUI frontend (which used to accompany the [sftpman-python](https://github.com/spantaleev/sftpman-python) backend).

Since the `sftpman` backend was rewritten from Python to [Rust](https://www.rust-lang.org/) (see [sftpman-rs](https://github.com/spantaleev/sftpman-rs)), a new GUI frontend was necessary and it's what you see here.

The version intentionally starts from 2.0.0, to clearly distinguish this from the old Python-based software (v1).

`sftpman` v2 (and its `sftpman-iced` GUI) are still **mostly-backward compatible** with the old Python-based `sftpman` versions (v1) with the following exceptions:

- You can now use custom local mount endpoints for filesystems, instead of just the default `/mnt/sshfs/{id}` directory.

- Some CLI commands for `sftpman` have different names (`sftpman setup` being replaced by `sftpman create` and `sftpman update`)

- Installing `sftpman-iced` no longer automatically installs the `sftpman` CLI tool. If you need it, consider installing it separately.