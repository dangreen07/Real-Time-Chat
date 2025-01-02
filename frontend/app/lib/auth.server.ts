import { parseCookieHeader } from "./utils";

export async function hasValidSession(request: Request) {
    const cookieHeader = request.headers.get("Cookie");
    if (cookieHeader !== null) {
      const cookies = parseCookieHeader(cookieHeader);
      const session = cookies["session_id"];
      if (session) {
        try {
          const response = await fetch(process.env.SERVER_URL_FROM_SERVER + "/validate_session", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              session_id: session,
            }),
          });
          const response_body = await response.text();
          if (response_body == "true") {
            return true;
          }
        }
        catch {
          return false;
        }
      }
  }
  return false;
}

export async function getUser(request: Request): Promise<{ id: string, username: string, permissions: string } | null> {
  const cookieHeader = request.headers.get("Cookie");
  if (cookieHeader !== null) {
    const cookies = parseCookieHeader(cookieHeader);
    const session = cookies["session_id"];
    if (session) {
      const response = await fetch(process.env.SERVER_URL_FROM_SERVER + "/user/" + session, {
        method: "GET"
      });
      const response_body = await response.json() as { id: string, username: string, permissions: string };
      return response_body;
    }
  }
  return null;
}