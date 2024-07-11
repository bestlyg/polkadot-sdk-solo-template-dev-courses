import { ApiPromise as BaseApiPromise, WsProvider } from "@polkadot/api";

export const ApiPromise = BaseApiPromise as typeof BaseApiPromise & {
  get consts(): string;
};
