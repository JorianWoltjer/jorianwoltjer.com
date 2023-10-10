import { BACKEND } from "@/config";
import { NextResponse } from 'next/server';

async function unauthorized(req) {
    const url = new URL("/login", req.url);
    url.searchParams.set("next", req.nextUrl.pathname + req.nextUrl.search);
    return NextResponse.redirect(url);
}

// Admin panel redirect
export async function middleware(req) {
    const sid = req.cookies.get("sid")?.value;
    // Sanity check
    if (!sid || !/^[a-zA-Z0-9+_=\/\-]+$/.test(sid)) {
        return unauthorized(req);
    }
    // Proxy to the backend
    const res = await fetch(BACKEND + "/login", {
        headers: {
            cookie: `sid=${sid}`
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