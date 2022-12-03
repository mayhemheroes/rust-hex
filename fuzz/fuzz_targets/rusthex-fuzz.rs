#![no_main]
use libfuzzer_sys::fuzz_target;
use hex::{encode, decode};
use std::str;

fuzz_target!(|data: &[u8]| {
    if data.len() > 2 {
        let opt1 = data[0];
        let opt2 = data[1];
        match opt1 {
            0=>{
                match str::from_utf8(&data[2..]) {
                    Ok(in_string)=>{
                        match opt2 {
                            0=>{
                                hex::decode(in_string);
                            },
                            1=>{
                                hex::encode(in_string);
                            },
                            2=>{
                                hex::encode_upper(in_string);
                            },
                            3=>{
                                let mut out_slice = [0 as u8; 8196 * 2];
                                hex::encode_to_slice(in_string, &mut out_slice);
                            },
                            4=>{
                                let mut out_slice = [0 as u8; 8196 * 2];
                                hex::decode_to_slice(in_string, &mut out_slice);
                            },
                            _=>()
                        }
                        
                    },
                    Err(..)=>()
                }
            },
            1=>{
                hex::encode(&data[2..]);
            },
            2=>{
                let mut out_slice = [0 as u8; 8196 * 2];
                hex::encode_to_slice(&data[2..], &mut out_slice);
            },
            _=>()
        }
        
    }
});
