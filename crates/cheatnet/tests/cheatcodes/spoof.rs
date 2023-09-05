use crate::{
    assert_success,
    common::{
        call_contract_getter_by_name, deploy_contract, get_contracts, recover_data,
        state::create_cheatnet_state,
    },
};
use cairo_felt::Felt252;
use cheatnet::{
    conversions::{
        class_hash_to_felt, contract_address_to_felt, felt_from_short_string,
        felt_selector_from_name,
    },
    rpc::call_contract,
    CheatnetState,
};
use starknet_api::core::ContractAddress;

#[allow(clippy::too_many_arguments)]
fn assert_all_mock_checker_getters(
    state: &mut CheatnetState,
    contract_address: &ContractAddress,
    expected_version: &[Felt252],
    expected_account_address: &[Felt252],
    expected_max_fee: &[Felt252],
    expected_signature: &[Felt252],
    expected_tx_hash: &[Felt252],
    expected_chain_id: &[Felt252],
    expected_nonce: &[Felt252],
) {
    let tx_hash = call_contract_getter_by_name(state, contract_address, "get_tx_hash");
    assert_success!(tx_hash, expected_tx_hash);
    let nonce = call_contract_getter_by_name(state, contract_address, "get_nonce");
    assert_success!(nonce, expected_nonce);
    let account_address =
        call_contract_getter_by_name(state, contract_address, "get_account_contract_address");
    assert_success!(account_address, expected_account_address);
    let version = call_contract_getter_by_name(state, contract_address, "get_version");
    assert_success!(version, expected_version);
    let chain_id = call_contract_getter_by_name(state, contract_address, "get_chain_id");
    assert_success!(chain_id, expected_chain_id);
    let max_fee = call_contract_getter_by_name(state, contract_address, "get_max_fee");
    assert_success!(max_fee, expected_max_fee);
    let signature = call_contract_getter_by_name(state, contract_address, "get_signature");
    assert_success!(signature, expected_signature);
}

#[allow(clippy::type_complexity)]
fn call_mock_checker_getters(
    state: &mut CheatnetState,
    contract_address: &ContractAddress,
) -> (
    Vec<Felt252>,
    Vec<Felt252>,
    Vec<Felt252>,
    Vec<Felt252>,
    Vec<Felt252>,
    Vec<Felt252>,
    Vec<Felt252>,
) {
    let nonce = call_contract_getter_by_name(state, contract_address, "get_nonce");
    let nonce = recover_data(nonce);
    let account_address =
        call_contract_getter_by_name(state, contract_address, "get_account_contract_address");
    let account_address = recover_data(account_address);
    let version = call_contract_getter_by_name(state, contract_address, "get_version");
    let version = recover_data(version);
    let chain_id = call_contract_getter_by_name(state, contract_address, "get_chain_id");
    let chain_id = recover_data(chain_id);
    let max_fee = call_contract_getter_by_name(state, contract_address, "get_max_fee");
    let max_fee = recover_data(max_fee);
    let signature = call_contract_getter_by_name(state, contract_address, "get_signature");
    let signature = recover_data(signature);
    let tx_hash = call_contract_getter_by_name(state, contract_address, "get_tx_hash");
    let tx_hash = recover_data(tx_hash);

    (
        nonce,
        account_address,
        version,
        chain_id,
        max_fee,
        signature,
        tx_hash,
    )
}

#[test]
fn spoof_simple() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let (
        nonce_before,
        account_address_before,
        version_before,
        chain_id_before,
        max_fee_before,
        signature_before,
        _,
    ) = call_mock_checker_getters(&mut state, &contract_address);

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        contract_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &version_before,
        &account_address_before,
        &max_fee_before,
        &signature_before,
        &[Felt252::from(123)],
        &chain_id_before,
        &nonce_before,
    );
}

#[test]
fn start_spoof_multiple_times() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let (
        nonce_before,
        account_address_before,
        version_before,
        chain_id_before,
        max_fee_before,
        signature_before,
        tx_hash_before,
    ) = call_mock_checker_getters(&mut state, &contract_address);

    let expected_version = Felt252::from(13);
    let expected_account_address = Felt252::from(66);
    let expected_max_fee = Felt252::from(77);
    let expected_signature = vec![Felt252::from(88), Felt252::from(89)];
    let expected_tx_hash = Felt252::from(123);
    let expected_chain_id = Felt252::from(22);
    let expected_nonce = Felt252::from(33);

    state.start_spoof(
        contract_address,
        Some(expected_version.clone()),
        Some(expected_account_address.clone()),
        Some(expected_max_fee.clone()),
        Some(expected_signature.clone()),
        Some(expected_tx_hash.clone()),
        Some(expected_chain_id.clone()),
        Some(expected_nonce.clone()),
    );

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &[expected_version.clone()],
        &[expected_account_address.clone()],
        &[expected_max_fee.clone()],
        &[vec![Felt252::from(2)], expected_signature.clone()].concat(),
        &[expected_tx_hash.clone()],
        &[expected_chain_id.clone()],
        &[expected_nonce.clone()],
    );

    state.start_spoof(
        contract_address,
        None,
        Some(expected_account_address.clone()),
        None,
        Some(expected_signature.clone()),
        None,
        Some(expected_chain_id.clone()),
        None,
    );

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &version_before,
        &[expected_account_address],
        &max_fee_before,
        &[vec![Felt252::from(2)], expected_signature].concat(),
        &tx_hash_before,
        &[expected_chain_id],
        &nonce_before,
    );

    state.start_spoof(
        contract_address,
        Some(expected_version.clone()),
        None,
        Some(expected_max_fee.clone()),
        None,
        Some(expected_tx_hash.clone()),
        None,
        Some(expected_nonce.clone()),
    );

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &[expected_version],
        &account_address_before,
        &[expected_max_fee],
        &signature_before,
        &[expected_tx_hash],
        &chain_id_before,
        &[expected_nonce],
    );
}

#[test]
fn spoof_start_stop() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let (
        nonce_before,
        account_address_before,
        version_before,
        chain_id_before,
        max_fee_before,
        signature_before,
        tx_hash_before,
    ) = call_mock_checker_getters(&mut state, &contract_address);

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        contract_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &version_before,
        &account_address_before,
        &max_fee_before,
        &signature_before,
        &[Felt252::from(123)],
        &chain_id_before,
        &nonce_before,
    );

    state.stop_spoof(contract_address);

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &version_before,
        &account_address_before,
        &max_fee_before,
        &signature_before,
        &tx_hash_before,
        &chain_id_before,
        &nonce_before,
    );
}

#[test]
fn spoof_stop_no_effect() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let (
        nonce_before,
        account_address_before,
        version_before,
        chain_id_before,
        max_fee_before,
        signature_before,
        tx_hash_before,
    ) = call_mock_checker_getters(&mut state, &contract_address);

    state.stop_spoof(contract_address);

    assert_all_mock_checker_getters(
        &mut state,
        &contract_address,
        &version_before,
        &account_address_before,
        &max_fee_before,
        &signature_before,
        &tx_hash_before,
        &chain_id_before,
        &nonce_before,
    );
}

#[test]
fn spoof_with_other_syscall() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        contract_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    let selector = felt_selector_from_name("get_tx_hash_and_emit_event");

    let output =
        call_contract(&contract_address, &selector, vec![].as_slice(), &mut state).unwrap();

    assert_success!(output, vec![Felt252::from(123)]);
}

#[test]
fn spoof_in_constructor() {
    let mut state = create_cheatnet_state();

    let contracts = get_contracts();

    let contract_name = felt_from_short_string("ConstructorSpoofChecker");
    let class_hash = state.declare(&contract_name, &contracts).unwrap();
    let precalculated_address = state.precalculate_address(&class_hash, vec![].as_slice());

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        precalculated_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    let contract_address = state.deploy(&class_hash, &[]).unwrap();

    assert_eq!(precalculated_address, contract_address);

    let selector = felt_selector_from_name("get_stored_tx_hash");

    let output =
        call_contract(&contract_address, &selector, vec![].as_slice(), &mut state).unwrap();

    assert_success!(output, vec![Felt252::from(123)]);
}

#[test]
fn spoof_proxy() {
    let mut state = create_cheatnet_state();

    let contract_address = deploy_contract(&mut state, "SpoofChecker", vec![].as_slice());

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        contract_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    let selector = felt_selector_from_name("get_tx_hash");

    let output =
        call_contract(&contract_address, &selector, vec![].as_slice(), &mut state).unwrap();

    assert_success!(output, vec![Felt252::from(123)]);

    let proxy_address = deploy_contract(&mut state, "SpoofCheckerProxy", vec![].as_slice());
    let proxy_selector = felt_selector_from_name("get_spoof_checkers_tx_hash");
    let output = call_contract(
        &proxy_address,
        &proxy_selector,
        vec![contract_address_to_felt(contract_address)].as_slice(),
        &mut state,
    )
    .unwrap();

    assert_success!(output, vec![Felt252::from(123)]);
}

#[test]
fn spoof_library_call() {
    let mut state = create_cheatnet_state();

    let contracts = get_contracts();
    let contract_name = felt_from_short_string("SpoofChecker");
    let class_hash = state.declare(&contract_name, &contracts).unwrap();

    let lib_call_address = deploy_contract(&mut state, "SpoofCheckerLibCall", vec![].as_slice());

    let version = None;
    let account_contract_address = None;
    let max_fee = None;
    let signature = None;
    let transaction_hash = Some(Felt252::from(123));
    let chain_id = None;
    let nonce = None;

    state.start_spoof(
        lib_call_address,
        version,
        account_contract_address,
        max_fee,
        signature,
        transaction_hash,
        chain_id,
        nonce,
    );

    let lib_call_selector = felt_selector_from_name("get_tx_hash_with_lib_call");
    let output = call_contract(
        &lib_call_address,
        &lib_call_selector,
        vec![class_hash_to_felt(class_hash)].as_slice(),
        &mut state,
    )
    .unwrap();

    assert_success!(output, vec![Felt252::from(123)]);
}