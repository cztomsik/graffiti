import * as open from 'open'

// no idea what it actually supports but it works for urls
export async function openURL(url: string) {
  return open(url)
}
