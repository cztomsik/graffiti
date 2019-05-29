import { Document } from "./Document";
import { WindowEvent } from "../core/generated";
import { EventTarget } from "../events/EventTarget";
import { mixin } from "../core/utils";
import { SceneContext } from "../core/SceneContext";
import { handleWindowEvent } from "../events/handleWindowEvent";

export class Window {
  sceneContext = new SceneContext(this.id)
  document = new Document(this)
  listeners = {}
  //screen = { width: 1024, height: 768 }
  //navigator = {}
  //localStorage = new Storage()

  window = this

  // get location() { return this.document.location }

  //HTMLIFrameElement = class extends Element {}
  //Image = class extends Element {}

  constructor(private id) {}

  handleEvent(event: WindowEvent) {
    handleWindowEvent(this.document, event)
  }
}

mixin(Window, EventTarget)
