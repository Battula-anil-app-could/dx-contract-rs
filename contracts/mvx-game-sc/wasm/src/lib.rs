// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    mvx_game_sc
    (
        init => init
        createGame => create_game
        joinGame => join_game
        claimBackWager => claim_back_wager
        getTokenId => token_id
        getGameStartFee => game_start_fee
        getEnabled => enabled
        getLastGameId => last_game_id
        getGameSettings => game_settings
        getPlayers => players
        getGamesPerUser => games_per_user
        sendReward => send_reward
        enableSC => enable_sc
        disableSC => disable_sc
        setTokenId => set_token_id
        setGameStartFee => set_game_start_fee
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
