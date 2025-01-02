import { useEffect, useState } from "react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Message, NewMessage, WebSocketMessage, WebSocketObject } from "~/lib/messaging.server";
import Cookies from 'js-cookie';

export default function ChatSection({websocket, selectedContact, sendDisabled, server_url}: {websocket: WebSocket, selectedContact: string, sendDisabled: boolean, server_url: string}) {
    const [message, setMessage] = useState("");
    const [messages, setMessages] = useState<Message[]>([]);

    useEffect(() => {
        if (selectedContact !== "") {
            const session_id = Cookies.get("session_id")??"";
            fetch(`${server_url}/messages/${selectedContact}/${session_id}`)
                .then(response => response.json())
                .then(data => {
                    setMessages(data);
                })
                .catch(error => {
                    console.log(error);
                });
        }
    }, [selectedContact]);

    websocket.onmessage = (event) => {
        console.log("WebSocket message received");
        const current = JSON.parse(event.data) as NewMessage;
        const formattedMessage: Message = {
            id: "",
            user_id: current.user_id,
            recipient_id: current.recipient_id,
            message: current.message,
            sent_at: current.sent_at
        }
        setMessages([...messages, formattedMessage]);
    }

    useEffect(() => {
        console.log(messages);
    }, [messages]);

    return (
        <div id="chat-section" className="flex flex-col flex-grow gap-2 w-2/3 p-3">
            <div id="messages" className="flex flex-col flex-grow gap-2 w-full overflow-y-auto">
                {messages.sort((a,b) => a.sent_at - b.sent_at).map((current, index) => {
                    return (
                        <div key={index} className={`chat ${current.user_id != selectedContact ? "chat-end": "chat-start"}`}>
                            <div className="chat-bubble bg-blue-500 text-white">
                                <span className="text-sm font-semibold">{current.message}</span>
                            </div>
                        </div>
                    );
                })}
            </div>
            <div id="chat-input" className="flex gap-2 items-center">
                <Input value={message} onChange={(e) => setMessage(e.target.value)} placeholder="Type a message..." className="w-full dark:bg-zinc-700" />
                <Button disabled={sendDisabled || message.length == 0} onClick={() => {
                    if (selectedContact !== "") {
                        const sent_at = Date.now();
                        let sendMessage: WebSocketMessage = {
                            session_id: Cookies.get("session_id")??"",
                            message: message,
                            recipient: selectedContact,
                            sent_at: sent_at
                        };
                        let communicationMessage: WebSocketObject = {
                            session_id: Cookies.get("session_id")??"",
                            object_type: "message",
                            object: JSON.stringify(sendMessage)
                        }
                        let message_object: Message = {
                            id: "",
                            user_id: Cookies.get("session_id")??"",
                            recipient_id: selectedContact,
                            message: message,
                            sent_at: sent_at
                        }
                        websocket.send(JSON.stringify(communicationMessage));
                        setMessages([...messages, message_object]);
                        setMessage("");
                    }
                }}>Send</Button>
            </div>
        </div>
    )
}