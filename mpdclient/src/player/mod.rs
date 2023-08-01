use crate::raw::RawMpdClient;

pub struct Player<'cl> {
    pub(crate) inner: &'cl mut RawMpdClient,
}

