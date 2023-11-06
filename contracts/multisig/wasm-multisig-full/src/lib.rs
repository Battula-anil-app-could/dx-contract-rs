// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           28
// Async Callback:                       1
// Total number of exported functions:  30

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    multisig
    (
        init => init
        deposit => deposit
        signed => signed
        sign => sign
        unsign => unsign
        discardAction => discard_action
        getQuorum => quorum
        getNumBoardMembers => num_board_members
        getNumProposers => num_proposers
        getActionLastIndex => get_action_last_index
        proposeAddBoardMember => propose_add_board_member
        proposeAddProposer => propose_add_proposer
        proposeRemoveUser => propose_remove_user
        proposeChangeQuorum => propose_change_quorum
        proposeTransferExecute => propose_transfer_execute
        proposeAsyncCall => propose_async_call
        proposeSCDeployFromSource => propose_sc_deploy_from_source
        proposeSCUpgradeFromSource => propose_sc_upgrade_from_source
        quorumReached => quorum_reached
        performAction => perform_action_endpoint
        dnsRegister => dns_register
        getPendingActionFullInfo => get_pending_action_full_info
        userRole => user_role
        getAllBoardMembers => get_all_board_members
        getAllProposers => get_all_proposers
        getActionData => get_action_data
        getActionSigners => get_action_signers
        getActionSignerCount => get_action_signer_count
        getActionValidSignerCount => get_action_valid_signer_count
    )
}

dharitri_sc_wasm_adapter::async_callback! { multisig }
