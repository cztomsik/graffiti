import { Event } from '../events/Event'

export class EventTarget implements globalThis.EventTarget {
  // preact uses node._listeners
  _etListeners: { [type in string]?: readonly EventListenerOrEventListenerObject[] } = {}

  addEventListener(type, listener) {
    this._etListeners[type] = [...this._getListeners(type), listener]
  }

  removeEventListener(type, listener) {
    this._etListeners[type] = this._getListeners(type).filter(l => l !== listener)
  }

  dispatchEvent(event) {
    event.target = this

    this._dispatch(event)

    return !event.defaultPrevented
  }

  _fire(type, data = {}) {
    this.dispatchEvent(Object.assign(new Event(type), { target: this, ...data }))
  }

  _dispatch(event) {
    event.currentTarget = this

    for (const l of this._getListeners(event.type)) {
      if ('handleEvent' in l) {
        l.handleEvent(event)
      } else {
        l.call(this, event)
      }

      if (event.cancelBubbleImmediately) {
        break
      }
    }

    if (!event.cancelBubble) {
      this._bubble(event)
    }
  }

  _bubble(event) {
    const parent = this._getTheParent()

    if (parent) {
      parent._dispatch(event)
    }
  }

  // https://dom.spec.whatwg.org/#get-the-parent
  _getTheParent() {
    return null
  }

  _getListeners(type) {
    return this._etListeners[type] || []
  }

  // on* event handler props
  // - makes TS happy
  // - everything is here so we don't need to repeat it again for document & window
  // - we only define getter -> setter is not supported and will throw
  // - preact needs this for some golfing: name = (nameLower in dom ? nameLower : name).slice(2);
  //   https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80

  get onabort() { return null }
  get onanimationcancel() { return null }
  get onanimationend() { return null }
  get onanimationiteration() { return null }
  get onanimationstart() { return null }
  get onauxclick() { return null }
  get onblur() { return null }
  get oncancel() { return null }
  get oncanplay() { return null }
  get oncanplaythrough() { return null }
  get onchange() { return null }
  get onclick() { return null }
  get onclose() { return null }
  get oncontextmenu() { return null }
  get oncopy() { return null }
  get oncuechange() { return null }
  get oncut() { return null }
  get ondblclick() { return null }
  get ondrag() { return null }
  get ondragend() { return null }
  get ondragenter() { return null }
  get ondragexit() { return null }
  get ondragleave() { return null }
  get ondragover() { return null }
  get ondragstart() { return null }
  get ondrop() { return null }
  get ondurationchange() { return null }
  get onemptied() { return null }
  get onended() { return null }
  get onerror() { return null }
  get onfocus() { return null }
  get onfullscreenchange() { return null }
  get onfullscreenerror() { return null }
  get ongotpointercapture() { return null }
  get oninput() { return null }
  get oninvalid() { return null }
  get onkeydown() { return null }
  get onkeypress() { return null }
  get onkeyup() { return null }
  get onload() { return null }
  get onloadeddata() { return null }
  get onloadedmetadata() { return null }
  get onloadstart() { return null }
  get onlostpointercapture() { return null }
  get onmousedown() { return null }
  get onmouseenter() { return null }
  get onmouseleave() { return null }
  get onmousemove() { return null }
  get onmouseout() { return null }
  get onmouseover() { return null }
  get onmouseup() { return null }
  get onpaste() { return null }
  get onpause() { return null }
  get onplay() { return null }
  get onplaying() { return null }
  get onpointercancel() { return null }
  get onpointerdown() { return null }
  get onpointerenter() { return null }
  get onpointerleave() { return null }
  get onpointermove() { return null }
  get onpointerout() { return null }
  get onpointerover() { return null }
  get onpointerup() { return null }
  get onprogress() { return null }
  get onratechange() { return null }
  get onreset() { return null }
  get onresize() { return null }
  get onscroll() { return null }
  get onsecuritypolicyviolation() { return null }
  get onseeked() { return null }
  get onseeking() { return null }
  get onselect() { return null }
  get onselectionchange() { return null }
  get onselectstart() { return null }
  get onstalled() { return null }
  get onsubmit() { return null }
  get onsuspend() { return null }
  get ontimeupdate() { return null }
  get ontoggle() { return null }
  get ontouchcancel() { return null }
  get ontouchend() { return null }
  get ontouchmove() { return null }
  get ontouchstart() { return null }
  get ontransitioncancel() { return null }
  get ontransitionend() { return null }
  get ontransitionrun() { return null }
  get ontransitionstart() { return null }
  get onvolumechange() { return null }
  get onwaiting() { return null }
  get onwheel() { return null }

  // special case for react-dom
  // which tries to set noop to onclick to avoid some safari bug
  set onclick(cb) {}
}
