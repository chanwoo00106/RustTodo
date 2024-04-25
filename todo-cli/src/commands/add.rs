use clap::Args;

#[derive(Args, Debug)]
pub struct AddSubCommand {
    pub todo: String,
}
