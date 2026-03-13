use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "connSim")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new connection
    Create {
        /// specify the id for this connection
        id: String,
        /// specify the address/link for this connection
        address: String,
    },
    Activate {
        /// specify the id of the connection you want to activate
        id: String,
        /// specify the address/link for the connection you want to activate
        address: String,
    },
    Connect {
        /// specify the id of the connection you want to start connecting to.
        id: String,
        /// specify the address/link for the connection you want to start connecting to.
        address: String,
    },
    Succeed {
        /// specify the id of the connection you want to successfully connect to.
        id: String,
        /// specify the address/link for the connection you want to successfully connect to.
        address: String,
    },
    Fail {
        /// specify the id of the connection you want to fail connection
        id: String,
        /// specify the address/link for the connection you want to fail connection
        address: String,
    },
    Disconnect {
        /// specify the id of the connection you want to disconnect
        id: String,
        /// specify the address/link for the connection you want to disconnect
        address: String,
    },
    Inspect {
        /// specify the id of the connection you want to print summary
        id: String,
        /// specify the address/link for the connection you want to print summary
        address: String,
    },
    List,
}
