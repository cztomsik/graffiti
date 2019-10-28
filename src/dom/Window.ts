import { Document } from "./Document";
import { EventTarget } from "../events/EventTarget";
import { mixin } from '../core/utils';
import { SceneContext } from "../core/SceneContext";
import { handleWindowEvent } from "../events/handleWindowEvent";
import { Location } from './Location';

export class Window extends EventTarget {
  sceneContext = new SceneContext(this.id)
  window = this
  document = new Document(this)

  // minimal impl for mithril router
  history = {}
  location = new Location(this)

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
