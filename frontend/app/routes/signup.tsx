import { LoaderFunctionArgs, redirect } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { useState } from "react";
import NavigationBar from "~/components/NavigationBar";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { hasValidSession } from "~/lib/auth.server";
import Cookies from 'js-cookie';

export async function loader({
  request,
}: LoaderFunctionArgs) {
  const validSession = await hasValidSession(request);
  if (validSession) {
    return redirect("/");
  }
  const server_url = process.env.SERVER_URL;
  if (!server_url) {
    throw new Error("SERVER_URL environment variable is not set");
  }
  return {
    server_url
  };
}

export default function Signup() {
    const { server_url } = useLoaderData<typeof loader>();
    const [error, setError] = useState("");
    
    async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        const username = formData.get("username");
        const password = formData.get("password");
        const confirmPassword = formData.get("confirm-password");
        if (username === null || password === null || confirmPassword === null || username === "" || password === "" || confirmPassword === "") {
            setError("Please enter a username, password, and confirm password");
            return;
        }
        if (password !== confirmPassword) {
            setError("Passwords do not match");
            return;
        }
        const response = await fetch(server_url + "/signup", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: username,
                password: password
            })
        });
        const data = await response.json() as {session_id: string, error: string}
        if (data.error === "") {
            // Setting the session cookie
            Cookies.set("session_id", data.session_id);
            // Redirecting to the home page
            window.location.href = "/";
        }
        else {
            setError("Invalid username or password");
        }
    }

    return (
    <div className="min-h-screen flex flex-col">
        <title>Real Time Chat | Signup</title>
        <NavigationBar loggedIn={false} />
        <div id="signup-section" className="min-h-screen flex flex-col w-full h-full justify-center items-center gap-4">
            <Card className="w-full max-w-sm">
                <CardHeader>
                    <CardTitle>Signup</CardTitle>
                    <CardDescription>Enter a username and password below to signup</CardDescription>
                </CardHeader>
                <CardContent>
                    <form onSubmit={handleSubmit} className="flex flex-col gap-2">
                        <div className="flex flex-col space-y-2">
                            <Label htmlFor="name">Username</Label>
                            <Input id="username" name="username" type="text" placeholder="Username" />
                        </div>
                        <div className="flex flex-col space-y-2">
                            <Label htmlFor="password">Password</Label>
                            <Input name="password" id="password" type="password" placeholder="Password" />
                        </div>
                        <div className="flex flex-col space-y-2">
                            <Label htmlFor="password">Confirm Password</Label>
                            <Input name="confirm-password" id="confirm-password" type="password" placeholder="Confirm password" />
                        </div>
                        {error != "" && <p className="text-red-500 text-md text-center">{error}</p>}
                        <Button type="submit" className="w-full" variant="default">Signup</Button>
                        <span className="text-sm">Already have an account? <a href="/login" className="underline underline-offset-4">Log In</a></span>
                    </form>
                </CardContent>
            </Card>
        </div>
    </div>
    )
}