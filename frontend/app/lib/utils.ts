import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function parseCookieHeader(cookieHeader: string) {
  const splitCookies = cookieHeader.split(";");
  const keyValuePars = splitCookies.map(pair => pair.split("="));
  const cookies = keyValuePars.reduce((acc, v) => {
      acc[decodeURIComponent(v[0].trim())] = decodeURIComponent(v[1].trim());
      return acc;
  }, {} as Record<string, string>);
  return cookies;
}