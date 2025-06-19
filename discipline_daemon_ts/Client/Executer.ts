import * as Displayer from "../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as Error from "./ExecuteError.ts"
import * as Client from "./Client.ts"
import { Err, isErr, Ok, Tried } from "../ElementaryTypes/Tried.ts"
import { Unique } from "../ElementaryTypes/Unique.ts";

export type Executer<
  OperationCreatorArguments extends unknown[],
  OperationOutcome,
> = Unique<"App.Client.Executer", {
  readonly execute: (
    client: Client.Client, 
    ...createOperationArgs: OperationCreatorArguments
  ) => Promise<Tried<OperationOutcome, Error.T>>

  // TODO: Should we call this "executeReturnDisplayer"
  //       so we use the current name for a displayer that displays 
  //       this executer instance?
  readonly displayer: Displayer.Inspector<Tried<OperationOutcome, Error.T>>
}>

export function implement<
  Operation, 
  OperationOutcome,
  OperationCreatorArguments extends unknown[]
>(
  path: string[],
  
  createOperation: (...args: OperationCreatorArguments) => Operation,
  
  operationDisplayer: Displayer.Inspector<Operation>,
  operationJsonSerializer: JsonSerializer.JsonSerializer<Operation>,
  operationJsonDeserializer: JsonDeserializer.JsonDeserializer<Operation>,

  operationOutcomeDisplayer: Displayer.Inspector<OperationOutcome>,
  operationOutcomeJsonSerializer: JsonSerializer.JsonSerializer<OperationOutcome>,
  operationOutcomeJsonDeserializer: JsonDeserializer.JsonDeserializer<OperationOutcome>,
): 
  Executer<OperationCreatorArguments, OperationOutcome> 
{
  const displayer = Tried.Displayer(
    operationOutcomeDisplayer,
    Error.displayer,
  )

  function execute(
    client: Client.Client, 
    ...operationCreatorArguments: OperationCreatorArguments
  ): 
    Promise<Tried<OperationOutcome, Error.T>> 
  {
    return executeImpl(
      client, 
      path, 
      createOperation(...operationCreatorArguments), 
      operationJsonSerializer,
      operationOutcomeJsonDeserializer,
    )
  }

  return Unique({
    displayer,
    execute,
  })
}

async function executeImpl<Operation, OperationOutcome>(
  client: Client.Client,
  path: string[],
  operation: Operation,
  operationJsonSerializer: JsonSerializer.JsonSerializer<Operation>,
  operationOutcomeJsonDeserializer: JsonDeserializer.JsonDeserializer<OperationOutcome>,
): 
  Promise<Tried<OperationOutcome, Error.T>> 
{
  const maybeOperationSerialized = operationJsonSerializer.serialize(operation)
  if (isErr(maybeOperationSerialized)) {
    return Err(Error.SerializeOperation(
      Tried.error(maybeOperationSerialized),
    ))
  }

  const operationSerialized = Tried.value(maybeOperationSerialized)

  let response: Response

  try {
    response = await fetch(`http://${client.host}:${client.port}/${path.join("/")}`, {
      body: operationSerialized,
      method: "POST",
      headers: {
        "Content-Length": operationSerialized.length.toString(),
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "*",
        "Access-Control-Allow-Headers": "*"
      },
    })
  } catch (error) {
    return Err(Error.Fetch(error))
  }

  if (!response.ok) {
    return Err(Error.BadResponse(response.status, response.statusText))
  }

  let outcomeAsJson: string
  try {
    outcomeAsJson = await response.text()
  } catch (error) {
    return Err(Error.Fetch(error))
  }

  const maybeOutcome = operationOutcomeJsonDeserializer.deserialize(outcomeAsJson)
  if (isErr(maybeOutcome)) {
    return Err(Error.DeserializeOperationOutcome(
      Tried.error(maybeOutcome),
      outcomeAsJson,
    ))
  }

  return Ok(Tried.value(maybeOutcome))
}
