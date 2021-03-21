import { Event } from '../events/Event'

export const GET_THE_PARENT = Symbol()

export class EventTarget implements globalThis.EventTarget {
  // beware collisions, preact is using node._listeners
  #listeners: { [type in string]: readonly EventListenerOrEventListenerObject[] } = Object.create(
    new Proxy(
      {},
      {
        // return empty array for unknown event types
        get: () => [],
      }
    )
  )

  addEventListener(type: string, listener) {
    this.#listeners[type] = [...this.#listeners[type], listener]
  }

  removeEventListener(type: string, listener) {
    this.#listeners[type] = this.#listeners[type].filter(l => l !== listener)
  }

  dispatchEvent(event: Event) {
    event.target = this

    this._dispatch(event)

    return !event.defaultPrevented
  }

  _fire(type: string, data = {}) {
    this.dispatchEvent(Object.assign(new Event(type), { target: this, ...data }))
  }

  // TODO: inline in dispatchEvent but it MUST NOT set event.target during bubbling
  _dispatch(event: Event) {
    event.currentTarget = this

    for (const l of this.#listeners[event.type]) {
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
      // bubble
      this[GET_THE_PARENT]()?._dispatch(event)
    }
  }

  // https://dom.spec.whatwg.org/#get-the-parent
  [GET_THE_PARENT](): EventTarget | null {
    return null
  }

  // on* event handler props
  // - makes TS happy
  // - everything is here so we don't need to repeat it again in all possible ev targets
  // - we only define getter, setter is not supported and will throw
  // - preact needs this for some golfing: name = (nameLower in dom ? nameLower : name).slice(2);
  //   https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80
  //
  // TODO: unsupported events could only be declared so they throw when used

  get onabort() { return null }
  get onafterprint() { return null }
  get onanimationcancel() { return null }
  get onanimationend() { return null }
  get onanimationiteration() { return null }
  get onanimationstart() { return null }
  get onauxclick() { return null }
  get onbeforeprint() { return null }
  get onbeforeunload() { return null }
  get onblur() { return null }
  get oncancel() { return null }
  get oncanplay() { return null }
  get oncanplaythrough() { return null }
  get onchange() { return null }
  get onclick() { return null }
  get onclose() { return null }
  get oncompassneedscalibration() { return null }
  get oncontextmenu() { return null }
  get oncopy() { return null }
  get oncuechange() { return null }
  get oncut() { return null }
  get ondblclick() { return null }
  get ondevicelight() { return null }
  get ondevicemotion() { return null }
  get ondeviceorientation() { return null }
  get ondeviceorientationabsolute() { return null }
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
  get onhashchange() { return null }
  get oninput() { return null }
  get oninvalid() { return null }
  get onkeydown() { return null }
  get onkeypress() { return null }
  get onkeyup() { return null }
  get onlanguagechange() { return null }
  get onload() { return null }
  get onloadeddata() { return null }
  get onloadedmetadata() { return null }
  get onloadstart() { return null }
  get onlostpointercapture() { return null }
  get onmessage() { return null }
  get onmessageerror() { return null }
  get onmousedown() { return null }
  get onmouseenter() { return null }
  get onmouseleave() { return null }
  get onmousemove() { return null }
  get onmouseout() { return null }
  get onmouseover() { return null }
  get onmouseup() { return null }
  get onmousewheel() { return null }
  get onoffline() { return null }
  get ononline() { return null }
  get onorientationchange() { return null }
  get onpagehide() { return null }
  get onpageshow() { return null }
  get onpaste() { return null }
  get onpause() { return null }
  get onplay() { return null }
  get onplaying() { return null }
  get onpointercancel() { return null }
  get onpointerdown() { return null }
  get onpointerenter() { return null }
  get onpointerleave() { return null }
  get onpointerlockchange() { return null }
  get onpointerlockerror() { return null }
  get onpointermove() { return null }
  get onpointerout() { return null }
  get onpointerover() { return null }
  get onpointerup() { return null }
  get onpopstate() { return null }
  get onprogress() { return null }
  get onratechange() { return null }
  get onreadystatechange() { return null }
  get onrejectionhandled() { return null }
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
  get onstorage() { return null }
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
  get onunhandledrejection() { return null }
  get onunload() { return null }
  get onvisibilitychange() { return null }
  get onvolumechange() { return null }
  get onvrdisplayactivate() { return null }
  get onvrdisplayblur() { return null }
  get onvrdisplayconnect() { return null }
  get onvrdisplaydeactivate() { return null }
  get onvrdisplaydisconnect() { return null }
  get onvrdisplayfocus() { return null }
  get onvrdisplaypointerrestricted() { return null }
  get onvrdisplaypointerunrestricted() { return null }
  get onvrdisplaypresentchange() { return null }
  get onwaiting() { return null }
  get onwheel() { return null }
  get onzoom() { return null }

  // special-case for react-dom
  // which tries to set noop to onclick to avoid some safari bug
  set onclick(cb) {}

  // ignore vendor
  onmsgesturechange
  onmsgesturedoubletap
  onmsgestureend
  onmsgesturehold
  onmsgesturestart
  onmsgesturetap
  onmsinertiastart
  onmspointercancel
  onmspointerdown
  onmspointerenter
  onmspointerleave
  onmspointermove
  onmspointerout
  onmspointerover
  onmspointerup

  // WTF
  [index: number]: Window
}
