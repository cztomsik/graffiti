import { NOOP } from '../util'

const rafCbs: FrameRequestCallback[] = []
let nextRafHandle = 0

export const requestAnimationFrame = (callback: FrameRequestCallback): number => {
  if (rafCbs.length === 0) {
    const animate = () => {
      const timestamp = performance.now()

      for (const cb of rafCbs) {
        cb(timestamp)
      }

      rafCbs.length = 0
    }

    // TODO: maybe we should cap at 60fps
    //       but maybe it's ok because rendering will block until next frame
    setTimeout(animate)
  }

  rafCbs.push(callback)

  return nextRafHandle++
}

export const cancelAnimationFrame = (handle: number) => {
  const index = nextRafHandle - handle

  if (index >= 0) {
    // replace so that other indices remain valid too
    rafCbs[index] = NOOP
  }
}
