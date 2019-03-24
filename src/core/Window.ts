import { send } from './nativeApi'
import { SceneContext } from './SceneContext'
import { WindowId, WindowEvent } from './generated'

export class Window {
  rootSurface = 0
  sceneContext: SceneContext

  constructor(private id: WindowId) {
    this.sceneContext = new SceneContext(this.id)
  }

  // this is how you should update the scene
  // pass a callback and do whatever you need with the context
  // which will build the message and send it immediately
  updateScene(cb: (ctx: SceneContext) => void) {
    const ctx = this.getSceneContext()

    cb(ctx)
    ctx.flush()
  }

  // sometimes it's necessary to keep the context around during multiple function calls
  // (in reconciler we need to return id but we also don't want to send the batch yet)
  getSceneContext() {
    return this.sceneContext
  }

  handleEvent(event: WindowEvent) {
    console.log(event)
  }

  setSize(width: number, height: number) {
    // TODO (sync)
  }

  // TODO (sync)
  // show/hide() - explicit and simple to do
  //
  // it's not clear if close() should just call handler so that app can show
  // confirmation or if it should force the close, etc. let's leave it for later
}

export const __callbacks = []
