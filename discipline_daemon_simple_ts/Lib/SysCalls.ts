import { Mutex } from "./Mutex.ts"
import { Err, Ok, Tried } from "../Prelude.ts";

const encoder = new TextEncoder()
const decoder = new TextDecoder()

const mutex = new Mutex();

// export async function logoutUser(username: string) {
  
// }

// export async function changeUserPassword(username: string, newPassword: string) {
//   return Err(null) as Tried<null, null>
// }


export async function logoutUser(username: string) {
  return await mutex.lock(async () => {
    return
    try {
      const command = new Deno.Command("pkill", {
        args: ["-TERM", "-u", username], // Graceful logout
        stdout: "piped",
        stderr: "piped",
      });
    
      const process = command.spawn();
      const { success, stderr } = await process.output();
    
      if (!success) {
        console.error(`App.SysCalls.LogoutUser: \nUsername: '${username}'. \nStderr: ${decoder.decode(stderr)}.`);
      }
    } catch (error) {
      console.error(`App.SysCalls.LogoutUser: \nError: ${error}.`)
    }
  })
}

export async function changeUserPassword(username: string, newPassword: string): Promise<Tried<null, null>> {
  return await mutex.lock(async () => {
    try {
      const command = new Deno.Command("chpasswd", {
        stdin: "piped",
        stderr: "piped",
      });
      
      const process = command.spawn();
      const writer = process.stdin.getWriter();
      await writer.write(encoder.encode(`${username}:${newPassword}\n`));
      await writer.close();
      
      const { success, stderr } = await process.output();
      
      if (!success) {
        console.error(`App.SysCalls.ChangeUserPassword: \nUsername: ${username}. \nStderr: ${decoder.decode(stderr)}.`);
        return Err(null)
      }
      return Ok(null)
    } catch (error) {
      console.log(`App.SysCalls.ChangeUserPassword: \nUsername: ${username}. \nError: ${error}.`);
      return Err(null)
    }
  })
}
