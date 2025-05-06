use ethers::types::Address;
use std::{convert, str::FromStr};


trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T : EthereumAddress>(address: T)  -> Address {
    let converted_address : Address = address.convert_address().unwrap();
    converted_address
}


#[cfg(test)]
mod tests { 
    use super::*; 

    #[test]
    fn test_poly() {
        let addr = Address::from_str("0xeBec795c9c8bBD61FFc14A6662944748F299cAcf").unwrap();
        assert_eq!(addr,Address::from_str("0xeBec795c9c8bBD61FFc14A6662944748F299cAcf").unwrap());

        let get_eth_data = get_ethereum_data(addr);
        assert_eq!(get_eth_data, Address::from_str("0xeBec795c9c8bBD61FFc14A6662944748F299cAcf").unwrap());
    }
}