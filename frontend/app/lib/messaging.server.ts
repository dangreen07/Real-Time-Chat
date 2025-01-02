import { parseCookieHeader } from "./utils";

export interface Contact {
    id: string,
    username: string,
    full_name: string,
    permissions: string
}

export interface Message extends NewMessage {
    id: string
}

export interface NewMessage {
    user_id: string,
    recipient_id: string,
    message: string,
    sent_at: number
}

export interface WebSocketMessage {
  session_id: string,
  message: string,
  recipient: string,
  sent_at: number
}

export interface WebSocketObject {
    session_id: string,
    object_type: string,
    object: string
}

export async function getContacts(server_url: string, request: Request): Promise<Contact[]> {
    const cookieHeader = request.headers.get("Cookie");
    if (cookieHeader !== null) {
      const cookies = parseCookieHeader(cookieHeader);
      const session = cookies["session_id"];
      if (session) {
        const response = await fetch(`${server_url}/contacts/${session}`);
        if (response.status === 200) {
            const contacts = await response.json() as Contact[];
            return contacts;
        }
      }
    }
    return [];
}