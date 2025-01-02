import { Contact } from "~/lib/messaging.server";

export default function ContactsList({contacts, setSelectedContact}: {contacts: Contact[], setSelectedContact: React.Dispatch<React.SetStateAction<string>>}) {
    return (
        <div id="contacts-list" className="flex flex-col flex-grow gap-2 p-2 w-1/3 border-r-2 dark:border-zinc-700">
            {contacts.map(contact => (
                <button key={contact.id} onClick={() => {setSelectedContact(contact.id)}} className="flex gap-2 items-center">
                    <div className="flex gap-2 items-center border dark:border-zinc-700 w-full p-2 rounded-md">
                        <span className="text-sm font-bold">{contact.full_name}</span>
                    </div>
                </button>
            ))}
        </div>
    )
}