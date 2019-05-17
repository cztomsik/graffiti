import { Document } from "./Document";
import { WindowEvent } from "../core/generated";
import { Event } from "../events/Event";
import { EventTarget } from "../events/EventTarget";
import { mixin } from "../core/utils";
import { SceneContext } from "../core/SceneContext";

export class Window {
  sceneContext = new SceneContext(this.id)
  document = new Document(this.sceneContext)
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

  // preact does some golfing with `on<event> in window` to detect casing so we pretend to have these props too
  // https://github.com/developit/preact/blob/a23b921391545fce712dfc92ea200f35158207d0/src/diff/props.js#L79
  //
  // TODO: we can use this opportunity to also explicitly unsupport on* props (and maybe a lots of others)
  set onclick(v) {
    throw new Error('unsupported')
  }
}

mixin(Window, EventTarget)
