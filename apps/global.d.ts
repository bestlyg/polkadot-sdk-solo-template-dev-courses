import { Getters } from "@polkadot/api/base/Getters";
import { Init } from "@polkadot/api/base/Init";
import {
  ApiBase,
  ApiDecoration,
  ApiTypes,
  DecoratedErrors,
  DecoratedEvents,
  QueryableCalls,
  QueryableConsts,
  QueryableModuleConsts,
  QueryableStorage,
  QueryableStorageMulti,
} from "@polkadot/api/types";
import { RegistryError } from "@polkadot/types-codec/types/registry";

// declare module "@polkadot/api" {
//   export class ApiPromise extends ApiBase<"promise"> {
//     get consts(): QueryableConsts<"promise"> & {
//       timestamp: QueryableModuleConsts;
//     };
//   }
// }
