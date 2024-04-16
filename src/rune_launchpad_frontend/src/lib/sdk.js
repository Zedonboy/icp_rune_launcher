import { createActor, canisterId } from '../../../declarations/rune_launchpad_backend';
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from '@dfinity/agent';
import { HOST, identityProvider } from './config';

export async function logout () {
    let auth = await AuthClient.create()
    auth.logout()
}

export async function authenticate(successFn) {
    let auth = await AuthClient.create()

    await auth.login({
        identityProvider: identityProvider,
        maxTimeToLive: BigInt(3_600_000_000_000),
        onSuccess: successFn
    })

}

export async function user_authenticated() {
    let client = await AuthClient.create()
    return await client.isAuthenticated()
}

export async function get_btc_deposit_address() {
    if(!(await user_authenticated())) {
        throw new Error("You must authenticated")
    }

    let auth = await AuthClient.create()

    let backend_actor = get_actor(auth.getIdentity())
    return await backend_actor.get_p2pkh_address()
}

export async function create_rune(symbol, amount, decimals) {
    if(!(await user_authenticated())) {
        throw new Error("You must authenticated")
    }

    let auth = await AuthClient.create()
    let backend_actor = get_actor(auth.getIdentity())
    return await backend_actor.etch_rune({
        symbol,
        amount,
        decimals
    })
}

export async function withdraw_btc(address, amount) {
    if(!(await user_authenticated())) {
        throw new Error("You must authenticated")
    }

    let auth = await AuthClient.create()
    let backend_actor = get_actor(auth.getIdentity())
    return await backend_actor.withdraw_btc(address, BigInt(amount))

}


function get_actor(identity, id) {
    let agent = new HttpAgent({
        identity: identity,
        host: HOST,
    })
    return createActor( id || canisterId, {
        agent
    })

}