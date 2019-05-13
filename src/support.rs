use lavish_rpc::Atom;
use std::marker::PhantomData;

use bytes::*;
use futures::{Async, Poll, Stream};
use tokio_io::AsyncRead;

#[must_use = "futures do nothing unless polled"]
pub struct RpcSystem<P, NP, R, Read>
where
    P: Atom,
    NP: Atom,
    R: Atom,
    Read: AsyncRead,
{
    read: Read,
    _p: PhantomData<P>,
    _np: PhantomData<NP>,
    _r: PhantomData<R>,
}

impl<P, NP, R, Read> RpcSystem<P, NP, R, Read>
where
    P: Atom,
    NP: Atom,
    R: Atom,
    Read: AsyncRead,
{
    pub fn new(read: Read) -> Self {
        Self {
            _p: PhantomData,
            _np: PhantomData,
            _r: PhantomData,
            read,
        }
    }
}

impl<P, NP, R, Read> Stream for RpcSystem<P, NP, R, Read>
where
    P: Atom,
    NP: Atom,
    R: Atom,
    Read: AsyncRead,
{
    type Item = ();
    type Error = String;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut buf = BytesMut::with_capacity(1024);
        buf.resize(1024, 0);
        match self.read.poll_read(&mut buf) {
            Ok(Async::Ready(n)) => {
                let s = String::from_utf8_lossy(&buf[..n]);
                println!("[rpc] was ready, read {} bytes: {:?}", n, s);
                Ok(Async::Ready(Some(())))
            }
            Ok(Async::NotReady) => {
                println!("[rpc] was not ready!");
                Ok(Async::NotReady)
            }
            Err(e) => Err(format!("underlying error: {:?}", e)),
        }
    }
}
