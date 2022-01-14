use erased_serde::Serialize;

type Params = dyn Serialize + Sync + Send;

pub struct DataBinancePay {
    pub time: i64,
    pub nonce: String,
    pub params: Box<Params>,
}

impl DataBinancePay {
    pub fn params(self) -> impl serde::Serialize + Sync + Send {
        self.params
    }
}

unsafe impl Send for DataBinancePay {}

pub struct DataBinance {
    pub params: Box<Params>,
}

impl DataBinance {
    pub fn params(self) -> impl serde::Serialize + Sync + Send {
        self.params
    }
}

unsafe impl Send for DataBinance {}

pub struct DataKraken {
    pub nonce: u64,
    pub method: String,
    pub params: Box<Params>,
}

impl DataKraken {
    pub fn params(self) -> impl serde::Serialize + Sync + Send {
        self.params
    }
}

unsafe impl Send for DataKraken {}
