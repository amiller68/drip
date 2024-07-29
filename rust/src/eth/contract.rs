use std::error::Error;
use std::sync::Arc;

use alloy::json_abi::JsonAbi;

use alloy::json_abi::{Event, Function, Token};
use alloy::primitives::{Address, Bytes, U256};

use super::EthClient;

pub trait Contract: Send + Sync {
    fn address(&self) -> Address;
    fn abi(&self) -> &JsonAbi;
    fn client(&self) -> &Arc<EthClient>;

    fn encode_function_data(
        &self,
        function: &str,
        args: &[Token],
    ) -> Result<Bytes, Box<dyn Error>> {
        let function = self
            .abi()
            .function(function)
            .ok_or_else(|| Box::<dyn Error>::from(format!("Function {} not found", function)))?;
        let encoded = function
            .encode_input(args)
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?;
        Ok(Bytes::from(encoded))
    }

    fn decode_function_output(
        &self,
        function: &str,
        data: &[u8],
    ) -> Result<Vec<Token>, Box<dyn Error>> {
        let function = self
            .abi()
            .function(function)
            .ok_or_else(|| Box::<dyn Error>::from(format!("Function {} not found", function)))?;
        let decoded = function
            .decode_output(data)
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?;
        Ok(decoded)
    }

    async fn call(&self, function: &str, args: &[Token]) -> Result<Vec<Token>, Box<dyn Error>> {
        let data = self.encode_function_data(function, args)?;
        let result = self.client().call_contract(self.address(), data).await?;
        self.decode_function_output(function, &result)
    }

    async fn send(
        &self,
        function: &str,
        args: &[Token],
        value: U256,
    ) -> Result<Bytes, Box<dyn Error>> {
        let data = self.encode_function_data(function, args)?;
        self.client()
            .send_transaction(self.address(), data, value)
            .await
    }
}
