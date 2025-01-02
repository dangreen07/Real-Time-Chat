import { Message } from "./messaging.server";
import { parseCookieHeader } from "./utils";

export async function getMessages(server_url: string, recipient_id: string, request: Request): Promise<Message[]> {
    const cookieHeader = request.headers.get("Cookie");
    if (cookieHeader !== null) {
      const cookies = parseCookieHeader(cookieHeader);
      const session = cookies["session_id"];
      if (session) {
        const response = await fetch(`${server_url}/messages/${recipient_id}/${session}`);
        if (response.status === 200) {
            const messages = await response.json() as Message[];
            return messages;
        }
        else {
          console.log(await response.text());
        }
      }
    }
    return [];
  }