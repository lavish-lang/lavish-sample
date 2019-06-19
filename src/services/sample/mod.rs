// This file is generated by lavish: DO NOT EDIT
// https://github.com/fasterthanlime/lavish

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::all, unknown_lints, unused, non_snake_case)]

pub use schema::*;

pub mod protocol {
    #[derive(Debug, ::lavish::serde_derive::Serialize, Clone)]
    #[allow(non_camel_case_types, unused)]
    #[serde(untagged)]
    pub enum Params {
        GetCookies(super::schema::get_cookies::Params),
        Reverse(super::schema::reverse::Params),
        GetUserAgent(super::schema::get_user_agent::Params),
        Ping_Ping(super::schema::ping::ping::Params),
        Ping(super::schema::ping::Params),
        Cookies_Get(super::schema::cookies::get::Params),
        Universe_Earth_Country_City_NewYork(super::schema::universe::earth::country::city::new_york::Params),
        Session_Login_SolveTotp(super::schema::session::login::solve_totp::Params),
        Session_Login(super::schema::session::login::Params),
    }
    impl ::lavish::Atom for Params {
        fn method(&self) -> &'static str {
            match self {
                Params::GetCookies(_) => "get_cookies",
                Params::Reverse(_) => "reverse",
                Params::GetUserAgent(_) => "get_user_agent",
                Params::Ping_Ping(_) => "ping.ping",
                Params::Ping(_) => "ping",
                Params::Cookies_Get(_) => "cookies.get",
                Params::Universe_Earth_Country_City_NewYork(_) => "universe.earth.country.city.new_york",
                Params::Session_Login_SolveTotp(_) => "session.login.solve_totp",
                Params::Session_Login(_) => "session.login",
            }
        }
        fn deserialize(method: &str, de: &mut ::lavish::erased_serde::Deserializer) -> ::lavish::erased_serde::Result<Self> {
            use ::lavish::erased_serde::deserialize as __DS;
            use ::lavish::serde::de::Error;

            match method {
                "get_cookies" => 
                    Ok(Params::GetCookies(__DS::<super::schema::get_cookies::Params>(de)?)),
                "reverse" => 
                    Ok(Params::Reverse(__DS::<super::schema::reverse::Params>(de)?)),
                "get_user_agent" => 
                    Ok(Params::GetUserAgent(__DS::<super::schema::get_user_agent::Params>(de)?)),
                "ping.ping" => 
                    Ok(Params::Ping_Ping(__DS::<super::schema::ping::ping::Params>(de)?)),
                "ping" => 
                    Ok(Params::Ping(__DS::<super::schema::ping::Params>(de)?)),
                "cookies.get" => 
                    Ok(Params::Cookies_Get(__DS::<super::schema::cookies::get::Params>(de)?)),
                "universe.earth.country.city.new_york" => 
                    Ok(Params::Universe_Earth_Country_City_NewYork(__DS::<super::schema::universe::earth::country::city::new_york::Params>(de)?)),
                "session.login.solve_totp" => 
                    Ok(Params::Session_Login_SolveTotp(__DS::<super::schema::session::login::solve_totp::Params>(de)?)),
                "session.login" => 
                    Ok(Params::Session_Login(__DS::<super::schema::session::login::Params>(de)?)),
                _ =>
                    Err(::lavish::erased_serde::Error::custom(format!("unknown method: {}", method))),
            }
        }
    }

    #[derive(Clone, Debug, ::lavish::serde_derive::Serialize)]
    #[allow(non_camel_case_types, unused)]
    #[serde(untagged)]
    pub enum Results {
        GetCookies(super::schema::get_cookies::Results),
        Reverse(super::schema::reverse::Results),
        GetUserAgent(super::schema::get_user_agent::Results),
        Ping_Ping(super::schema::ping::ping::Results),
        Ping(super::schema::ping::Results),
        Cookies_Get(super::schema::cookies::get::Results),
        Universe_Earth_Country_City_NewYork(super::schema::universe::earth::country::city::new_york::Results),
        Session_Login_SolveTotp(super::schema::session::login::solve_totp::Results),
        Session_Login(super::schema::session::login::Results),
    }
    impl ::lavish::Atom for Results {
        fn method(&self) -> &'static str {
            match self {
                Results::GetCookies(_) => "get_cookies",
                Results::Reverse(_) => "reverse",
                Results::GetUserAgent(_) => "get_user_agent",
                Results::Ping_Ping(_) => "ping.ping",
                Results::Ping(_) => "ping",
                Results::Cookies_Get(_) => "cookies.get",
                Results::Universe_Earth_Country_City_NewYork(_) => "universe.earth.country.city.new_york",
                Results::Session_Login_SolveTotp(_) => "session.login.solve_totp",
                Results::Session_Login(_) => "session.login",
            }
        }
        fn deserialize(method: &str, de: &mut ::lavish::erased_serde::Deserializer) -> ::lavish::erased_serde::Result<Self> {
            use ::lavish::erased_serde::deserialize as __DS;
            use ::lavish::serde::de::Error;

            match method {
                "get_cookies" => 
                    Ok(Results::GetCookies(__DS::<super::schema::get_cookies::Results>(de)?)),
                "reverse" => 
                    Ok(Results::Reverse(__DS::<super::schema::reverse::Results>(de)?)),
                "get_user_agent" => 
                    Ok(Results::GetUserAgent(__DS::<super::schema::get_user_agent::Results>(de)?)),
                "ping.ping" => 
                    Ok(Results::Ping_Ping(__DS::<super::schema::ping::ping::Results>(de)?)),
                "ping" => 
                    Ok(Results::Ping(__DS::<super::schema::ping::Results>(de)?)),
                "cookies.get" => 
                    Ok(Results::Cookies_Get(__DS::<super::schema::cookies::get::Results>(de)?)),
                "universe.earth.country.city.new_york" => 
                    Ok(Results::Universe_Earth_Country_City_NewYork(__DS::<super::schema::universe::earth::country::city::new_york::Results>(de)?)),
                "session.login.solve_totp" => 
                    Ok(Results::Session_Login_SolveTotp(__DS::<super::schema::session::login::solve_totp::Results>(de)?)),
                "session.login" => 
                    Ok(Results::Session_Login(__DS::<super::schema::session::login::Results>(de)?)),
                _ =>
                    Err(::lavish::erased_serde::Error::custom(format!("unknown method: {}", method))),
            }
        }
    }

    #[derive(Debug, ::lavish::serde_derive::Serialize, Clone)]
    #[allow(non_camel_case_types, unused)]
    #[serde(untagged)]
    pub enum NotificationParams {}
    impl ::lavish::Atom for NotificationParams {
        fn method(&self) -> &'static str {
            panic!("no variants for NotificationParams")
        }
        fn deserialize(method: &str, de: &mut ::lavish::erased_serde::Deserializer) -> ::lavish::erased_serde::Result<Self> {
            use ::lavish::erased_serde::deserialize as __DS;
            use ::lavish::serde::de::Error;

            match method {
                _ =>
                    Err(::lavish::erased_serde::Error::custom(format!("unknown method: {}", method))),
            }
        }
    }

    pub type Caller = ::lavish::Caller<super::protocol::Params, super::protocol::NotificationParams, super::protocol::Results>;
    pub type Handler<CL> = ::lavish::Handler<CL, super::protocol::Params, super::protocol::NotificationParams, super::protocol::Results>;
    pub trait Callable<R>
    {
        fn upcast_params(self) -> Params;
        fn downcast_results(results: Results) -> Option<R>;
    }

    pub trait Implementable<P>
    {
        fn method() -> &'static str;
        fn downcast_params(params: Params) -> Option<P>;
        fn upcast_results(self) -> Results;
    }

    #[derive(Clone, Copy)]
    pub struct Slottable<P, R>
    where
        R: Implementable<P>,
    {
        pub phantom: std::marker::PhantomData<(P, R)>,
    }

}

pub mod schema {
    #[derive(::lavish::serde_derive::Serialize, ::lavish::serde_derive::Deserialize, Clone, Debug)]
    pub struct Cookie {
        /// The key of the cookie
        pub key: String,
        /// The value of the cookie.
        /// Although it's typed as a string, it can be anything underneath.
        pub value: String,
        /// just adding something
        pub comment: Option<String>,
    }
    pub use get_cookies::method as get_cookies;
    pub mod get_cookies {
        pub fn method() -> super::super::protocol::Slottable<Params, Results> {
            super::super::protocol::Slottable { phantom: std::marker::PhantomData }
        }
        #[derive(Clone, ::lavish::serde_derive::Deserialize, Debug, ::lavish::serde_derive::Serialize)]
        pub struct Params {
        }

        #[derive(::lavish::serde_derive::Serialize, ::lavish::serde_derive::Deserialize, Clone, Debug)]
        pub struct Results {
            pub cookies: Vec<super::Cookie>,
        }

        impl super::super::protocol::Callable<Results> for Params {
            fn upcast_params(self) -> super::super::protocol::Params {
                super::super::protocol::Params::GetCookies(self)
            }
            fn downcast_results(results: super::super::protocol::Results) -> Option<Results> {
                match results {
                    super::super::protocol::Results::GetCookies(r) => Some(r),
                    _ => None,
                }
            }
        }

        impl super::super::protocol::Implementable<Params> for Results {
            fn method() -> &'static str {
                "get_cookies"
            }
            fn upcast_results(self) -> super::super::protocol::Results {
                super::super::protocol::Results::GetCookies(self)
            }
            fn downcast_params(params: super::super::protocol::Params) -> Option<Params> {
                match params {
                    super::super::protocol::Params::GetCookies(p) => Some(p),
                    _ => None,
                }
            }
        }
    }
    pub use reverse::method as reverse;
    pub mod reverse {
        pub fn method() -> super::super::protocol::Slottable<Params, Results> {
            super::super::protocol::Slottable { phantom: std::marker::PhantomData }
        }
        #[derive(::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize, Clone, Debug)]
        pub struct Params {
            pub s: String,
        }

        #[derive(::lavish::serde_derive::Deserialize, Clone, Debug, ::lavish::serde_derive::Serialize)]
        pub struct Results {
            pub s: String,
        }

        impl super::super::protocol::Callable<Results> for Params {
            fn upcast_params(self) -> super::super::protocol::Params {
                super::super::protocol::Params::Reverse(self)
            }
            fn downcast_results(results: super::super::protocol::Results) -> Option<Results> {
                match results {
                    super::super::protocol::Results::Reverse(r) => Some(r),
                    _ => None,
                }
            }
        }

        impl super::super::protocol::Implementable<Params> for Results {
            fn method() -> &'static str {
                "reverse"
            }
            fn upcast_results(self) -> super::super::protocol::Results {
                super::super::protocol::Results::Reverse(self)
            }
            fn downcast_params(params: super::super::protocol::Params) -> Option<Params> {
                match params {
                    super::super::protocol::Params::Reverse(p) => Some(p),
                    _ => None,
                }
            }
        }
    }
    pub use get_user_agent::method as get_user_agent;
    pub mod get_user_agent {
        pub fn method() -> super::super::protocol::Slottable<Params, Results> {
            super::super::protocol::Slottable { phantom: std::marker::PhantomData }
        }
        #[derive(::lavish::serde_derive::Deserialize, Clone, ::lavish::serde_derive::Serialize, Debug)]
        pub struct Params {
        }

        #[derive(Clone, ::lavish::serde_derive::Deserialize, Debug, ::lavish::serde_derive::Serialize)]
        pub struct Results {
            pub user_agent: String,
        }

        impl super::super::protocol::Callable<Results> for Params {
            fn upcast_params(self) -> super::super::protocol::Params {
                super::super::protocol::Params::GetUserAgent(self)
            }
            fn downcast_results(results: super::super::protocol::Results) -> Option<Results> {
                match results {
                    super::super::protocol::Results::GetUserAgent(r) => Some(r),
                    _ => None,
                }
            }
        }

        impl super::super::protocol::Implementable<Params> for Results {
            fn method() -> &'static str {
                "get_user_agent"
            }
            fn upcast_results(self) -> super::super::protocol::Results {
                super::super::protocol::Results::GetUserAgent(self)
            }
            fn downcast_params(params: super::super::protocol::Params) -> Option<Params> {
                match params {
                    super::super::protocol::Params::GetUserAgent(p) => Some(p),
                    _ => None,
                }
            }
        }
    }
    pub use ping::method as ping;
    pub mod ping {
        pub fn method() -> super::super::protocol::Slottable<Params, Results> {
            super::super::protocol::Slottable { phantom: std::marker::PhantomData }
        }
        #[derive(Clone, ::lavish::serde_derive::Serialize, ::lavish::serde_derive::Deserialize, Debug)]
        pub struct Params {
        }

        #[derive(Clone, Debug, ::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize)]
        pub struct Results {
        }

        impl super::super::protocol::Callable<Results> for Params {
            fn upcast_params(self) -> super::super::protocol::Params {
                super::super::protocol::Params::Ping(self)
            }
            fn downcast_results(results: super::super::protocol::Results) -> Option<Results> {
                match results {
                    super::super::protocol::Results::Ping(r) => Some(r),
                    _ => None,
                }
            }
        }

        impl super::super::protocol::Implementable<Params> for Results {
            fn method() -> &'static str {
                "ping"
            }
            fn upcast_results(self) -> super::super::protocol::Results {
                super::super::protocol::Results::Ping(self)
            }
            fn downcast_params(params: super::super::protocol::Params) -> Option<Params> {
                match params {
                    super::super::protocol::Params::Ping(p) => Some(p),
                    _ => None,
                }
            }
        }

        pub use ping::method as ping;
        pub mod ping {
            pub fn method() -> super::super::super::protocol::Slottable<Params, Results> {
                super::super::super::protocol::Slottable { phantom: std::marker::PhantomData }
            }
            #[derive(Clone, ::lavish::serde_derive::Serialize, Debug, ::lavish::serde_derive::Deserialize)]
            pub struct Params {
            }

            #[derive(Debug, Clone, ::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize)]
            pub struct Results {
            }

            impl super::super::super::protocol::Callable<Results> for Params {
                fn upcast_params(self) -> super::super::super::protocol::Params {
                    super::super::super::protocol::Params::Ping_Ping(self)
                }
                fn downcast_results(results: super::super::super::protocol::Results) -> Option<Results> {
                    match results {
                        super::super::super::protocol::Results::Ping_Ping(r) => Some(r),
                        _ => None,
                    }
                }
            }

            impl super::super::super::protocol::Implementable<Params> for Results {
                fn method() -> &'static str {
                    "ping.ping"
                }
                fn upcast_results(self) -> super::super::super::protocol::Results {
                    super::super::super::protocol::Results::Ping_Ping(self)
                }
                fn downcast_params(params: super::super::super::protocol::Params) -> Option<Params> {
                    match params {
                        super::super::super::protocol::Params::Ping_Ping(p) => Some(p),
                        _ => None,
                    }
                }
            }
        }
    }
    pub mod cookies {
        pub use get::method as get;
        pub mod get {
            pub fn method() -> super::super::super::protocol::Slottable<Params, Results> {
                super::super::super::protocol::Slottable { phantom: std::marker::PhantomData }
            }
            #[derive(::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize, Debug, Clone)]
            pub struct Params {
            }

            #[derive(Debug, ::lavish::serde_derive::Serialize, Clone, ::lavish::serde_derive::Deserialize)]
            pub struct Results {
                pub cookies: Vec<super::super::Cookie>,
            }

            impl super::super::super::protocol::Callable<Results> for Params {
                fn upcast_params(self) -> super::super::super::protocol::Params {
                    super::super::super::protocol::Params::Cookies_Get(self)
                }
                fn downcast_results(results: super::super::super::protocol::Results) -> Option<Results> {
                    match results {
                        super::super::super::protocol::Results::Cookies_Get(r) => Some(r),
                        _ => None,
                    }
                }
            }

            impl super::super::super::protocol::Implementable<Params> for Results {
                fn method() -> &'static str {
                    "cookies.get"
                }
                fn upcast_results(self) -> super::super::super::protocol::Results {
                    super::super::super::protocol::Results::Cookies_Get(self)
                }
                fn downcast_params(params: super::super::super::protocol::Params) -> Option<Params> {
                    match params {
                        super::super::super::protocol::Params::Cookies_Get(p) => Some(p),
                        _ => None,
                    }
                }
            }
        }
    }
    pub mod universe {
        pub mod earth {
            pub mod country {
                pub mod city {
                    pub use new_york::method as new_york;
                    pub mod new_york {
                        pub fn method() -> super::super::super::super::super::super::protocol::Slottable<Params, Results> {
                            super::super::super::super::super::super::protocol::Slottable { phantom: std::marker::PhantomData }
                        }
                        #[derive(::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize, Debug, Clone)]
                        pub struct Params {
                        }

                        #[derive(Clone, ::lavish::serde_derive::Deserialize, Debug, ::lavish::serde_derive::Serialize)]
                        pub struct Results {
                        }

                        impl super::super::super::super::super::super::protocol::Callable<Results> for Params {
                            fn upcast_params(self) -> super::super::super::super::super::super::protocol::Params {
                                super::super::super::super::super::super::protocol::Params::Universe_Earth_Country_City_NewYork(self)
                            }
                            fn downcast_results(results: super::super::super::super::super::super::protocol::Results) -> Option<Results> {
                                match results {
                                    super::super::super::super::super::super::protocol::Results::Universe_Earth_Country_City_NewYork(r) => Some(r),
                                    _ => None,
                                }
                            }
                        }

                        impl super::super::super::super::super::super::protocol::Implementable<Params> for Results {
                            fn method() -> &'static str {
                                "universe.earth.country.city.new_york"
                            }
                            fn upcast_results(self) -> super::super::super::super::super::super::protocol::Results {
                                super::super::super::super::super::super::protocol::Results::Universe_Earth_Country_City_NewYork(self)
                            }
                            fn downcast_params(params: super::super::super::super::super::super::protocol::Params) -> Option<Params> {
                                match params {
                                    super::super::super::super::super::super::protocol::Params::Universe_Earth_Country_City_NewYork(p) => Some(p),
                                    _ => None,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod session {
        pub use login::method as login;
        pub mod login {
            pub fn method() -> super::super::super::protocol::Slottable<Params, Results> {
                super::super::super::protocol::Slottable { phantom: std::marker::PhantomData }
            }
            #[derive(Debug, ::lavish::serde_derive::Deserialize, Clone, ::lavish::serde_derive::Serialize)]
            pub struct Params {
                pub username: String,
                pub password: String,
            }

            #[derive(::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize, Clone, Debug)]
            pub struct Results {
            }

            impl super::super::super::protocol::Callable<Results> for Params {
                fn upcast_params(self) -> super::super::super::protocol::Params {
                    super::super::super::protocol::Params::Session_Login(self)
                }
                fn downcast_results(results: super::super::super::protocol::Results) -> Option<Results> {
                    match results {
                        super::super::super::protocol::Results::Session_Login(r) => Some(r),
                        _ => None,
                    }
                }
            }

            impl super::super::super::protocol::Implementable<Params> for Results {
                fn method() -> &'static str {
                    "session.login"
                }
                fn upcast_results(self) -> super::super::super::protocol::Results {
                    super::super::super::protocol::Results::Session_Login(self)
                }
                fn downcast_params(params: super::super::super::protocol::Params) -> Option<Params> {
                    match params {
                        super::super::super::protocol::Params::Session_Login(p) => Some(p),
                        _ => None,
                    }
                }
            }

            pub use solve_totp::method as solve_totp;
            pub mod solve_totp {
                pub fn method() -> super::super::super::super::protocol::Slottable<Params, Results> {
                    super::super::super::super::protocol::Slottable { phantom: std::marker::PhantomData }
                }
                #[derive(Clone, ::lavish::serde_derive::Serialize, ::lavish::serde_derive::Deserialize, Debug)]
                pub struct Params {
                }

                #[derive(::lavish::serde_derive::Deserialize, ::lavish::serde_derive::Serialize, Clone, Debug)]
                pub struct Results {
                    pub result: String,
                }

                impl super::super::super::super::protocol::Callable<Results> for Params {
                    fn upcast_params(self) -> super::super::super::super::protocol::Params {
                        super::super::super::super::protocol::Params::Session_Login_SolveTotp(self)
                    }
                    fn downcast_results(results: super::super::super::super::protocol::Results) -> Option<Results> {
                        match results {
                            super::super::super::super::protocol::Results::Session_Login_SolveTotp(r) => Some(r),
                            _ => None,
                        }
                    }
                }

                impl super::super::super::super::protocol::Implementable<Params> for Results {
                    fn method() -> &'static str {
                        "session.login.solve_totp"
                    }
                    fn upcast_results(self) -> super::super::super::super::protocol::Results {
                        super::super::super::super::protocol::Results::Session_Login_SolveTotp(self)
                    }
                    fn downcast_params(params: super::super::super::super::protocol::Params) -> Option<Params> {
                        match params {
                            super::super::super::super::protocol::Params::Session_Login_SolveTotp(p) => Some(p),
                            _ => None,
                        }
                    }
                }
            }
        }
    }
    pub mod client {
        #[derive(Clone)]
        pub struct Client {
            caller: super::super::protocol::Caller,
        }

        impl Client {
            pub fn new(caller: super::super::protocol::Caller) -> Self {
                Self { caller }
            }
            pub fn call<P, R>(&self, p: P) -> Result<R, ::lavish::Error>
            where
                P: super::super::protocol::Callable<R>,
            {
                self.caller.call(
                    p.upcast_params(),
                    P::downcast_results,
                )
            }
        }
        pub type Runtime = ::lavish::Runtime<Client>;
        pub struct Call<T, P> {
            pub state: ::std::sync::Arc<T>,
            pub client: super::client::Client,
            pub params: P,
        }

        impl<T, P> Call<T, P> {
            fn downcast<PP, F>(self, f: F) -> Result<Call<T, PP>, ::lavish::Error>
            where
                F: Fn(P) -> Option<PP>,
            {
                Ok(Call {
                    state: self.state,
                    client: self.client,
                    params: f(self.params).ok_or_else(|| ::lavish::Error::WrongParams)?,
                })
            }
            pub fn shutdown_runtime(&self) {
                self.client.caller.shutdown_runtime();
            }
        }
        pub type SlotReturn = Result<super::super::protocol::Results, ::lavish::Error>;
        pub type SlotFn<T> = Fn(Call<T, super::super::protocol::Params>) -> SlotReturn + 'static + Send + Sync;
        pub struct Router<T>
        where
            T: Send + Sync + 'static
        {
            state: std::sync::Arc<T>,
            slots: ::std::collections::HashMap<&'static str, Box<SlotFn<T>>>,
        }

        impl<T> Router<T>
        where
            T: Send + Sync + 'static,
        {
            pub fn new(state: ::std::sync::Arc<T>) -> Self {
                Self {
                    state,
                    slots: ::std::collections::HashMap::new(),
                }
            }
            pub fn handle<S, P, R, F>(&mut self, s: S, f: F)
            where
                S: Fn() -> super::super::protocol::Slottable<P, R>,
                R: super::super::protocol::Implementable<P>,
                F: Fn(Call<T, P>) -> Result<R, ::lavish::Error> + 'static + Send + Sync,
            {
                self.slots.insert(R::method(), Box::new(move |call| {
                    let call = call.downcast(R::downcast_params)?;
                    f(call).map(|r| r.upcast_results())
                }));
            }
        }
        impl<T> ::lavish::Handler<Client, super::super::protocol::Params, super::super::protocol::NotificationParams, super::super::protocol::Results> for Router<T>
        where
            T: Send + Sync + 'static,
        {
            fn handle(&self, caller: super::super::protocol::Caller, params: super::super::protocol::Params) -> Result<super::super::protocol::Results, ::lavish::Error> {
                use ::lavish::Atom;
                let slot = self.slots.get(params.method())
                    .ok_or_else(|| ::lavish::Error::MethodUnimplemented(params.method()))?;
                let call = Call {
                    state: self.state.clone(),
                    client: super::client::Client { caller },
                    params,
                };
                slot(call)
            }
            fn make_client(caller: super::super::protocol::Caller) -> Client {
                Client { caller }
            }
        }
    }

    pub mod server {
        #[derive(Clone)]
        pub struct Client {
            caller: super::super::protocol::Caller,
        }

        impl Client {
            pub fn new(caller: super::super::protocol::Caller) -> Self {
                Self { caller }
            }
            pub fn call<P, R>(&self, p: P) -> Result<R, ::lavish::Error>
            where
                P: super::super::protocol::Callable<R>,
            {
                self.caller.call(
                    p.upcast_params(),
                    P::downcast_results,
                )
            }
        }
        pub type Runtime = ::lavish::Runtime<Client>;
        pub struct Call<T, P> {
            pub state: ::std::sync::Arc<T>,
            pub client: super::server::Client,
            pub params: P,
        }

        impl<T, P> Call<T, P> {
            fn downcast<PP, F>(self, f: F) -> Result<Call<T, PP>, ::lavish::Error>
            where
                F: Fn(P) -> Option<PP>,
            {
                Ok(Call {
                    state: self.state,
                    client: self.client,
                    params: f(self.params).ok_or_else(|| ::lavish::Error::WrongParams)?,
                })
            }
            pub fn shutdown_runtime(&self) {
                self.client.caller.shutdown_runtime();
            }
        }
        pub type SlotReturn = Result<super::super::protocol::Results, ::lavish::Error>;
        pub type SlotFn<T> = Fn(Call<T, super::super::protocol::Params>) -> SlotReturn + 'static + Send + Sync;
        pub struct Router<T>
        where
            T: Send + Sync + 'static
        {
            state: std::sync::Arc<T>,
            slots: ::std::collections::HashMap<&'static str, Box<SlotFn<T>>>,
        }

        impl<T> Router<T>
        where
            T: Send + Sync + 'static,
        {
            pub fn new(state: ::std::sync::Arc<T>) -> Self {
                Self {
                    state,
                    slots: ::std::collections::HashMap::new(),
                }
            }
            pub fn handle<S, P, R, F>(&mut self, s: S, f: F)
            where
                S: Fn() -> super::super::protocol::Slottable<P, R>,
                R: super::super::protocol::Implementable<P>,
                F: Fn(Call<T, P>) -> Result<R, ::lavish::Error> + 'static + Send + Sync,
            {
                self.slots.insert(R::method(), Box::new(move |call| {
                    let call = call.downcast(R::downcast_params)?;
                    f(call).map(|r| r.upcast_results())
                }));
            }
        }
        impl<T> ::lavish::Handler<Client, super::super::protocol::Params, super::super::protocol::NotificationParams, super::super::protocol::Results> for Router<T>
        where
            T: Send + Sync + 'static,
        {
            fn handle(&self, caller: super::super::protocol::Caller, params: super::super::protocol::Params) -> Result<super::super::protocol::Results, ::lavish::Error> {
                use ::lavish::Atom;
                let slot = self.slots.get(params.method())
                    .ok_or_else(|| ::lavish::Error::MethodUnimplemented(params.method()))?;
                let call = Call {
                    state: self.state.clone(),
                    client: super::server::Client { caller },
                    params,
                };
                slot(call)
            }
            fn make_client(caller: super::super::protocol::Caller) -> Client {
                Client { caller }
            }
        }
    }

}

