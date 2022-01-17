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

pub enum Query {
    Url(String),
    Params(Box<Params>),
}

pub struct DataBinance {
    pub query: Query,
}

impl DataBinance {
    pub fn query(self) -> Query {
        self.query
    }
}

unsafe impl Send for DataBinance {}

pub struct DataKraken {
    pub nonce: u64,
    pub method: String,
    pub query: Query,
}

impl DataKraken {
    pub fn query(self) -> Query {
        self.query
    }
}

unsafe impl Send for DataKraken {}
