use clap::Args;

#[derive(Args, Debug)]
pub struct RemoveSubCommand {
    pub todo: String,
}
