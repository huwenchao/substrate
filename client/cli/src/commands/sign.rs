// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Implementation of the `sign` subcommand
use crate::{error, VersionInfo, with_crypto_scheme, pair_from_suri};
use super::{SharedParams, get_password, read_message, read_uri};
use structopt::StructOpt;
use sc_service::{Configuration, ChainSpec};

/// The `sign` command
#[derive(Debug, StructOpt, Clone)]
#[structopt(
	name = "sign",
	about = "Sign a message, with a given (secret) key"
)]
pub struct SignCmd {
	/// The secret key URI.
	/// If the value is a file, the file content is used as URI.
	/// If not given, you will be prompted for the URI.
	#[structopt(long)]
	suri: Option<String>,

	/// Message to sign, if not provided you will be prompted to
	/// pass the message via STDIN
	#[structopt(long)]
	message: Option<String>,

	/// The message on STDIN is hex-encoded data
	#[structopt(long)]
	hex: bool,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,
}


impl SignCmd {
	/// Run the command
	pub fn run(self) -> error::Result<()> {
		let message = read_message(self.message, self.hex)?;
		let suri = read_uri(self.suri)?;
		let password = get_password(&self.shared_params)?;

		let signature = with_crypto_scheme!(
			self.shared_params.scheme,
			sign(
				&suri,
				&password,
				message
			)
		)?;

		println!("{}", signature);
		Ok(())
	}

	/// Update and prepare a `Configuration` with command line parameters
	pub fn update_config<F>(
		&self,
		mut config: &mut Configuration,
		spec_factory: F,
		version: &VersionInfo,
	) -> error::Result<()> where
		F: FnOnce(&str) -> Result<Box<dyn ChainSpec>, String>,
	{
		self.shared_params.update_config(&mut config, spec_factory, version)?;

		Ok(())
	}
}

fn sign<P: sp_core::Pair>(suri: &str, password: &str, message: Vec<u8>) ->  error::Result<String> {
	let pair = pair_from_suri::<P>(suri, password);
	Ok(format!("{}", hex::encode(pair.sign(&message))))
}
