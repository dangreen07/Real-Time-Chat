import { FaUser } from "react-icons/fa";
import { ModeToggle } from "./ModeToggle";
import { Button } from "./ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from "./ui/dropdown-menu";
import Cookies from 'js-cookie';

export default function NavigationBar({loggedIn, server_url}: {loggedIn: boolean, server_url: string}) {

    async function logout() {
        await fetch(server_url + "/logout", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            session_id: Cookies.get("session_id"),
          })
        });
        Cookies.remove("session_id");
        window.location.href = "/";
    }

    return (
    <div className="flex w-full px-2 absolute top-0">
    <div id="navigation-bar" className="flex w-full justify-between p-3 items-center h-20 border-b-2 border-zinc-200 dark:border-zinc-700">
        <div id="left-side">
            <button onClick={() => {
                if(window.location.href !== "/")
                {
                    window.location.href = "/";
                }
            }}>
                <img src="/logo-180.png" alt="Real Time Chat Logo" className="h-12 w-12" />
            </button>
        </div>
        <div id="right-side" className="flex gap-3 items-center">
            {loggedIn ?
            <DropdownMenu>
                <DropdownMenuTrigger>
                    <FaUser size={20} />
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuLabel>My Account</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem onClick={logout}>Log Out</DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
            :
            <Button onClick={() => {
                if (window.location.href !== "/signup")
                {
                    window.location.href = "/signup";
                }
            }} variant="default" size="default">
                Sign Up
            </Button>}
            <ModeToggle />
        </div>
    </div>
    </div>
    );
}