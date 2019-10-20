import { Document } from "./Document";
import { EventTarget } from "../events/EventTarget";
import { mixin } from "../core/utils";
import { SceneContext } from "../core/SceneContext";
import { handleWindowEvent } from "../events/handleWindowEvent";

export class Window extends EventTarget {
  sceneContext = new SceneContext(this.id)
  window = this
  document = new Document(this)

  // mithril router
  history = {}
  location = {
    hash: '',
    search: ''
  }

  // react-dom needs both
  navigator = {
    userAgent: 'graffiti'
  }
  HTMLIFrameElement = class {}

  constructor(private id) {
    super()
  }

  handleEvent(event) {
    handleWindowEvent(this.document, event)
  }
}

mixin(Window, EventTarget)
