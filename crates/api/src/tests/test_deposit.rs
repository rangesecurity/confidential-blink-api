use {
    super::get_user_ata, crate::tests::BlinkTestClient, common::test_helpers::test_key,
    solana_client::nonblocking::rpc_client::RpcClient, solana_sdk::signature::Keypair,
    std::sync::Arc,
};

#[tokio::test(flavor = "multi_thread")]
async fn test_deposit() {
    let key = test_key();
    let mint = Keypair::new();
    let rpc = Arc::new(RpcClient::new("http://localhost:8899".to_string()));

    let mut test_client = BlinkTestClient::new(rpc).await;

    test_client.create_confidential_mint(&key, &mint).await;

    test_client.test_initialize(&key, &mint).await;

    test_client.mint_tokens(&key, &mint, 1_000_000).await;

    assert_eq!(
        test_client
            .rpc
            .get_token_account_balance(&get_user_ata(&key, &mint))
            .await
            .unwrap()
            .amount
            .parse::<u64>()
            .unwrap(),
        1_000_000
    );

    test_client.test_deposit(&key, &mint, 100).await;
    test_client.test_apply(&key, &mint).await;
    test_client.test_deposit(&key, &mint, 10).await;

    let balances = test_client.get_balances(&key, &mint).await;
    assert_eq!(balances.pending_balance, 0.00001);
    assert_eq!(balances.available_balnace, 0.0001);
    assert_eq!(balances.non_confidential_balance, 0.99989);
}
