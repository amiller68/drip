use alloy::sol_types::sol;

sol!(
    #[sol(rpc)]
    Drop,
    "../contracts/out/Drop.sol/Drop.json",
);
