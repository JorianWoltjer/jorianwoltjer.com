// Different options for local development and docker-compose
export const BACKEND = process.env.BACKEND || "http://localhost:8000"  // Internal
export const BACKEND_API = process.env.NEXT_PUBLIC_BACKEND_API || "http://localhost:8000" // External

export const SLUG_REGEX = /^[\w-\/]+$/

export function getWebsocketURL(path) {
    if (BACKEND_API.startsWith('http')) {
        return BACKEND_API.replace(/^http/, 'ws') + path
    } else {  // Handle relative URLs
        return 'ws://' + location.host + "/api" + path
    }
}
