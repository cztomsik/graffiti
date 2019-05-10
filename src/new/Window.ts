import { Document } from "./Document";
import { Window as OldWnd } from '../core'
import { WindowEvent } from "../core/generated";
import { Event } from "./events/Event";
import { EventTarget } from "./events/EventTarget";

export class Window extends OldWnd {
  document = new Document(this.sceneContext)

  handleEvent(event: WindowEvent) {
    console.log(event)

    switch (event.tag) {
      case 'MouseUp': {
        this.document._getEl(event.value.target).dispatchEvent(new Event('click'))
      }
    }
  }

  // preact does some golfing with `onevent in window` do detect casing
  // https://github.com/developit/preact/blob/a23b921391545fce712dfc92ea200f35158207d0/src/diff/props.js#L79
  //
  // TODO: we can use this opportunity to also explicitly unsupport on* props (and maybe a lot of others)
  set onclick(v) {
    throw new Error('unsupported')
  }
}

Object.assign(Window.prototype, EventTarget.prototype)
