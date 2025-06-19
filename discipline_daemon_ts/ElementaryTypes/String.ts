import { Tried } from "@Pkg/Tried";
import { GenericError } from "@Pkg/GenericError";

const decoder = new TextDecoder()

export const String = {
  fromUtf8(bytes: Uint8Array): Tried<string, GenericError> {
    throw ""
  },
  fromUnknown(value: unknown): string {
    return ""
  }
}
