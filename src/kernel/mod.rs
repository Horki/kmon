pub mod cmd;
pub mod info;
pub mod lkm;
pub mod log;
use clap::ArgMatches;
use info::KernelInfo;
use lkm::{KernelModules, ListArgs};
use log::KernelLogs;
use crate::style::Style;

/* Kernel struct for logs, information and modules */
pub struct Kernel {
	pub logs: KernelLogs,
	pub info: KernelInfo,
	pub modules: KernelModules<'static>,
}

impl Kernel {
	/**
	 * Create a new kernel instance.
	 *
	 * @param  ArgMatches
	 * @return Kernel
	 */
	pub fn new(args: &ArgMatches) -> Self {
		Self {
			logs: KernelLogs::default(),
			info: KernelInfo::new(),
			modules: KernelModules::new(ListArgs::new(args), Style::new(args)),
		}
	}
}
