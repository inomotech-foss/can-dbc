use std::io;
use std::io::prelude::*;
use std::{
    collections::HashSet,
    fs::{self, File},
    path::PathBuf,
};

const MAX_REST_LEN: usize = 2048;

#[test]
fn main() -> io::Result<()> {
    // List of currently failing tests that should be fixed in the future
    let ignored_entries: HashSet<PathBuf> = [
        "./tests/cantools-dbcs/CamelCaseEmpty.dbc",
        "./tests/cantools-dbcs/emc32.dbc",
        "./tests/cantools-dbcs/empty_ns.dbc",
        "./tests/cantools-dbcs/issue_199_extended.dbc",
        "./tests/cantools-dbcs/issue_199.dbc",
        "./tests/cantools-dbcs/issue_228.dbc",
        "./tests/cantools-dbcs/message-dlc-zero.dbc",
        "./tests/cantools-dbcs/no_signals.dbc",
        "./tests/cantools-dbcs/socialledge.dbc",
    ]
    .iter()
    .map(PathBuf::from)
    .collect();

    let entries = fs::read_dir("./tests/cantools-dbcs")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for dbc_path in entries {
        println!("Next: {:?}", dbc_path);
        if ignored_entries.contains(&dbc_path) {
            println!("Ignoring: {:?}", dbc_path);
            continue;
        }

        let mut f = File::open(&dbc_path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        if let Err(err) = can_dbc::DBC::from_slice(&buffer) {
            match err {
                can_dbc::Error::Incomplete(_, rest) => {
                    let mut truncated_rest = rest.chars().take(MAX_REST_LEN).collect::<String>();
                    if truncated_rest.len() < rest.len() {
                        use std::fmt::Write;
                        write!(&mut truncated_rest, "... (truncated from {} bytes)", rest.len()).unwrap();
                    }
                    panic!("Unable to parse full DBC file, remainder: {:?}", truncated_rest);
                }
                other => panic!("Failed to parse DBC file {:?}: {:#?}", dbc_path, other),
            }
        }
    }

    Ok(())
}
