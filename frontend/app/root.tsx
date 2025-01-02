import type {
  LoaderFunction,
  LoaderFunctionArgs,
} from '@remix-run/node';
import { json } from '@remix-run/node';
import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  useLoaderData,
  useFetcher,
} from '@remix-run/react';

import "./tailwind.css";
import { getThemeFromCookie } from '~/lib/theme.server';
import { ThemeProvider } from './components/theme-provider';

// Code for dark mode toggle
// https://github.com/abdulkader/remix-shadcn-ui-dark-theme-demo/tree/main

export const loader: LoaderFunction = async ({ request }: LoaderFunctionArgs) => {
  const theme = await getThemeFromCookie(request);
  return json({
    theme,
  });
};

export default function App() {
  const { theme = 'system' } = useLoaderData<typeof loader>();
  const fetcher = useFetcher();
  const onThemeChange = (theme: string) => {
    fetcher.submit(
      { theme },
      {
        method: 'post',
        encType: 'application/json',
        action: '/api/toggleTheme',
      },
    );
  };
  return (
    <html lang="en" className={theme ?? 'theme'}>
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width,initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body className="bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100">
        <ThemeProvider defaultTheme={theme} onThemeChange={onThemeChange}>
          <Outlet />
        </ThemeProvider>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}