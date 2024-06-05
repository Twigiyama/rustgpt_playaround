
use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}
impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
       match Address::from_str(self) {
           Ok(address) => Ok(address),
           Err(_) => Err("Invalid Ethereum Address")
       }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let convert_address: Address = address.convert_address().unwrap();
    convert_address
}

#[cfg(test)]
mod test {
    use super::*;
}

#[test]
fn tests_poly() {
    let addr = Address::from_str("0x0816c11B230e15a0E844a111775AF6ea0EaCaaf2").unwrap();
    let new_addr: Address = get_ethereum_data(addr);
    let new_addr2: Address = get_ethereum_data( "0x0816c11B230e15a0E844a111775AF6ea0EaCaaf2");
    assert_eq!(new_addr, addr);
    assert_eq!(new_addr2, addr);
}