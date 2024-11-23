import { BACKEND } from "@/config";
import { NextResponse } from 'next/server';

async function unauthorized(req) {
    const url = new URL("/login", req.url);
    url.searchParams.set("next", req.nextUrl.pathname + req.nextUrl.search);
    return NextResponse.redirect(url);
}

// Admin panel redirect
export async function middleware(req) {
    const session = req.cookies.get("session")?.value;
    // Sanity check
    if (!session || !/^[a-zA-Z0-9+_=\/\-]+$/.test(session)) {
        return unauthorized(req);
    }
    // Proxy to the backend
    const res = await fetch(BACKEND + "/check", {
        headers: {
            cookie: `session=${session}`
        }
    });
    if (!res.ok) {
        return unauthorized(req);
    }

    return NextResponse.next();
}

export const config = {
    matcher: '/admin/:path*'
}