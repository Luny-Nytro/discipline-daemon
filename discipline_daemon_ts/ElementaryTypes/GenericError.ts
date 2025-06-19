import * as Displayer from "./Display.ts"
import { Unique } from "./Unique.ts";

export type GenericError = Unique<"App.Elementary.GenericError", {
  currentContext: Context
  readonly previousContexts: Context[]
}>

// export function create(message: string): GenericError {
//   return Unique({
//     message
//   })
// }

// export const displayer = Displayer.implement<GenericError>(me => 
//   Displayer.asWrappedString("GenericError", me.message)
// )

type Context = {
  /// The action that failed
  action: string
  // Error messages caused by the failure of the action specified in
  // "Context.action"
  errors: string[]
  // Additional information about the context in the form of key-value pairs
  attachments: ContextAttachment[]
}

type ContextAttachment = {
  name: string
  info: string
}

export const GenericError = {
  new(action: string): GenericError {
    return Unique({
      currentContext: {
        action,
        errors: [],
        attachments: [],
      },

      previousContexts: [],
    })
  },

  changeContext(me: GenericError, action: string): GenericError {
    me.previousContexts.push(me.currentContext)
    me.currentContext = {
      action,
      errors: [],
      attachments: [],
    }
    return me
  },

  addMessage(me: GenericError, error: string): GenericError {
    me.currentContext.errors.push(error)
    return me
  },

  addNamedAttachment(me: GenericError, name: string, info: string): GenericError {
    me.currentContext.attachments.push({ name, info })
    return me
  },

  asNativeError(me: GenericError): Error {
    throw "NOT IMPLEMENTED"
  }
}

export class GenericError1 {
  private currentContext: Context
  private readonly previousContexts: Context[]

  constructor(action: string) {
    this.currentContext = {
      action,
      errors: [],
      attachments: [],
    };

    this.previousContexts = [];
  }

  addContext(action: string): GenericError1 {
    this.previousContexts.push(this.currentContext)
    this.currentContext = {
      action,
      errors: [],
      attachments: [],
    }

    return this
  }

  addMessage(error: string): GenericError1 {
    this.currentContext.errors.push(error)
    return this
  }

  addNamedAttachment(name: string, info: string): GenericError1 {
    this.currentContext.attachments.push({ name, info })
    return this
  }

  // asNativeError(me: GenericError): Error {
  //   throw "NOT IMPLEMENTED"
  // }
}