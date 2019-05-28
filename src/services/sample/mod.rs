// This file is generated by lavish: DO NOT EDIT
// https://github.com/fasterthanlime/lavish

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![allow(unused)]

pub use __::*;

mod __ {
    // Notes: as of 2019-05-21, futures-preview is required
    use futures::prelude::*;
    use std::pin::Pin;
    use std::sync::Arc;
    
    use lavish_rpc as rpc;
    use rpc::{Atom, erased_serde, serde_derive::*};
    
    #[derive(Serialize, Debug)]
    #[serde(untagged)]
    #[allow(non_camel_case_types, unused)]
    pub enum Params {
        get_cookies(get_cookies::Params),
        get_user_agent(get_user_agent::Params),
    }
    
    #[derive(Serialize, Debug)]
    #[serde(untagged)]
    #[allow(non_camel_case_types, unused)]
    pub enum Results {
        get_cookies(get_cookies::Results),
        get_user_agent(get_user_agent::Results),
    }
    
    #[derive(Serialize, Debug)]
    #[serde(untagged)]
    #[allow(non_camel_case_types, unused)]
    pub enum NotificationParams {
    }
    
    pub type Message = rpc::Message<Params, NotificationParams, Results>;
    pub type Handle = rpc::Handle<Params, NotificationParams, Results>;
    pub type System = rpc::System<Params, NotificationParams, Results>;
    pub type Protocol = rpc::Protocol<Params, NotificationParams, Results>;
    
    pub fn protocol() -> Protocol {
        Protocol::new()
    }
    
    impl rpc::Atom for Params {
        fn method(&self) -> &'static str {
            match self {
                Params::get_cookies(_) => "get_cookies",
                Params::get_user_agent(_) => "get_user_agent",
            }
        }
        
        fn deserialize(
            method: &str,
            de: &mut erased_serde::Deserializer,
        ) -> erased_serde::Result<Self> {
            use erased_serde::deserialize as deser;
            use serde::de::Error;
            
            match method {
                "get_cookies" =>
                    Ok(Params::get_cookies(deser::<get_cookies::Params>(de)?)),
                "get_user_agent" =>
                    Ok(Params::get_user_agent(deser::<get_user_agent::Params>(de)?)),
                _ => Err(erased_serde::Error::custom(format!(
                    "unknown method: {}",
                    method,
                ))),
            }
        }
    }
    
    impl rpc::Atom for Results {
        fn method(&self) -> &'static str {
            match self {
                Results::get_cookies(_) => "get_cookies",
                Results::get_user_agent(_) => "get_user_agent",
            }
        }
        
        fn deserialize(
            method: &str,
            de: &mut erased_serde::Deserializer,
        ) -> erased_serde::Result<Self> {
            use erased_serde::deserialize as deser;
            use serde::de::Error;
            
            match method {
                "get_cookies" =>
                    Ok(Results::get_cookies(deser::<get_cookies::Results>(de)?)),
                "get_user_agent" =>
                    Ok(Results::get_user_agent(deser::<get_user_agent::Results>(de)?)),
                _ => Err(erased_serde::Error::custom(format!(
                    "unknown method: {}",
                    method,
                ))),
            }
        }
    }
    
    impl rpc::Atom for NotificationParams {
        fn method(&self) -> &'static str {
            match self {
                _ => unimplemented!()
            }
        }
        
        fn deserialize(
            method: &str,
            de: &mut erased_serde::Deserializer,
        ) -> erased_serde::Result<Self> {
            use erased_serde::deserialize as deser;
            use serde::de::Error;
            
            match method {
                _ => Err(erased_serde::Error::custom(format!(
                    "unknown method: {}",
                    method,
                ))),
            }
        }
    }
    
    pub struct Call<T, PP> {
        pub state: Arc<T>,
        pub handle: Handle,
        pub params: PP,
    }
    
    pub type SlotFuture = 
        Future<Output = Result<Results, rpc::Error>> + Send + 'static;
    
    pub type SlotReturn = Pin<Box<SlotFuture>>;
    
    pub type SlotFn<'a, T> = 
        Fn(Arc<T>, Handle, Params) -> SlotReturn + 'a + Send + Sync;
    
    pub type Slot<'a, T> = Option<Box<SlotFn<'a, T>>>;
    
    pub struct Handler<'a, T> {
        state: Arc<T>,
        get_cookies: Slot<'a, T>,
        get_user_agent: Slot<'a, T>,
    }
    
    impl<'a, T> Handler<'a, T> {
        pub fn new(state: T) -> Self {
            Self {
                state: Arc::new(state),
                get_cookies: None,
                get_user_agent: None,
            }
        }
    }
    
    type HandlerRet = Pin<Box<dyn Future<Output = Result<Results, rpc::Error>> + Send + 'static>>;
    
    impl<'a, T> rpc::Handler<Params, NotificationParams, Results, HandlerRet> for Handler<'a, T>
    where
        T: Send + Sync,
    {
        fn handle(&self, handle: Handle, params: Params) -> HandlerRet {
            let method = params.method();
            let slot = match params {
                Params::get_cookies(_) => self.get_cookies.as_ref(),
                Params::get_user_agent(_) => self.get_user_agent.as_ref(),
                _ => None,
            };
            match slot {
                Some(slot_fn) => {
                    let res = slot_fn(self.state.clone(), handle, params);
                    Box::pin(async move { Ok(res.await?) })
                }
                None => Box::pin(async move { Err(rpc::Error::MethodUnimplemented(method)) }),
            }
        }
    }
    
    use lavish_rpc::serde_derive::*;
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Cookie {
        pub key: String,
        pub value: String,
    }
    
    pub mod get_cookies {
        use futures::prelude::*;
        use lavish_rpc::serde_derive::*;
        use super::super::__;
        
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
        }
        
        impl Params {
            pub fn downgrade(p: __::Params) -> Option<Self> {
                match p {
                    __::Params::get_cookies(p) => Some(p),
                    _ => None,
                }
            }
        }
        
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Results {
            pub cookies: Vec<super::Cookie>,
        }
        
        impl Results {
            pub fn downgrade(p: __::Results) -> Option<Self> {
                match p {
                    __::Results::get_cookies(p) => Some(p),
                    _ => None,
                }
            }
        }
        
        pub async fn call(h: &__::Handle, p: ()) -> Result<Results, lavish_rpc::Error> {
            h.call(
                __::Params::get_cookies(Params {}),
                Results::downgrade,
            ).await
        }
        
        pub fn register<'a, T, F, FT>(h: &mut __::Handler<'a, T>, f: F)
        where
            F: Fn(__::Call<T, Params>) -> FT + Sync + Send + 'a,
            FT: Future<Output = Result<Results, lavish_rpc::Error>> + Send + 'static,
        {
            h.get_cookies = Some(Box::new(move |state, handle, params| {
                Box::pin(
                    f(__::Call {
                        state, handle,
                        params: Params::downgrade(params).unwrap(),
                    }).map_ok(__::Results::get_cookies)
                )
            }));
        }
    }
    
    pub mod get_user_agent {
        use futures::prelude::*;
        use lavish_rpc::serde_derive::*;
        use super::super::__;
        
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
        }
        
        impl Params {
            pub fn downgrade(p: __::Params) -> Option<Self> {
                match p {
                    __::Params::get_user_agent(p) => Some(p),
                    _ => None,
                }
            }
        }
        
        #[derive(Serialize, Deserialize, Debug)]
        pub struct Results {
            pub user_agent: String,
        }
        
        impl Results {
            pub fn downgrade(p: __::Results) -> Option<Self> {
                match p {
                    __::Results::get_user_agent(p) => Some(p),
                    _ => None,
                }
            }
        }
        
        pub async fn call(h: &__::Handle, p: ()) -> Result<Results, lavish_rpc::Error> {
            h.call(
                __::Params::get_user_agent(Params {}),
                Results::downgrade,
            ).await
        }
        
        pub fn register<'a, T, F, FT>(h: &mut __::Handler<'a, T>, f: F)
        where
            F: Fn(__::Call<T, Params>) -> FT + Sync + Send + 'a,
            FT: Future<Output = Result<Results, lavish_rpc::Error>> + Send + 'static,
        {
            h.get_user_agent = Some(Box::new(move |state, handle, params| {
                Box::pin(
                    f(__::Call {
                        state, handle,
                        params: Params::downgrade(params).unwrap(),
                    }).map_ok(__::Results::get_user_agent)
                )
            }));
        }
    }
    
    pub fn peer_with_handler<IO, T, F>(io: IO, pool: &futures::executor::ThreadPool, state: T, setup: F) -> Result<Handle, lavish_rpc::Error>
    where
        IO: lavish_rpc::IO,
        T: Send + Sync + 'static,
        F: Fn(&mut Handler<'static, T>),
    {
        let mut handler = Handler::new(state);
        setup(&mut handler);
        Ok(lavish_rpc::System::new(protocol(), handler, io, pool.clone())?.handle())
    }
}
