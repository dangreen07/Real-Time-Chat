import { LoaderFunctionArgs, redirect } from "@remix-run/node";
import { MetaFunction, useLoaderData } from "@remix-run/react";
import { useState } from "react";
import NavigationBar from "~/components/NavigationBar";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { hasValidSession } from "~/lib/auth.server";
import Cookies from 'js-cookie';

export const meta: MetaFunction = () => {
  return [
    { title: "Real Time Chat | Login" },
    { name: "description", content: "The page for logging in to the Real Time Chat application." },
  ];
};

export async function loader({
  request,
}: LoaderFunctionArgs) {
  const validSession = await hasValidSession(request);
  if (validSession) {
    return redirect("/chat");
  }
  const server_url = process.env.SERVER_URL;
  if (!server_url) {
    throw new Error("SERVER_URL environment variable is not set");
  }
  return {
    server_url
  };
}

export default function Login() {
    const { server_url } = useLoaderData<typeof loader>();
    const [error, setError] = useState("");

    async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        const username = formData.get("username");
        const password = formData.get("password");
        if (username === null || password === null || username === "" || password === "") {
            setError("Please enter a username and password");
            return;
        }
        const response = await fetch(server_url + "/login", {
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
            Cookies.set("session_id", data.session_id, { expires: 30, sameSite: "Lax", path: "/"});
            // Redirecting to the home page
            window.location.href = "/";
        }
        else {
            setError("Invalid username or password");
        }
    }

    return (
    <div className="min-h-screen flex flex-col">
        <title>Real Time Chat | Login</title>
        <NavigationBar loggedIn={false} server_url={server_url} />
        <div id="login-section" className="min-h-screen flex flex-col w-full h-full justify-center items-center gap-4">
            <Card className="w-full max-w-sm">
                <CardHeader>
                    <CardTitle>Login</CardTitle>
                    <CardDescription>Enter your username and password below to login</CardDescription>
                </CardHeader>
                <CardContent>
                    <form onSubmit={handleSubmit} className="flex flex-col gap-2">
                        <div className="flex flex-col space-y-2">
                            <Label htmlFor="name">Username</Label>
                            <Input id="username" name="username" type="text" placeholder="Your username" />
                        </div>
                        <div className="flex flex-col space-y-2">
                            <div className="flex justify-between items-center">
                                <Label htmlFor="password">Password</Label>
                                {/* <Button type="button" size="link" variant="link">Forgot your password?</Button> */}
                            </div>
                            <Input name="password" id="password" type="password" placeholder="Your password" />
                        </div>
                        {error != "" && <p className="text-red-500 text-md">{error}</p>}
                        <Button type="submit" className="w-full" variant="default">Login</Button>
                        <span className="text-sm">Don&apos;t have an account? <a href="/signup" className="underline underline-offset-4">Sign up</a></span>
                    </form>
                </CardContent>
            </Card>
        </div>
    </div>
    );
}