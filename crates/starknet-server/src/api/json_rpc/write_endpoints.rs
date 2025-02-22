use starknet_types::rpc::transactions::broadcasted_deploy_account_transaction::BroadcastedDeployAccountTransaction;
use starknet_types::rpc::transactions::broadcasted_invoke_transaction::BroadcastedInvokeTransaction;
use starknet_types::rpc::transactions::BroadcastedDeclareTransaction;

use super::error::ApiError;
use super::models::{
    DeclareTransactionOutput, DeployAccountTransactionOutput, InvokeTransactionOutput,
};
use crate::api::json_rpc::error::RpcResult;
use crate::api::json_rpc::JsonRpcHandler;

impl JsonRpcHandler {
    pub(crate) async fn add_declare_transaction(
        &self,
        request: BroadcastedDeclareTransaction,
    ) -> RpcResult<DeclareTransactionOutput> {
        let (transaction_hash, class_hash) = match request {
            BroadcastedDeclareTransaction::V1(broadcasted_declare_txn) => self
                .api
                .starknet
                .write()
                .await
                .add_declare_transaction_v1(*broadcasted_declare_txn)?,
            BroadcastedDeclareTransaction::V2(broadcasted_declare_txn) => self
                .api
                .starknet
                .write()
                .await
                .add_declare_transaction_v2(*broadcasted_declare_txn)?,
        };

        Ok(DeclareTransactionOutput { transaction_hash, class_hash })
    }

    pub(crate) async fn add_deploy_account_transaction(
        &self,
        request: BroadcastedDeployAccountTransaction,
    ) -> RpcResult<DeployAccountTransactionOutput> {
        let (transaction_hash, contract_address) =
            self.api.starknet.write().await.add_deploy_account_transaction(request).map_err(
                |err| match err {
                    starknet_core::error::Error::StateError(
                        starknet_core::error::StateError::NoneClassHash(_),
                    ) => ApiError::ClassHashNotFound,
                    unknown_error => ApiError::StarknetDevnetError(unknown_error),
                },
            )?;

        Ok(DeployAccountTransactionOutput { transaction_hash, contract_address })
    }

    pub(crate) async fn add_invoke_transaction(
        &self,
        request: BroadcastedInvokeTransaction,
    ) -> RpcResult<InvokeTransactionOutput> {
        let transaction_hash = self.api.starknet.write().await.add_invoke_transaction(request)?;

        Ok(InvokeTransactionOutput { transaction_hash })
    }
}

#[cfg(test)]
mod tests {
    use starknet_types::rpc::transactions::broadcasted_declare_transaction_v1::BroadcastedDeclareTransactionV1;
    use starknet_types::rpc::transactions::broadcasted_deploy_account_transaction::BroadcastedDeployAccountTransaction;

    #[test]
    fn check_correct_deserialization_of_deploy_account_transaction_request() {
        test_deploy_account_transaction();
    }

    /// The example uses declare_v1.json from test_data/rpc/declare_v1.json
    /// Which declares the example from https://www.cairo-lang.org/docs/hello_starknet/intro.html#your-first-contract
    /// The example was compiled locally and send via Postman to https://alpha4.starknet.io/gateway/add_transaction
    #[test]
    fn parsed_base64_gzipped_json_contract_class_correctly() {
        let json_string = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/rpc/declare_v1.json"
        ))
        .unwrap();

        let _broadcasted_declare_transaction_v1: BroadcastedDeclareTransactionV1 =
            serde_json::from_str(&json_string).unwrap();
    }

    fn test_deploy_account_transaction() -> BroadcastedDeployAccountTransaction {
        let json_string = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/rpc/deploy_account.json"
        ))
        .unwrap();

        let broadcasted_deploy_account_transaction: BroadcastedDeployAccountTransaction =
            serde_json::from_str(&json_string).unwrap();

        broadcasted_deploy_account_transaction
    }
}
