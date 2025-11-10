use assert_cmd::prelude::*;
use assert_cmd::cargo::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn lists_non_hidden_in_cwd_sorted() -> Result<(), Box<dyn std::error::Error>> {
    // Fixture
    let tmp = assert_fs::TempDir::new()?;
    tmp.child("a").touch()?;
    tmp.child("b.txt").touch()?;
    tmp.child(".hidden").touch()?;
    let dir = tmp.child("dir");
    dir.create_dir_all()?;
    dir.child("x").touch()?;

    // Run ruls in that dir
    let mut cmd = Command::new(cargo_bin!("ruls"));
    cmd.current_dir(tmp.path());

    cmd.assert()
        .success()
        .stdout("a\nb.txt\ndir\n");

    tmp.close()?;
    Ok(())
}
