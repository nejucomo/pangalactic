use std::io::Read;
use pangalactic_store::ReadVerify;

#[derive(derive_more::From)]
pub struct Reader<R>(R);

impl<R: Read> for Reader<R> {

}
