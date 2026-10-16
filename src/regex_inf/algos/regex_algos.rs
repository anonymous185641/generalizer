use clap::ArgEnum;

#[derive(Clone,Debug,Ord,PartialOrd,Eq,PartialEq,Hash,ArgEnum)]
pub enum RegexAlgos {
    Soa2Chare,
    CRX,
    ConMiner,
    GenICHARE,
    Soa2Sore,
    Soa2Soire,
    K_TSSI_Brzozowski,
    K_TSSI_StateRemoval,
    K_TSSI_TransitiveClosure,
}


