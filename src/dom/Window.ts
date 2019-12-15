import { Document } from './Document';
import { Event } from '../events/Event';
import { EventTarget } from '../events/EventTarget';
import { SceneContext } from '../core/SceneContext';
import { handleWindowEvent } from '../events/handleWindowEvent';
import { Location } from './Location';
import { History } from './History';

// TODO: @mixin(EventTarget) so that it's both
// correct & types are ok too
export class Window extends EventTarget {
  window = this
  self = this

  sceneContext = new SceneContext(this.id)
  document = new Document(this)

  // minimal impl for mithril/wouter
  history = new History(this)
  location = new Location(this.history)

  // react-dom needs both
  navigator = {
    userAgent: 'graffiti'
  }
  HTMLIFrameElement = class {}

  // wouter needs global Event & it could be referenced via window.* too
  Event = Event

  constructor(private id) {
    super()
  }

  handleEvent(event) {
    handleWindowEvent(this.document, event)
  }
}
