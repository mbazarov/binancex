use super::hmac::gen_signature_to_slice;
use super::query_string::to_query_string_writer;

use rust_decimal::Decimal;
use serde::ser::Serialize;

fn add_query_separator(this: &mut Url) {
    this.url.push_str("?");
    this.query_start_pos = Some(this.url.len());
    this.add_separator_fn = add_param_separator;
}

fn add_param_separator(this: &mut Url) {
    this.url.push_str("&");
}

// pub trait UrlEncodedParams {
//     fn add_params(&mut self, params: &str);
//     fn add_param_str(&mut self, key: &str, val: &str);
//     fn add_param_integer<I: itoa::Integer>(&mut self, param: &str, val: I);
//     fn add_param_decimal(&mut self, param: &str, val: Decimal);
//     fn add_recv_window(&mut self, recv_window: u16);
//     fn add_timestamp(&mut self);
//     fn add_signature(&mut self, signature: &str);
//     fn len(&self) -> usize;
//     fn as_str(&self) -> &str;
//     fn to_string(&self) -> String;
// }

pub struct Url {
    url: String,
    query_start_pos: Option<usize>,
    add_separator_fn: fn(&mut Self),
}

impl Url {
    pub fn with_capacity(host: &str, path: &str, size: usize) -> Self {
        let mut url = String::with_capacity(size);
        url.push_str(host);
        url.push_str(path);

        Url {
            url,
            query_start_pos: None,
            add_separator_fn: add_query_separator,
        }
    }

    #[inline(always)]
    fn add_separator(&mut self) {
        (self.add_separator_fn)(self);
    }

    #[inline]
    pub fn add_params(&mut self, params: &str) {
        self.add_separator();
        self.url.push_str(params);
    }

    #[inline]
    pub fn add_param_str(&mut self, key: &str, val: &str) {
        self.add_separator();

        self.url.push_str(key);
        self.url.push_str("=");
        self.url.push_str(val);
    }

    #[inline]
    pub fn add_param_integer<I: itoa::Integer>(&mut self, param: &str, val: I) {
        let mut buf = itoa::Buffer::new();
        let s = buf.format(val);

        self.add_param_str(param, s);
    }

    #[inline]
    pub fn add_param_decimal(&mut self, param: &str, val: Decimal) {
        use std::fmt::Write;

        self.add_separator();
        self.url.push_str(param);
        self.url.push_str("=");

        write!(&mut self.url, "{}", val).unwrap();
    }

    #[inline]
    pub fn add_params_from_data<S>(&mut self, data: &S) -> Result<(), serde_qs::Error>
    where
        S: Serialize,
    {
        self.add_separator();
        to_query_string_writer(
            data,
            unsafe {
                self.url.as_mut_vec()
            }
        )
    }

    #[inline]
    pub fn add_recv_window(&mut self, recv_window: u16) {
        if recv_window > 0 {
            self.add_param_integer("recvWindow", recv_window as i32);
        }
    }

    #[inline(always)]
    pub fn add_timestamp(&mut self) {
        self.add_param_integer("timestamp", chrono::Utc::now().timestamp_millis());
    }

    #[inline]
    pub fn add_signature(&mut self, signature: &str) {
        self.url.push_str("&signature=");
        self.url.push_str(signature);
    }

    #[inline]
    pub fn gen_and_add_signature(&mut self, secret_key: &ring::hmac::Key) {
        unsafe {
            let mut buf = [0_u8; 64];
            gen_signature_to_slice(
                self.url[self.query_start_pos.unwrap()..].as_bytes(),
                &mut buf,
                secret_key,
            ).unwrap();
            self.url.push_str("&signature=");
            self.url.push_str(std::str::from_utf8_unchecked(&buf));
        }
    }

    #[inline]
    pub fn get_query(&self) -> Option<&str> {
        if let Some(pos) = self.query_start_pos {
            return Some(&self.url[pos..]);
        }

        None
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.url.len()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.url.capacity()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }

    #[inline(always)]
    pub fn into_string(self) -> String {
        self.url
    }
}

// fn query_set_fn_add_separator(this: &mut UrlQuery) {
//     this.add_separator_fn = query_add_param_separator;
// }
//
// fn query_add_param_separator(this: &mut UrlQuery) {
//     this.query.push_str("&");
// }
//
// pub struct UrlQuery {
//     query: String,
//     add_separator_fn: fn(&mut Self),
// }
//
// impl UrlQuery {
//     pub fn new() -> Self {
//         Self {
//             query: String::new(),
//             add_separator_fn: query_set_fn_add_separator,
//         }
//     }
//
//     pub fn with_capacity(size: usize) -> Self {
//         Self {
//             query: String::with_capacity(size),
//             add_separator_fn: query_set_fn_add_separator,
//         }
//     }
//
//     pub fn from_string(params: String) -> Self {
//         Self {
//             query: params,
//             add_separator_fn: query_add_param_separator,
//         }
//     }
//
//     #[inline(always)]
//     fn add_separator(&mut self) {
//         (self.add_separator_fn)(self);
//     }
//
//     #[inline]
//     pub fn add_params(&mut self, params: &str) {
//         self.add_separator();
//         self.query.push_str(params);
//     }
//
//     #[inline]
//     pub fn add_param_str(&mut self, key: &str, val: &str) {
//         self.add_separator();
//
//         self.query.push_str(key);
//         self.query.push_str("=");
//         self.query.push_str(val);
//     }
//
//     #[inline]
//     pub fn add_param_integer<I: itoa::Integer>(&mut self, param: &str, val: I) {
//         let mut buf = itoa::Buffer::new();
//         let s = buf.format(val);
//
//         self.add_param_str(param, s);
//     }
//
//     #[inline]
//     pub fn add_param_decimal(&mut self, param: &str, val: Decimal) {
//         use std::fmt::Write;
//
//         self.add_separator();
//         self.query.push_str(param);
//         self.query.push_str("=");
//
//         write!(&mut self.query, "{}", val).unwrap();
//     }
//
//     #[inline]
//     pub fn add_recv_window(&mut self, recv_window: u16) {
//         if recv_window > 0 {
//             self.add_param_integer("recvWindow", recv_window as i32);
//         }
//     }
//
//     #[inline(always)]
//     pub fn add_timestamp(&mut self) {
//         self.add_param_integer("timestamp", chrono::Utc::now().timestamp_millis());
//     }
//
//     #[inline]
//     pub fn add_signature(&mut self, signature: &str) {
//         self.query.push_str("&signature=");
//         self.query.push_str(signature);
//     }
//
//     #[inline(always)]
//     pub fn len(&self) -> usize {
//         self.query.len()
//     }
//
//     #[inline(always)]
//     pub fn capacity(&self) -> usize {
//         self.query.capacity()
//     }
//
//     #[inline(always)]
//     pub fn is_empty(&self) -> bool {
//         self.query.is_empty()
//     }
//
//     #[inline(always)]
//     pub fn as_str(&self) -> &str {
//         self.query.as_str()
//     }
//
//     #[inline(always)]
//     pub fn into_string(self) -> String {
//         self.query
//     }
// }

// pub trait ToQuery {
//     fn to_query(&self) -> String;
// }
//
// use binance_schemes::spot::account::NewOrderReq;
//
// impl ToQuery for NewOrderReq {
//     fn to_query(&self) -> String {
//         let mut query = UrlQuery::with_capacity(1024);
//         query.add_param_str("symbol", &self.symbol);
//         query.add_param_str("side", &format!("{:?}", self.side));
//
//         "".to_owned()
//     }
// }
