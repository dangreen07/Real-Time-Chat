import { type LoaderFunctionArgs, type MetaFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import NavigationBar from "~/components/NavigationBar";
import { Button } from "~/components/ui/button";
import { hasValidSession } from "~/lib/auth.server";

export const meta: MetaFunction = () => {
  return [
    { title: "Real Time Chat" },
    { name: "description", content: "An app to chat with other users in real time." },
  ];
};

export async function loader({
  request,
}: LoaderFunctionArgs) {
  const validSession = await hasValidSession(request);
  const server_url = process.env.SERVER_URL;
  if (!server_url) {
    throw new Error("SERVER_URL environment variable is not set");
  }
  return {
    validSession,
    server_url
  };
}

export default function Index() {
  const { validSession, server_url } = useLoaderData<typeof loader>();
  return (
    <div className="flex min-h-screen flex-col">
      <NavigationBar loggedIn={validSession} server_url={server_url} />
      <div id="hero-section" className="min-h-screen flex flex-col w-full h-full justify-center items-center gap-4">
        <h1 className="text-6xl font-bold">Messaging made simple</h1>
        <p className="text-2xl">
          A simple chat app with contacts and groupchats
        </p>
        <div className="flex flex-col items-center justify-center w-full h-full">
          <Button variant="default" size="lg" onClick={() => {
            window.location.href = "/signup";
          }}>Get Started</Button>
        </div>
      </div>
    </div>
  );
}