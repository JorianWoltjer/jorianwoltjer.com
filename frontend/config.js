// Different options for local development and docker-compose
export const BACKEND = process.env.BACKEND || "http://localhost:8000"  // Internal address
export const BACKEND_API = process.env.NEXT_PUBLIC_BACKEND_API || "http://localhost:8000" // External address
export const CDN = process.env.NEXT_PUBLIC_CDN || ""  // nginx
export const HOST = process.env.NEXT_PUBLIC_SITE_URL || "http://localhost:8000"

export const SLUG_REGEX = /^[\w-\/]+$/

export function getWebsocketURL(path) {
  if (BACKEND_API.startsWith('http')) {
    return BACKEND_API.replace(/^http/, 'ws') + path
  } else {  // Handle relative URLs
    const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:'
    return `${protocol}//${location.host}/api${path}`
  }
}
