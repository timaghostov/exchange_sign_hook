use std::any::Any;
use std::future::Future;
use std::pin::Pin;

use crate::hook::error::SignError;
use crate::hook::error::SignResult;
use crate::hook::model::DataBinance;
use crate::hook::model::DataBinancePay;
use crate::hook::model::DataKraken;

type IHookAny = dyn Any + Sync + Send;

pub trait AppSign: Any + Sync + Send {
    fn box_clone(&self) -> Box<dyn AppSign>;

    fn clone_as_any(&self) -> Box<IHookAny>;
}

impl<T> AppSign for T
where
    T: Any + Sync + Send + Clone,
{
    fn box_clone(&self) -> Box<dyn AppSign> {
        Box::new(self.clone())
    }

    fn clone_as_any(&self) -> Box<IHookAny> {
        Box::new(self.clone())
    }
}

pub type FnResult = Pin<Box<dyn Future<Output = SignResult> + Send + 'static>>;
type FnClosure<Data> = Box<dyn Fn(Box<dyn AppSign>, Data) -> FnResult + Send + Sync>;

pub type FnClosureBinancePay = FnClosure<DataBinancePay>;
pub type FnClosureBinance = FnClosure<DataBinance>;
pub type FnClosureKraken = FnClosure<DataKraken>;

pub struct AsyncResult;

impl AsyncResult {
    pub fn error(s: impl Into<String>) -> FnResult {
        let s = s.into();
        Box::pin(async move { SignResult::Err(SignError::other(s)) })
    }
}
