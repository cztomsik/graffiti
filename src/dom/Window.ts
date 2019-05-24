import { Document } from "./Document";
import { WindowEvent } from "../core/generated";
import { Event } from "../events/Event";
import { EventTarget } from "../events/EventTarget";
import { mixin } from "../core/utils";
import { SceneContext } from "../core/SceneContext";

export class Window {
  sceneContext = new SceneContext(this.id)
  document = new Document(this)
  listeners = {}
  //screen = { width: 1024, height: 768 }
  //location = { href: '' }
  //navigator = {}
  //localStorage = new Storage()
  window = this

  //HTMLIFrameElement = class extends Element {}
  //Image = class extends Element {}

  constructor(private id) {}

  handleEvent(event: WindowEvent) {
    console.log(event)

    switch (event.tag) {
      case 'MouseUp': {
        this.document._getEl(event.value.target).dispatchEvent(new Event('click'))
      }
    }
  }
}

mixin(Window, EventTarget)
