import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface InitClientParamsFields {}

export interface InitClientParamsJSON {}

export class InitClientParams {
  constructor(fields: InitClientParamsFields) {}

  static layout(property?: string) {
    return borsh.struct([], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new InitClientParams({})
  }

  static toEncodable(fields: InitClientParamsFields) {
    return {}
  }

  toJSON(): InitClientParamsJSON {
    return {}
  }

  static fromJSON(obj: InitClientParamsJSON): InitClientParams {
    return new InitClientParams({})
  }

  toEncodable() {
    return InitClientParams.toEncodable(this)
  }
}
