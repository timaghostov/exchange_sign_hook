use std::sync::Arc;

use erased_serde::Serialize;

use crate::hook::model::DataBinance;
use crate::hook::model::DataBinancePay;
use crate::hook::model::DataKraken;
use crate::hook::types::AppSign;
use crate::hook::types::AsyncResult;
use crate::hook::types::FnClosureBinance;
use crate::hook::types::FnClosureBinancePay;
use crate::hook::types::FnClosureKraken;
use crate::hook::types::FnResult;

#[derive(Debug)]
enum ExchangeClosure<Lambda> {
    Stub,
    Closure(Lambda),
}

type ECBinancePay = ExchangeClosure<FnClosureBinancePay>;
type ECBinance = ExchangeClosure<FnClosureBinance>;
type ECKraken = ExchangeClosure<FnClosureKraken>;

#[derive(Clone)]
pub struct SignClosure {
    inner: Arc<Inner>,
}

struct Inner {
    app: Box<dyn AppSign>,
    binance_pay: ECBinancePay,
    binance: ECBinance,
    kraken: ECKraken,
}

pub struct SignClosureBuilder {
    app: Box<dyn AppSign>,
    binance_pay: ECBinancePay,
    binance: ECBinance,
    kraken: ECKraken,
}

impl SignClosure {
    pub fn builder(app: Box<dyn AppSign>) -> SignClosureBuilder {
        SignClosureBuilder {
            app,
            binance_pay: ECBinancePay::Stub,
            binance: ECBinance::Stub,
            kraken: ECKraken::Stub,
        }
    }

    pub fn sign_binance_pay<J>(&self, time: i64, nonce: String, params: J) -> FnResult
    where
        J: Serialize + Sync + Send + 'static,
    {
        let data = DataBinancePay {
            time,
            nonce,
            params: Box::new(params),
        };
        let inner = Arc::clone(&self.inner);
        (*inner).sign_binance_pay(data)
    }

    pub fn sign_binance<J>(&self, params: J) -> FnResult
    where
        J: Serialize + Sync + Send + 'static,
    {
        let data = DataBinance {
            params: Box::new(params),
        };
        let inner = Arc::clone(&self.inner);
        (*inner).sign_binance(data)
    }

    pub fn sign_kraken<J>(&self, nonce: u64, method: String, params: J) -> FnResult
    where
        J: Serialize + Sync + Send + 'static,
    {
        let data = DataKraken {
            nonce,
            method,
            params: Box::new(params),
        };
        let inner = Arc::clone(&self.inner);
        (*inner).sign_kraken(data)
    }
}

impl Inner {
    fn sign_binance_pay(&self, data: DataBinancePay) -> FnResult {
        match self.binance_pay {
            ExchangeClosure::Stub => AsyncResult::error("Binance Pay closure missing."),
            ExchangeClosure::Closure(ref closure) => (closure)(self.app.box_clone(), data),
        }
    }

    fn sign_binance(&self, data: DataBinance) -> FnResult {
        match self.binance {
            ExchangeClosure::Stub => AsyncResult::error("Binance closure missing."),
            ExchangeClosure::Closure(ref closure) => (closure)(self.app.box_clone(), data),
        }
    }

    fn sign_kraken(&self, data: DataKraken) -> FnResult {
        match self.kraken {
            ExchangeClosure::Stub => AsyncResult::error("Kraken closure missing."),
            ExchangeClosure::Closure(ref closure) => (closure)(self.app.box_clone(), data),
        }
    }
}

impl SignClosureBuilder {
    pub fn binance_pay(self, closure: FnClosureBinancePay) -> Self {
        Self {
            app: self.app,
            binance_pay: ECBinancePay::Closure(closure),
            binance: self.binance,
            kraken: self.kraken,
        }
    }

    pub fn binance(self, closure: FnClosureBinance) -> Self {
        Self {
            app: self.app,
            binance_pay: self.binance_pay,
            binance: ECBinance::Closure(closure),
            kraken: self.kraken,
        }
    }

    pub fn kraken(self, closure: FnClosureKraken) -> Self {
        Self {
            app: self.app,
            binance_pay: self.binance_pay,
            binance: self.binance,
            kraken: ECKraken::Closure(closure),
        }
    }

    pub fn finish(self) -> SignClosure {
        SignClosure {
            inner: Arc::new(Inner {
                app: self.app,
                binance_pay: self.binance_pay,
                binance: self.binance,
                kraken: self.kraken,
            }),
        }
    }
}
