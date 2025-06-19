import * as Path from "https://deno.land/std@0.210.0/path/mod.ts"
import { Ok } from "Pkg";
import { Err } from "Pkg";

function serialize(value: unknown) {
  return JSON.stringify(value)
}

function deserialize<T>(string: string) {
  return JSON.parse(string) as any as T
}

const encoder = new TextEncoder()
const decoder = new TextDecoder()

function dir(path: string) {
  const index = path.lastIndexOf(Path.SEP)
  return index !== 0
    ? path.slice(0, index)
    : path
}

async function writeBinaryFile(path: string, data: Uint8Array) {
  try {
    await Deno.writeFile(path, data)
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      await Deno.mkdir(dir(path), { recursive: true })
      await Deno.writeFile(path, data)
    } else {
      throw error
    }  
  }
}

async function writeUTF8File(path: string, data: string) {
  await writeBinaryFile(path, encoder.encode(data))
}

async function writeJSONFile(path: string, data: unknown) {
  await writeUTF8File(path, JSON.stringify(data))
}

async function readBinaryFile(path: string, fallback: () => Uint8Array) {
  try {
    return await Deno.readFile(path)
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      const data = fallback()
      await writeBinaryFile(path, data)
      return data
    }
    
    throw error
  }
}

async function readUTF8File(path: string, fallback: () => string) {
  return decoder.decode(await readBinaryFile(path, () => encoder.encode(fallback())))
}

async function readJSONFile(path: string, fallback: () => unknown) {
  return deserialize(await readUTF8File(path, () => serialize(fallback())))
}

export async function read<T>(path: string, fallback: () => T) {
  try {
    return new Ok((await readJSONFile(path, fallback)) as any as T)
  } catch (error) {
    return new Err(new UnknownError(error))
  }
}

export async function save<T>(path: string, data: T) {
  try {
    return new Ok((await writeJSONFile(path, data)) as T)
  } catch (error) {
    return new Err(new UnknownError(error))
  }
}

export class UnknownError {
  constructor(readonly error: unknown) {}
}