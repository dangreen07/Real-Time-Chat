import { LoaderFunctionArgs, redirect } from "@remix-run/node";
import { Await, MetaFunction, useLoaderData } from "@remix-run/react";
import { Suspense, useEffect, useState } from "react";
import NavigationBar from "~/components/NavigationBar";
import { hasValidSession } from "~/lib/auth.server";
import { getContacts, WebSocketObject } from "~/lib/messaging.server";
import ContactsList from "./ContactsList";
import ContactsListFallback from "./ContactsListFallback";
import ChatSection from "./ChatSection";
import Cookies from 'js-cookie';

export const meta: MetaFunction = () => {
  return [
    { title: "Real Time Chat | Chat" },
    { name: "description", content: "The page for chatting with other users in real time." },
  ];
};

export async function loader({request}: LoaderFunctionArgs) {
    const isAuthenticated = await hasValidSession(request);
    if (!isAuthenticated) {
        return redirect("/");
    }
    const server_url = process.env.SERVER_URL_FROM_SERVER;
    if (!server_url) {
        throw new Error("SERVER_URL_FROM_SERVER environment variable is not set");
    }
    const ws_server_url = process.env.WS_SERVER_URL;
    if (!ws_server_url) {
        throw new Error("WS_SERVER_URL environment variable is not set");
    }
    const contacts = getContacts(server_url, request);
    return {
        server_url,
        ws_server_url,
        contacts
    }
}

export default function Chat() {
    const { server_url, ws_server_url, contacts } = useLoaderData<typeof loader>();
    const [websocket, setWebsocket] = useState(new WebSocket(ws_server_url));
    const [selectedContact, setSelectedContact] = useState("");
    const [sendDisabled, setSendDisabled] = useState(true);

    useEffect(() => {
        websocket.onopen = () => {
            console.log("WebSocket opened");
            const identificationMessage: WebSocketObject = {
                session_id: Cookies.get("session_id")??"",
                object_type: "identify",
                object: ""
            }
            websocket.send(JSON.stringify(identificationMessage));
            setSendDisabled(false);
        }

        websocket.onclose = () => {
            setSendDisabled(true);
            try {
                let ws = new WebSocket(ws_server_url);
                setWebsocket(ws);
            }
            catch {
                return;
            }
        }
    },[]);

    return (
        <div className="min-h-screen flex flex-col">
            <NavigationBar loggedIn={true} server_url={server_url} />
            <div id="main-section" className="flex flex-col h-screen">
                <div id="padding" className="h-20 w-full" />
                <div id="content" className="flex flex-grow min-h-[calc(100vh-5rem)] gap-2 w-full p-3">
                    <Suspense fallback={<ContactsListFallback />}>
                        <Await resolve={contacts}>
                            {contacts => <ContactsList contacts={contacts} setSelectedContact={setSelectedContact} />}
                        </Await>
                    </Suspense>
                    <ChatSection
                    websocket={websocket}
                    selectedContact={selectedContact}
                    sendDisabled={sendDisabled}
                    server_url={server_url}
                    />
                </div>
            </div>
        </div>
    )
}