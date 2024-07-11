import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { Getters } from "@polkadot/api/base/Getters";
import { Init } from "@polkadot/api/base/Init";
import {
  ApiDecoration,
  ApiTypes,
  DecoratedErrors,
  DecoratedEvents,
  QueryableCalls,
  QueryableConsts,
  QueryableModuleConsts,
  QueryableStorage,
  QueryableStorageMulti,
  ApiBase,
} from "@polkadot/api/types";
import { RegistryError } from "@polkadot/types-codec/types/registry";

const sleep = (t: number) => new Promise((r) => setTimeout(r, t));

const localURL = "ws://127.0.0.1:9944";
const addr1 = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const addr2 = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

async function main() {
  const wsProvider = new WsProvider(localURL);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {},
  });
  await api.isReady;


  // const keyring = new Keyring({ type: "sr25519" });
  // const alice = keyring.addFromUri("//Alice");
  // const bob = keyring.addFromUri("//Bob");

  // const tx = await api.tx.balances.transferKeepAlive(addr2, '')
  // const hash = await tx.signAndSend(alice);
  // console.log(hash.toHex());

  // const tx = api.tx.balances.transferKeepAlive(addr2, 12345);
  // const hash = await tx.signAndSend(alice);
  // console.log(hash.toHex());

  // const tx = api.tx.templateModule.doSomething(10000);
  // const hash = await tx.signAndSend(alice);
  // console.log(hash.toHex());

  // api.rpc.chain.subscribeNewHeads((header) => {
  //   console.log(header.toHuman());
  // });

  // api.query.system.account(addr1, (account) => {
  //   console.log(account.nonce.toHuman(), account.data.free.toHuman());
  // });

  // await sleep(20000);

  // const res = await api.call.accountNonceApi.accountNonce(addr1);
  // console.log(res.toHuman());
  const header = await api.rpc.chain.getHeader();
}

main().then(
  (res) => {
    console.log("====Success");
    console.log(res);
    // process.exit(0);
  },
  (err) => {
    console.log("====Error");
    console.error(err);
    process.exit(1);
  }
);
