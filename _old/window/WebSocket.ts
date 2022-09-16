// @ts-expect-error
export const WebSocket = globalThis.WebSocket ?? (await import('ws')).default
