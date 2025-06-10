use alloy::sol;

sol! {
    #[sol(rpc)]
    contract DepositContract {
        #[derive(Debug)]
        struct TokenTransfer {
            address token;
            uint256 amount;
        }

        #[derive(Debug)]
        function deposit(TokenTransfer[] tokenTransfers) public payable;
    }
}
