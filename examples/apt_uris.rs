use apt_cmd::AptGet;

fn main() -> anyhow::Result<()> {
    futures::executor::block_on(async move {
        for package in AptGet::new().noninteractive().upgrade_uris().await?? {
            println!("{:?}", package);
        }

        Ok(())
    })
}