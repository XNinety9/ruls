use assert_cmd::prelude::*;
use assert_cmd::cargo::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn lists_hidden_with_a_flag() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = assert_fs::TempDir::new()?;
    tmp.child(".hidden").touch()?;
    tmp.child("a").touch()?;
    tmp.child("dir").create_dir_all()?;

    let mut cmd = Command::new(cargo_bin!("ruls"));
    cmd.current_dir(tmp.path()).args(["-a"]);

    cmd.assert()
        .success()
        .stdout(".hidden\na\ndir\n");

    tmp.close()?;
    Ok(())
}

#[test]
fn reverses_order_with_r_flag() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = assert_fs::TempDir::new()?;
    tmp.child("a").touch()?;
    tmp.child("b.txt").touch()?;
    tmp.child("dir").create_dir_all()?;

    let mut cmd = Command::new(cargo_bin!("ruls"));
    cmd.current_dir(tmp.path()).args(["-r"]);

    cmd.assert()
        .success()
        .stdout("dir\nb.txt\na\n");

    tmp.close()?;
    Ok(())
}

#[test]
fn prints_headers_with_multiple_operands() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = assert_fs::TempDir::new()?;
    let d1 = tmp.child("d1"); d1.create_dir_all()?; d1.child("x").touch()?;
    let d2 = tmp.child("d2"); d2.create_dir_all()?; d2.child("a").touch()?;

    let mut cmd = Command::new(cargo_bin!("ruls"));
    cmd.current_dir(tmp.path()).args(["d1", "d2"]);

    cmd.assert()
        .success()
        .stdout("d1:\nx\n\nd2:\na\n");

    tmp.close()?;
    Ok(())
}
