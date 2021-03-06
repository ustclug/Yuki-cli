use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use std::io;

pub(crate) struct CtLogs;

impl Commander for CtLogs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("logs")
            .about("Container logs")
            .arg(
                Arg::with_name("follow")
                    .short("f")
                    .long("follow")
                    .help("Follow log output"),
            )
            .arg(
                Arg::with_name("tail")
                    .short("t")
                    .long("tail")
                    .takes_value(true)
                    .help("Number of lines to show from the end of the logs"),
            )
            .arg(Arg::with_name("NAME").help("Repository name"))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut remote = ctx.remote.join("containers")?;
        let name = args.value_of("NAME").unwrap();
        remote.path_segments_mut().unwrap().push(name).push("logs");
        let mut req = ctx.get(remote);
        if args.is_present("follow") {
            req.query(&[("follow", "1")]);
        }
        if args.is_present("tail") {
            value_t_or_exit!(args, "tail", u32);
            req.query(&[("tail", args.value_of("tail").unwrap())]);
        }
        let mut r = req.send()?;
        exit_on_error!(r);
        r.copy_to(&mut io::stdout())?;
        Ok(())
    }
}
