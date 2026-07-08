/// A description of a side effect. The runtime executes these;
/// `update` only ever returns them.
pub enum Command {
    None,
    FetchUrl(String),
}