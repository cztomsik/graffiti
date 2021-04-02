import { Event } from '../events/Event'

export const GET_THE_PARENT = Symbol()

export class EventTarget implements globalThis.EventTarget {
  // beware collisions, preact is using node._listeners
  #listeners: { [type in string]: EventListenerOrEventListenerObject[] } = Object.create(
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

  [GET_LISTENER](type: string) {
    return this.#listeners[type].find(l => l instanceof InlineListener)
  }

  [SET_LISTENER](type: string, listener) {
    const listeners = this.#listeners[type]
    const index = listeners[type].findIndex(l => l instanceof InlineListener)

    listeners[~index ?index :listeners.length] = new InlineListener(listener)
  }

  // on* event handler props
  // - makes TS happy
  // - everything is here so we don't need to repeat it again in all possible ev targets
  // - preact needs this for some golfing: name = (nameLower in dom ? nameLower : name).slice(2);
  //   https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80

  get onabort() {
    return this[GET_LISTENER]('onabort')
  }

  set onabort(listener) {
    this[SET_LISTENER]('onabort', listener)
  }
  
  get onafterprint() {
    return this[GET_LISTENER]('onafterprint')
  }

  set onafterprint(listener) {
    this[SET_LISTENER]('onafterprint', listener)
  }
  
  get onanimationcancel() {
    return this[GET_LISTENER]('onanimationcancel')
  }

  set onanimationcancel(listener) {
    this[SET_LISTENER]('onanimationcancel', listener)
  }
  
  get onanimationend() {
    return this[GET_LISTENER]('onanimationend')
  }

  set onanimationend(listener) {
    this[SET_LISTENER]('onanimationend', listener)
  }
  
  get onanimationiteration() {
    return this[GET_LISTENER]('onanimationiteration')
  }

  set onanimationiteration(listener) {
    this[SET_LISTENER]('onanimationiteration', listener)
  }
  
  get onanimationstart() {
    return this[GET_LISTENER]('onanimationstart')
  }

  set onanimationstart(listener) {
    this[SET_LISTENER]('onanimationstart', listener)
  }
  
  get onauxclick() {
    return this[GET_LISTENER]('onauxclick')
  }

  set onauxclick(listener) {
    this[SET_LISTENER]('onauxclick', listener)
  }
  
  get onbeforeprint() {
    return this[GET_LISTENER]('onbeforeprint')
  }

  set onbeforeprint(listener) {
    this[SET_LISTENER]('onbeforeprint', listener)
  }
  
  get onbeforeunload() {
    return this[GET_LISTENER]('onbeforeunload')
  }

  set onbeforeunload(listener) {
    this[SET_LISTENER]('onbeforeunload', listener)
  }
  
  get onblur() {
    return this[GET_LISTENER]('onblur')
  }

  set onblur(listener) {
    this[SET_LISTENER]('onblur', listener)
  }
  
  get oncancel() {
    return this[GET_LISTENER]('oncancel')
  }

  set oncancel(listener) {
    this[SET_LISTENER]('oncancel', listener)
  }
  
  get oncanplay() {
    return this[GET_LISTENER]('oncanplay')
  }

  set oncanplay(listener) {
    this[SET_LISTENER]('oncanplay', listener)
  }
  
  get oncanplaythrough() {
    return this[GET_LISTENER]('oncanplaythrough')
  }

  set oncanplaythrough(listener) {
    this[SET_LISTENER]('oncanplaythrough', listener)
  }
  
  get onchange() {
    return this[GET_LISTENER]('onchange')
  }

  set onchange(listener) {
    this[SET_LISTENER]('onchange', listener)
  }
  
  get onclick() {
    return this[GET_LISTENER]('onclick')
  }

  set onclick(listener) {
    this[SET_LISTENER]('onclick', listener)
  }
  
  get onclose() {
    return this[GET_LISTENER]('onclose')
  }

  set onclose(listener) {
    this[SET_LISTENER]('onclose', listener)
  }
  
  get oncompassneedscalibration() {
    return this[GET_LISTENER]('oncompassneedscalibration')
  }

  set oncompassneedscalibration(listener) {
    this[SET_LISTENER]('oncompassneedscalibration', listener)
  }
  
  get oncontextmenu() {
    return this[GET_LISTENER]('oncontextmenu')
  }

  set oncontextmenu(listener) {
    this[SET_LISTENER]('oncontextmenu', listener)
  }
  
  get oncopy() {
    return this[GET_LISTENER]('oncopy')
  }

  set oncopy(listener) {
    this[SET_LISTENER]('oncopy', listener)
  }
  
  get oncuechange() {
    return this[GET_LISTENER]('oncuechange')
  }

  set oncuechange(listener) {
    this[SET_LISTENER]('oncuechange', listener)
  }
  
  get oncut() {
    return this[GET_LISTENER]('oncut')
  }

  set oncut(listener) {
    this[SET_LISTENER]('oncut', listener)
  }
  
  get ondblclick() {
    return this[GET_LISTENER]('ondblclick')
  }

  set ondblclick(listener) {
    this[SET_LISTENER]('ondblclick', listener)
  }
  
  get ondevicelight() {
    return this[GET_LISTENER]('ondevicelight')
  }

  set ondevicelight(listener) {
    this[SET_LISTENER]('ondevicelight', listener)
  }
  
  get ondevicemotion() {
    return this[GET_LISTENER]('ondevicemotion')
  }

  set ondevicemotion(listener) {
    this[SET_LISTENER]('ondevicemotion', listener)
  }
  
  get ondeviceorientation() {
    return this[GET_LISTENER]('ondeviceorientation')
  }

  set ondeviceorientation(listener) {
    this[SET_LISTENER]('ondeviceorientation', listener)
  }
  
  get ondeviceorientationabsolute() {
    return this[GET_LISTENER]('ondeviceorientationabsolute')
  }

  set ondeviceorientationabsolute(listener) {
    this[SET_LISTENER]('ondeviceorientationabsolute', listener)
  }
  
  get ondrag() {
    return this[GET_LISTENER]('ondrag')
  }

  set ondrag(listener) {
    this[SET_LISTENER]('ondrag', listener)
  }
  
  get ondragend() {
    return this[GET_LISTENER]('ondragend')
  }

  set ondragend(listener) {
    this[SET_LISTENER]('ondragend', listener)
  }
  
  get ondragenter() {
    return this[GET_LISTENER]('ondragenter')
  }

  set ondragenter(listener) {
    this[SET_LISTENER]('ondragenter', listener)
  }
  
  get ondragexit() {
    return this[GET_LISTENER]('ondragexit')
  }

  set ondragexit(listener) {
    this[SET_LISTENER]('ondragexit', listener)
  }
  
  get ondragleave() {
    return this[GET_LISTENER]('ondragleave')
  }

  set ondragleave(listener) {
    this[SET_LISTENER]('ondragleave', listener)
  }
  
  get ondragover() {
    return this[GET_LISTENER]('ondragover')
  }

  set ondragover(listener) {
    this[SET_LISTENER]('ondragover', listener)
  }
  
  get ondragstart() {
    return this[GET_LISTENER]('ondragstart')
  }

  set ondragstart(listener) {
    this[SET_LISTENER]('ondragstart', listener)
  }
  
  get ondrop() {
    return this[GET_LISTENER]('ondrop')
  }

  set ondrop(listener) {
    this[SET_LISTENER]('ondrop', listener)
  }
  
  get ondurationchange() {
    return this[GET_LISTENER]('ondurationchange')
  }

  set ondurationchange(listener) {
    this[SET_LISTENER]('ondurationchange', listener)
  }
  
  get onemptied() {
    return this[GET_LISTENER]('onemptied')
  }

  set onemptied(listener) {
    this[SET_LISTENER]('onemptied', listener)
  }
  
  get onended() {
    return this[GET_LISTENER]('onended')
  }

  set onended(listener) {
    this[SET_LISTENER]('onended', listener)
  }
  
  get onerror() {
    return this[GET_LISTENER]('onerror')
  }

  set onerror(listener) {
    this[SET_LISTENER]('onerror', listener)
  }
  
  get onfocus() {
    return this[GET_LISTENER]('onfocus')
  }

  set onfocus(listener) {
    this[SET_LISTENER]('onfocus', listener)
  }
  
  get onfullscreenchange() {
    return this[GET_LISTENER]('onfullscreenchange')
  }

  set onfullscreenchange(listener) {
    this[SET_LISTENER]('onfullscreenchange', listener)
  }
  
  get onfullscreenerror() {
    return this[GET_LISTENER]('onfullscreenerror')
  }

  set onfullscreenerror(listener) {
    this[SET_LISTENER]('onfullscreenerror', listener)
  }

  get ongamepadconnected() {
    return this[GET_LISTENER]('ongamepadconnected')
  }

  set ongamepadconnected(listener) {
    this[SET_LISTENER]('ongamepadconnected', listener)
  }

  get ongamepaddisconnected() {
    return this[GET_LISTENER]('ongamepaddisconnected')
  }

  set ongamepaddisconnected(listener) {
    this[SET_LISTENER]('ongamepaddisconnected', listener)
  }

  get ongotpointercapture() {
    return this[GET_LISTENER]('ongotpointercapture')
  }

  set ongotpointercapture(listener) {
    this[SET_LISTENER]('ongotpointercapture', listener)
  }
  
  get onhashchange() {
    return this[GET_LISTENER]('onhashchange')
  }

  set onhashchange(listener) {
    this[SET_LISTENER]('onhashchange', listener)
  }
  
  get oninput() {
    return this[GET_LISTENER]('oninput')
  }

  set oninput(listener) {
    this[SET_LISTENER]('oninput', listener)
  }
  
  get oninvalid() {
    return this[GET_LISTENER]('oninvalid')
  }

  set oninvalid(listener) {
    this[SET_LISTENER]('oninvalid', listener)
  }
  
  get onkeydown() {
    return this[GET_LISTENER]('onkeydown')
  }

  set onkeydown(listener) {
    this[SET_LISTENER]('onkeydown', listener)
  }
  
  get onkeypress() {
    return this[GET_LISTENER]('onkeypress')
  }

  set onkeypress(listener) {
    this[SET_LISTENER]('onkeypress', listener)
  }
  
  get onkeyup() {
    return this[GET_LISTENER]('onkeyup')
  }

  set onkeyup(listener) {
    this[SET_LISTENER]('onkeyup', listener)
  }
  
  get onlanguagechange() {
    return this[GET_LISTENER]('onlanguagechange')
  }

  set onlanguagechange(listener) {
    this[SET_LISTENER]('onlanguagechange', listener)
  }
  
  get onload() {
    return this[GET_LISTENER]('onload')
  }

  set onload(listener) {
    this[SET_LISTENER]('onload', listener)
  }
  
  get onloadeddata() {
    return this[GET_LISTENER]('onloadeddata')
  }

  set onloadeddata(listener) {
    this[SET_LISTENER]('onloadeddata', listener)
  }
  
  get onloadedmetadata() {
    return this[GET_LISTENER]('onloadedmetadata')
  }

  set onloadedmetadata(listener) {
    this[SET_LISTENER]('onloadedmetadata', listener)
  }
  
  get onloadstart() {
    return this[GET_LISTENER]('onloadstart')
  }

  set onloadstart(listener) {
    this[SET_LISTENER]('onloadstart', listener)
  }
  
  get onlostpointercapture() {
    return this[GET_LISTENER]('onlostpointercapture')
  }

  set onlostpointercapture(listener) {
    this[SET_LISTENER]('onlostpointercapture', listener)
  }
  
  get onmessage() {
    return this[GET_LISTENER]('onmessage')
  }

  set onmessage(listener) {
    this[SET_LISTENER]('onmessage', listener)
  }
  
  get onmessageerror() {
    return this[GET_LISTENER]('onmessageerror')
  }

  set onmessageerror(listener) {
    this[SET_LISTENER]('onmessageerror', listener)
  }
  
  get onmousedown() {
    return this[GET_LISTENER]('onmousedown')
  }

  set onmousedown(listener) {
    this[SET_LISTENER]('onmousedown', listener)
  }
  
  get onmouseenter() {
    return this[GET_LISTENER]('onmouseenter')
  }

  set onmouseenter(listener) {
    this[SET_LISTENER]('onmouseenter', listener)
  }
  
  get onmouseleave() {
    return this[GET_LISTENER]('onmouseleave')
  }

  set onmouseleave(listener) {
    this[SET_LISTENER]('onmouseleave', listener)
  }
  
  get onmousemove() {
    return this[GET_LISTENER]('onmousemove')
  }

  set onmousemove(listener) {
    this[SET_LISTENER]('onmousemove', listener)
  }
  
  get onmouseout() {
    return this[GET_LISTENER]('onmouseout')
  }

  set onmouseout(listener) {
    this[SET_LISTENER]('onmouseout', listener)
  }
  
  get onmouseover() {
    return this[GET_LISTENER]('onmouseover')
  }

  set onmouseover(listener) {
    this[SET_LISTENER]('onmouseover', listener)
  }
  
  get onmouseup() {
    return this[GET_LISTENER]('onmouseup')
  }

  set onmouseup(listener) {
    this[SET_LISTENER]('onmouseup', listener)
  }
  
  get onmousewheel() {
    return this[GET_LISTENER]('onmousewheel')
  }

  set onmousewheel(listener) {
    this[SET_LISTENER]('onmousewheel', listener)
  }
  
  get onoffline() {
    return this[GET_LISTENER]('onoffline')
  }

  set onoffline(listener) {
    this[SET_LISTENER]('onoffline', listener)
  }
  
  get ononline() {
    return this[GET_LISTENER]('ononline')
  }

  set ononline(listener) {
    this[SET_LISTENER]('ononline', listener)
  }
  
  get onorientationchange() {
    return this[GET_LISTENER]('onorientationchange')
  }

  set onorientationchange(listener) {
    this[SET_LISTENER]('onorientationchange', listener)
  }
  
  get onpagehide() {
    return this[GET_LISTENER]('onpagehide')
  }

  set onpagehide(listener) {
    this[SET_LISTENER]('onpagehide', listener)
  }
  
  get onpageshow() {
    return this[GET_LISTENER]('onpageshow')
  }

  set onpageshow(listener) {
    this[SET_LISTENER]('onpageshow', listener)
  }
  
  get onpaste() {
    return this[GET_LISTENER]('onpaste')
  }

  set onpaste(listener) {
    this[SET_LISTENER]('onpaste', listener)
  }
  
  get onpause() {
    return this[GET_LISTENER]('onpause')
  }

  set onpause(listener) {
    this[SET_LISTENER]('onpause', listener)
  }
  
  get onplay() {
    return this[GET_LISTENER]('onplay')
  }

  set onplay(listener) {
    this[SET_LISTENER]('onplay', listener)
  }
  
  get onplaying() {
    return this[GET_LISTENER]('onplaying')
  }

  set onplaying(listener) {
    this[SET_LISTENER]('onplaying', listener)
  }
  
  get onpointercancel() {
    return this[GET_LISTENER]('onpointercancel')
  }

  set onpointercancel(listener) {
    this[SET_LISTENER]('onpointercancel', listener)
  }
  
  get onpointerdown() {
    return this[GET_LISTENER]('onpointerdown')
  }

  set onpointerdown(listener) {
    this[SET_LISTENER]('onpointerdown', listener)
  }
  
  get onpointerenter() {
    return this[GET_LISTENER]('onpointerenter')
  }

  set onpointerenter(listener) {
    this[SET_LISTENER]('onpointerenter', listener)
  }
  
  get onpointerleave() {
    return this[GET_LISTENER]('onpointerleave')
  }

  set onpointerleave(listener) {
    this[SET_LISTENER]('onpointerleave', listener)
  }
  
  get onpointerlockchange() {
    return this[GET_LISTENER]('onpointerlockchange')
  }

  set onpointerlockchange(listener) {
    this[SET_LISTENER]('onpointerlockchange', listener)
  }
  
  get onpointerlockerror() {
    return this[GET_LISTENER]('onpointerlockerror')
  }

  set onpointerlockerror(listener) {
    this[SET_LISTENER]('onpointerlockerror', listener)
  }
  
  get onpointermove() {
    return this[GET_LISTENER]('onpointermove')
  }

  set onpointermove(listener) {
    this[SET_LISTENER]('onpointermove', listener)
  }
  
  get onpointerout() {
    return this[GET_LISTENER]('onpointerout')
  }

  set onpointerout(listener) {
    this[SET_LISTENER]('onpointerout', listener)
  }
  
  get onpointerover() {
    return this[GET_LISTENER]('onpointerover')
  }

  set onpointerover(listener) {
    this[SET_LISTENER]('onpointerover', listener)
  }
  
  get onpointerup() {
    return this[GET_LISTENER]('onpointerup')
  }

  set onpointerup(listener) {
    this[SET_LISTENER]('onpointerup', listener)
  }
  
  get onpopstate() {
    return this[GET_LISTENER]('onpopstate')
  }

  set onpopstate(listener) {
    this[SET_LISTENER]('onpopstate', listener)
  }
  
  get onprogress() {
    return this[GET_LISTENER]('onprogress')
  }

  set onprogress(listener) {
    this[SET_LISTENER]('onprogress', listener)
  }
  
  get onratechange() {
    return this[GET_LISTENER]('onratechange')
  }

  set onratechange(listener) {
    this[SET_LISTENER]('onratechange', listener)
  }
  
  get onreadystatechange() {
    return this[GET_LISTENER]('onreadystatechange')
  }

  set onreadystatechange(listener) {
    this[SET_LISTENER]('onreadystatechange', listener)
  }
  
  get onrejectionhandled() {
    return this[GET_LISTENER]('onrejectionhandled')
  }

  set onrejectionhandled(listener) {
    this[SET_LISTENER]('onrejectionhandled', listener)
  }
  
  get onreset() {
    return this[GET_LISTENER]('onreset')
  }

  set onreset(listener) {
    this[SET_LISTENER]('onreset', listener)
  }
  
  get onresize() {
    return this[GET_LISTENER]('onresize')
  }

  set onresize(listener) {
    this[SET_LISTENER]('onresize', listener)
  }
  
  get onscroll() {
    return this[GET_LISTENER]('onscroll')
  }

  set onscroll(listener) {
    this[SET_LISTENER]('onscroll', listener)
  }
  
  get onsecuritypolicyviolation() {
    return this[GET_LISTENER]('onsecuritypolicyviolation')
  }

  set onsecuritypolicyviolation(listener) {
    this[SET_LISTENER]('onsecuritypolicyviolation', listener)
  }
  
  get onseeked() {
    return this[GET_LISTENER]('onseeked')
  }

  set onseeked(listener) {
    this[SET_LISTENER]('onseeked', listener)
  }
  
  get onseeking() {
    return this[GET_LISTENER]('onseeking')
  }

  set onseeking(listener) {
    this[SET_LISTENER]('onseeking', listener)
  }
  
  get onselect() {
    return this[GET_LISTENER]('onselect')
  }

  set onselect(listener) {
    this[SET_LISTENER]('onselect', listener)
  }
  
  get onselectionchange() {
    return this[GET_LISTENER]('onselectionchange')
  }

  set onselectionchange(listener) {
    this[SET_LISTENER]('onselectionchange', listener)
  }
  
  get onselectstart() {
    return this[GET_LISTENER]('onselectstart')
  }

  set onselectstart(listener) {
    this[SET_LISTENER]('onselectstart', listener)
  }
  
  get onstalled() {
    return this[GET_LISTENER]('onstalled')
  }

  set onstalled(listener) {
    this[SET_LISTENER]('onstalled', listener)
  }
  
  get onstorage() {
    return this[GET_LISTENER]('onstorage')
  }

  set onstorage(listener) {
    this[SET_LISTENER]('onstorage', listener)
  }
  
  get onsubmit() {
    return this[GET_LISTENER]('onsubmit')
  }

  set onsubmit(listener) {
    this[SET_LISTENER]('onsubmit', listener)
  }
  
  get onsuspend() {
    return this[GET_LISTENER]('onsuspend')
  }

  set onsuspend(listener) {
    this[SET_LISTENER]('onsuspend', listener)
  }
  
  get ontimeupdate() {
    return this[GET_LISTENER]('ontimeupdate')
  }

  set ontimeupdate(listener) {
    this[SET_LISTENER]('ontimeupdate', listener)
  }
  
  get ontoggle() {
    return this[GET_LISTENER]('ontoggle')
  }

  set ontoggle(listener) {
    this[SET_LISTENER]('ontoggle', listener)
  }
  
  get ontouchcancel() {
    return this[GET_LISTENER]('ontouchcancel')
  }

  set ontouchcancel(listener) {
    this[SET_LISTENER]('ontouchcancel', listener)
  }
  
  get ontouchend() {
    return this[GET_LISTENER]('ontouchend')
  }

  set ontouchend(listener) {
    this[SET_LISTENER]('ontouchend', listener)
  }
  
  get ontouchmove() {
    return this[GET_LISTENER]('ontouchmove')
  }

  set ontouchmove(listener) {
    this[SET_LISTENER]('ontouchmove', listener)
  }
  
  get ontouchstart() {
    return this[GET_LISTENER]('ontouchstart')
  }

  set ontouchstart(listener) {
    this[SET_LISTENER]('ontouchstart', listener)
  }
  
  get ontransitioncancel() {
    return this[GET_LISTENER]('ontransitioncancel')
  }

  set ontransitioncancel(listener) {
    this[SET_LISTENER]('ontransitioncancel', listener)
  }
  
  get ontransitionend() {
    return this[GET_LISTENER]('ontransitionend')
  }

  set ontransitionend(listener) {
    this[SET_LISTENER]('ontransitionend', listener)
  }
  
  get ontransitionrun() {
    return this[GET_LISTENER]('ontransitionrun')
  }

  set ontransitionrun(listener) {
    this[SET_LISTENER]('ontransitionrun', listener)
  }
  
  get ontransitionstart() {
    return this[GET_LISTENER]('ontransitionstart')
  }

  set ontransitionstart(listener) {
    this[SET_LISTENER]('ontransitionstart', listener)
  }
  
  get onunhandledrejection() {
    return this[GET_LISTENER]('onunhandledrejection')
  }

  set onunhandledrejection(listener) {
    this[SET_LISTENER]('onunhandledrejection', listener)
  }
  
  get onunload() {
    return this[GET_LISTENER]('onunload')
  }

  set onunload(listener) {
    this[SET_LISTENER]('onunload', listener)
  }
  
  get onvisibilitychange() {
    return this[GET_LISTENER]('onvisibilitychange')
  }

  set onvisibilitychange(listener) {
    this[SET_LISTENER]('onvisibilitychange', listener)
  }
  
  get onvolumechange() {
    return this[GET_LISTENER]('onvolumechange')
  }

  set onvolumechange(listener) {
    this[SET_LISTENER]('onvolumechange', listener)
  }
  
  get onvrdisplayactivate() {
    return this[GET_LISTENER]('onvrdisplayactivate')
  }

  set onvrdisplayactivate(listener) {
    this[SET_LISTENER]('onvrdisplayactivate', listener)
  }
  
  get onvrdisplayblur() {
    return this[GET_LISTENER]('onvrdisplayblur')
  }

  set onvrdisplayblur(listener) {
    this[SET_LISTENER]('onvrdisplayblur', listener)
  }
  
  get onvrdisplayconnect() {
    return this[GET_LISTENER]('onvrdisplayconnect')
  }

  set onvrdisplayconnect(listener) {
    this[SET_LISTENER]('onvrdisplayconnect', listener)
  }
  
  get onvrdisplaydeactivate() {
    return this[GET_LISTENER]('onvrdisplaydeactivate')
  }

  set onvrdisplaydeactivate(listener) {
    this[SET_LISTENER]('onvrdisplaydeactivate', listener)
  }
  
  get onvrdisplaydisconnect() {
    return this[GET_LISTENER]('onvrdisplaydisconnect')
  }

  set onvrdisplaydisconnect(listener) {
    this[SET_LISTENER]('onvrdisplaydisconnect', listener)
  }
  
  get onvrdisplayfocus() {
    return this[GET_LISTENER]('onvrdisplayfocus')
  }

  set onvrdisplayfocus(listener) {
    this[SET_LISTENER]('onvrdisplayfocus', listener)
  }
  
  get onvrdisplaypointerrestricted() {
    return this[GET_LISTENER]('onvrdisplaypointerrestricted')
  }

  set onvrdisplaypointerrestricted(listener) {
    this[SET_LISTENER]('onvrdisplaypointerrestricted', listener)
  }
  
  get onvrdisplaypointerunrestricted() {
    return this[GET_LISTENER]('onvrdisplaypointerunrestricted')
  }

  set onvrdisplaypointerunrestricted(listener) {
    this[SET_LISTENER]('onvrdisplaypointerunrestricted', listener)
  }
  
  get onvrdisplaypresentchange() {
    return this[GET_LISTENER]('onvrdisplaypresentchange')
  }

  set onvrdisplaypresentchange(listener) {
    this[SET_LISTENER]('onvrdisplaypresentchange', listener)
  }
  
  get onwaiting() {
    return this[GET_LISTENER]('onwaiting')
  }

  set onwaiting(listener) {
    this[SET_LISTENER]('onwaiting', listener)
  }
  
  get onwheel() {
    return this[GET_LISTENER]('onwheel')
  }

  set onwheel(listener) {
    this[SET_LISTENER]('onwheel', listener)
  }
  
  get onzoom() {
    return this[GET_LISTENER]('onzoom')
  }

  set onzoom(listener) {
    this[SET_LISTENER]('onzoom', listener)
  }

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

const GET_LISTENER = Symbol()
const SET_LISTENER = Symbol()

class InlineListener {
  constructor(public handleEvent) {}
}
