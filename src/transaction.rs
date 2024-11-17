// use std::fmt::format;
// use clap::builder::Str;
//
// #[derive(Seriailize, Deserialize, Debug, Clone)]
// pub struct Transactions {
//     pub id: String,
//     pub vin: Vec<TXInput>,
//     pub vout: Vec<TXOutput>,
//
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct TXInput {
//     pub txid: String,
//     pub vout: i32,
//     pub script_sig: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct TXOutput {
//     pub value: i32,
//     pub script_pub_key: String,
// }
//
// impl Transaction {
//     pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
//         if data == String::from("") {
//             data += &format("Reward to '{}'", to);
//         }
//         let mut tx = Transaction {
//             id: String::new(),
//             vin: vec![TXInput {
//                 txid: String::new(),
//                 vout: -1,
//                 script_sig: data,
//             }],
//             vout: vec![TXOutput {
//                 value: 100,
//                 script_pub_key: to,
//             }],
//
//         }
//     }
// }