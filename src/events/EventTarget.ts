import { Event } from '../events/Event'

export const GET_THE_PARENT = Symbol()
const GET_LISTENER = Symbol()
const SET_LISTENER = Symbol()

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
    for (const l of this.#listeners[type]) {
      if (l instanceof InlineListener) {
        return l.handleEvent
      }
    }

    return null
  }

  [SET_LISTENER](type: string, listener) {
    const listeners = this.#listeners[type]
    const index = listeners.findIndex(l => l instanceof InlineListener)

    if (~index) {
      this.removeEventListener(type, listeners[index])
    }

    this.addEventListener(type, new InlineListener(listener))
  }

  // on* event handler props
  // - makes TS happy
  // - everything is here so we don't need to repeat it again in all possible ev targets
  // - preact needs this for some golfing: name = (nameLower in dom ? nameLower : name).slice(2);
  //   https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80

  get onabort() {
    return this[GET_LISTENER]('abort')
  }

  set onabort(listener) {
    this[SET_LISTENER]('abort', listener)
  }

  get onafterprint() {
    return this[GET_LISTENER]('afterprint')
  }

  set onafterprint(listener) {
    this[SET_LISTENER]('afterprint', listener)
  }

  get onanimationcancel() {
    return this[GET_LISTENER]('animationcancel')
  }

  set onanimationcancel(listener) {
    this[SET_LISTENER]('animationcancel', listener)
  }

  get onanimationend() {
    return this[GET_LISTENER]('animationend')
  }

  set onanimationend(listener) {
    this[SET_LISTENER]('animationend', listener)
  }

  get onanimationiteration() {
    return this[GET_LISTENER]('animationiteration')
  }

  set onanimationiteration(listener) {
    this[SET_LISTENER]('animationiteration', listener)
  }

  get onanimationstart() {
    return this[GET_LISTENER]('animationstart')
  }

  set onanimationstart(listener) {
    this[SET_LISTENER]('animationstart', listener)
  }

  get onauxclick() {
    return this[GET_LISTENER]('auxclick')
  }

  set onauxclick(listener) {
    this[SET_LISTENER]('auxclick', listener)
  }

  get onbeforeprint() {
    return this[GET_LISTENER]('beforeprint')
  }

  set onbeforeprint(listener) {
    this[SET_LISTENER]('beforeprint', listener)
  }

  get onbeforeunload() {
    return this[GET_LISTENER]('beforeunload')
  }

  set onbeforeunload(listener) {
    this[SET_LISTENER]('beforeunload', listener)
  }

  get onblur() {
    return this[GET_LISTENER]('blur')
  }

  set onblur(listener) {
    this[SET_LISTENER]('blur', listener)
  }

  get oncancel() {
    return this[GET_LISTENER]('cancel')
  }

  set oncancel(listener) {
    this[SET_LISTENER]('cancel', listener)
  }

  get oncanplay() {
    return this[GET_LISTENER]('canplay')
  }

  set oncanplay(listener) {
    this[SET_LISTENER]('canplay', listener)
  }

  get oncanplaythrough() {
    return this[GET_LISTENER]('canplaythrough')
  }

  set oncanplaythrough(listener) {
    this[SET_LISTENER]('canplaythrough', listener)
  }

  get onchange() {
    return this[GET_LISTENER]('change')
  }

  set onchange(listener) {
    this[SET_LISTENER]('change', listener)
  }

  get onclick() {
    return this[GET_LISTENER]('click')
  }

  set onclick(listener) {
    this[SET_LISTENER]('click', listener)
  }

  get onclose() {
    return this[GET_LISTENER]('close')
  }

  set onclose(listener) {
    this[SET_LISTENER]('close', listener)
  }

  get oncompassneedscalibration() {
    return this[GET_LISTENER]('compassneedscalibration')
  }

  set oncompassneedscalibration(listener) {
    this[SET_LISTENER]('compassneedscalibration', listener)
  }

  get oncontextmenu() {
    return this[GET_LISTENER]('contextmenu')
  }

  set oncontextmenu(listener) {
    this[SET_LISTENER]('contextmenu', listener)
  }

  get oncopy() {
    return this[GET_LISTENER]('copy')
  }

  set oncopy(listener) {
    this[SET_LISTENER]('copy', listener)
  }

  get oncuechange() {
    return this[GET_LISTENER]('cuechange')
  }

  set oncuechange(listener) {
    this[SET_LISTENER]('cuechange', listener)
  }

  get oncut() {
    return this[GET_LISTENER]('cut')
  }

  set oncut(listener) {
    this[SET_LISTENER]('cut', listener)
  }

  get ondblclick() {
    return this[GET_LISTENER]('dblclick')
  }

  set ondblclick(listener) {
    this[SET_LISTENER]('dblclick', listener)
  }

  get ondevicelight() {
    return this[GET_LISTENER]('devicelight')
  }

  set ondevicelight(listener) {
    this[SET_LISTENER]('devicelight', listener)
  }

  get ondevicemotion() {
    return this[GET_LISTENER]('devicemotion')
  }

  set ondevicemotion(listener) {
    this[SET_LISTENER]('devicemotion', listener)
  }

  get ondeviceorientation() {
    return this[GET_LISTENER]('deviceorientation')
  }

  set ondeviceorientation(listener) {
    this[SET_LISTENER]('deviceorientation', listener)
  }

  get ondeviceorientationabsolute() {
    return this[GET_LISTENER]('deviceorientationabsolute')
  }

  set ondeviceorientationabsolute(listener) {
    this[SET_LISTENER]('deviceorientationabsolute', listener)
  }

  get ondrag() {
    return this[GET_LISTENER]('drag')
  }

  set ondrag(listener) {
    this[SET_LISTENER]('drag', listener)
  }

  get ondragend() {
    return this[GET_LISTENER]('dragend')
  }

  set ondragend(listener) {
    this[SET_LISTENER]('dragend', listener)
  }

  get ondragenter() {
    return this[GET_LISTENER]('dragenter')
  }

  set ondragenter(listener) {
    this[SET_LISTENER]('dragenter', listener)
  }

  get ondragexit() {
    return this[GET_LISTENER]('dragexit')
  }

  set ondragexit(listener) {
    this[SET_LISTENER]('dragexit', listener)
  }

  get ondragleave() {
    return this[GET_LISTENER]('dragleave')
  }

  set ondragleave(listener) {
    this[SET_LISTENER]('dragleave', listener)
  }

  get ondragover() {
    return this[GET_LISTENER]('dragover')
  }

  set ondragover(listener) {
    this[SET_LISTENER]('dragover', listener)
  }

  get ondragstart() {
    return this[GET_LISTENER]('dragstart')
  }

  set ondragstart(listener) {
    this[SET_LISTENER]('dragstart', listener)
  }

  get ondrop() {
    return this[GET_LISTENER]('drop')
  }

  set ondrop(listener) {
    this[SET_LISTENER]('drop', listener)
  }

  get ondurationchange() {
    return this[GET_LISTENER]('durationchange')
  }

  set ondurationchange(listener) {
    this[SET_LISTENER]('durationchange', listener)
  }

  get onemptied() {
    return this[GET_LISTENER]('emptied')
  }

  set onemptied(listener) {
    this[SET_LISTENER]('emptied', listener)
  }

  get onended() {
    return this[GET_LISTENER]('ended')
  }

  set onended(listener) {
    this[SET_LISTENER]('ended', listener)
  }

  get onerror() {
    return this[GET_LISTENER]('error')
  }

  set onerror(listener) {
    this[SET_LISTENER]('error', listener)
  }

  get onfocus() {
    return this[GET_LISTENER]('focus')
  }

  set onfocus(listener) {
    this[SET_LISTENER]('focus', listener)
  }

  get onfullscreenchange() {
    return this[GET_LISTENER]('fullscreenchange')
  }

  set onfullscreenchange(listener) {
    this[SET_LISTENER]('fullscreenchange', listener)
  }

  get onfullscreenerror() {
    return this[GET_LISTENER]('fullscreenerror')
  }

  set onfullscreenerror(listener) {
    this[SET_LISTENER]('fullscreenerror', listener)
  }

  get ongamepadconnected() {
    return this[GET_LISTENER]('gamepadconnected')
  }

  set ongamepadconnected(listener) {
    this[SET_LISTENER]('gamepadconnected', listener)
  }

  get ongamepaddisconnected() {
    return this[GET_LISTENER]('gamepaddisconnected')
  }

  set ongamepaddisconnected(listener) {
    this[SET_LISTENER]('gamepaddisconnected', listener)
  }

  get ongotpointercapture() {
    return this[GET_LISTENER]('gotpointercapture')
  }

  set ongotpointercapture(listener) {
    this[SET_LISTENER]('gotpointercapture', listener)
  }

  get onhashchange() {
    return this[GET_LISTENER]('hashchange')
  }

  set onhashchange(listener) {
    this[SET_LISTENER]('hashchange', listener)
  }

  get oninput() {
    return this[GET_LISTENER]('input')
  }

  set oninput(listener) {
    this[SET_LISTENER]('input', listener)
  }

  get oninvalid() {
    return this[GET_LISTENER]('invalid')
  }

  set oninvalid(listener) {
    this[SET_LISTENER]('invalid', listener)
  }

  get onkeydown() {
    return this[GET_LISTENER]('keydown')
  }

  set onkeydown(listener) {
    this[SET_LISTENER]('keydown', listener)
  }

  get onkeypress() {
    return this[GET_LISTENER]('keypress')
  }

  set onkeypress(listener) {
    this[SET_LISTENER]('keypress', listener)
  }

  get onkeyup() {
    return this[GET_LISTENER]('keyup')
  }

  set onkeyup(listener) {
    this[SET_LISTENER]('keyup', listener)
  }

  get onlanguagechange() {
    return this[GET_LISTENER]('languagechange')
  }

  set onlanguagechange(listener) {
    this[SET_LISTENER]('languagechange', listener)
  }

  get onload() {
    return this[GET_LISTENER]('load')
  }

  set onload(listener) {
    this[SET_LISTENER]('load', listener)
  }

  get onloadeddata() {
    return this[GET_LISTENER]('loadeddata')
  }

  set onloadeddata(listener) {
    this[SET_LISTENER]('loadeddata', listener)
  }

  get onloadedmetadata() {
    return this[GET_LISTENER]('loadedmetadata')
  }

  set onloadedmetadata(listener) {
    this[SET_LISTENER]('loadedmetadata', listener)
  }

  get onloadstart() {
    return this[GET_LISTENER]('loadstart')
  }

  set onloadstart(listener) {
    this[SET_LISTENER]('loadstart', listener)
  }

  get onlostpointercapture() {
    return this[GET_LISTENER]('lostpointercapture')
  }

  set onlostpointercapture(listener) {
    this[SET_LISTENER]('lostpointercapture', listener)
  }

  get onmessage() {
    return this[GET_LISTENER]('message')
  }

  set onmessage(listener) {
    this[SET_LISTENER]('message', listener)
  }

  get onmessageerror() {
    return this[GET_LISTENER]('messageerror')
  }

  set onmessageerror(listener) {
    this[SET_LISTENER]('messageerror', listener)
  }

  get onmousedown() {
    return this[GET_LISTENER]('mousedown')
  }

  set onmousedown(listener) {
    this[SET_LISTENER]('mousedown', listener)
  }

  get onmouseenter() {
    return this[GET_LISTENER]('mouseenter')
  }

  set onmouseenter(listener) {
    this[SET_LISTENER]('mouseenter', listener)
  }

  get onmouseleave() {
    return this[GET_LISTENER]('mouseleave')
  }

  set onmouseleave(listener) {
    this[SET_LISTENER]('mouseleave', listener)
  }

  get onmousemove() {
    return this[GET_LISTENER]('mousemove')
  }

  set onmousemove(listener) {
    this[SET_LISTENER]('mousemove', listener)
  }

  get onmouseout() {
    return this[GET_LISTENER]('mouseout')
  }

  set onmouseout(listener) {
    this[SET_LISTENER]('mouseout', listener)
  }

  get onmouseover() {
    return this[GET_LISTENER]('mouseover')
  }

  set onmouseover(listener) {
    this[SET_LISTENER]('mouseover', listener)
  }

  get onmouseup() {
    return this[GET_LISTENER]('mouseup')
  }

  set onmouseup(listener) {
    this[SET_LISTENER]('mouseup', listener)
  }

  get onmousewheel() {
    return this[GET_LISTENER]('mousewheel')
  }

  set onmousewheel(listener) {
    this[SET_LISTENER]('mousewheel', listener)
  }

  get onoffline() {
    return this[GET_LISTENER]('offline')
  }

  set onoffline(listener) {
    this[SET_LISTENER]('offline', listener)
  }

  get ononline() {
    return this[GET_LISTENER]('online')
  }

  set ononline(listener) {
    this[SET_LISTENER]('online', listener)
  }

  get onorientationchange() {
    return this[GET_LISTENER]('orientationchange')
  }

  set onorientationchange(listener) {
    this[SET_LISTENER]('orientationchange', listener)
  }

  get onpagehide() {
    return this[GET_LISTENER]('pagehide')
  }

  set onpagehide(listener) {
    this[SET_LISTENER]('pagehide', listener)
  }

  get onpageshow() {
    return this[GET_LISTENER]('pageshow')
  }

  set onpageshow(listener) {
    this[SET_LISTENER]('pageshow', listener)
  }

  get onpaste() {
    return this[GET_LISTENER]('paste')
  }

  set onpaste(listener) {
    this[SET_LISTENER]('paste', listener)
  }

  get onpause() {
    return this[GET_LISTENER]('pause')
  }

  set onpause(listener) {
    this[SET_LISTENER]('pause', listener)
  }

  get onplay() {
    return this[GET_LISTENER]('play')
  }

  set onplay(listener) {
    this[SET_LISTENER]('play', listener)
  }

  get onplaying() {
    return this[GET_LISTENER]('playing')
  }

  set onplaying(listener) {
    this[SET_LISTENER]('playing', listener)
  }

  get onpointercancel() {
    return this[GET_LISTENER]('pointercancel')
  }

  set onpointercancel(listener) {
    this[SET_LISTENER]('pointercancel', listener)
  }

  get onpointerdown() {
    return this[GET_LISTENER]('pointerdown')
  }

  set onpointerdown(listener) {
    this[SET_LISTENER]('pointerdown', listener)
  }

  get onpointerenter() {
    return this[GET_LISTENER]('pointerenter')
  }

  set onpointerenter(listener) {
    this[SET_LISTENER]('pointerenter', listener)
  }

  get onpointerleave() {
    return this[GET_LISTENER]('pointerleave')
  }

  set onpointerleave(listener) {
    this[SET_LISTENER]('pointerleave', listener)
  }

  get onpointerlockchange() {
    return this[GET_LISTENER]('pointerlockchange')
  }

  set onpointerlockchange(listener) {
    this[SET_LISTENER]('pointerlockchange', listener)
  }

  get onpointerlockerror() {
    return this[GET_LISTENER]('pointerlockerror')
  }

  set onpointerlockerror(listener) {
    this[SET_LISTENER]('pointerlockerror', listener)
  }

  get onpointermove() {
    return this[GET_LISTENER]('pointermove')
  }

  set onpointermove(listener) {
    this[SET_LISTENER]('pointermove', listener)
  }

  get onpointerout() {
    return this[GET_LISTENER]('pointerout')
  }

  set onpointerout(listener) {
    this[SET_LISTENER]('pointerout', listener)
  }

  get onpointerover() {
    return this[GET_LISTENER]('pointerover')
  }

  set onpointerover(listener) {
    this[SET_LISTENER]('pointerover', listener)
  }

  get onpointerup() {
    return this[GET_LISTENER]('pointerup')
  }

  set onpointerup(listener) {
    this[SET_LISTENER]('pointerup', listener)
  }

  get onpopstate() {
    return this[GET_LISTENER]('popstate')
  }

  set onpopstate(listener) {
    this[SET_LISTENER]('popstate', listener)
  }

  get onprogress() {
    return this[GET_LISTENER]('progress')
  }

  set onprogress(listener) {
    this[SET_LISTENER]('progress', listener)
  }

  get onratechange() {
    return this[GET_LISTENER]('ratechange')
  }

  set onratechange(listener) {
    this[SET_LISTENER]('ratechange', listener)
  }

  get onreadystatechange() {
    return this[GET_LISTENER]('readystatechange')
  }

  set onreadystatechange(listener) {
    this[SET_LISTENER]('readystatechange', listener)
  }

  get onrejectionhandled() {
    return this[GET_LISTENER]('rejectionhandled')
  }

  set onrejectionhandled(listener) {
    this[SET_LISTENER]('rejectionhandled', listener)
  }

  get onreset() {
    return this[GET_LISTENER]('reset')
  }

  set onreset(listener) {
    this[SET_LISTENER]('reset', listener)
  }

  get onresize() {
    return this[GET_LISTENER]('resize')
  }

  set onresize(listener) {
    this[SET_LISTENER]('resize', listener)
  }

  get onscroll() {
    return this[GET_LISTENER]('scroll')
  }

  set onscroll(listener) {
    this[SET_LISTENER]('scroll', listener)
  }

  get onsecuritypolicyviolation() {
    return this[GET_LISTENER]('securitypolicyviolation')
  }

  set onsecuritypolicyviolation(listener) {
    this[SET_LISTENER]('securitypolicyviolation', listener)
  }

  get onseeked() {
    return this[GET_LISTENER]('seeked')
  }

  set onseeked(listener) {
    this[SET_LISTENER]('seeked', listener)
  }

  get onseeking() {
    return this[GET_LISTENER]('seeking')
  }

  set onseeking(listener) {
    this[SET_LISTENER]('seeking', listener)
  }

  get onselect() {
    return this[GET_LISTENER]('select')
  }

  set onselect(listener) {
    this[SET_LISTENER]('select', listener)
  }

  get onselectionchange() {
    return this[GET_LISTENER]('selectionchange')
  }

  set onselectionchange(listener) {
    this[SET_LISTENER]('selectionchange', listener)
  }

  get onselectstart() {
    return this[GET_LISTENER]('selectstart')
  }

  set onselectstart(listener) {
    this[SET_LISTENER]('selectstart', listener)
  }

  get onstalled() {
    return this[GET_LISTENER]('stalled')
  }

  set onstalled(listener) {
    this[SET_LISTENER]('stalled', listener)
  }

  get onstorage() {
    return this[GET_LISTENER]('storage')
  }

  set onstorage(listener) {
    this[SET_LISTENER]('storage', listener)
  }

  get onsubmit() {
    return this[GET_LISTENER]('submit')
  }

  set onsubmit(listener) {
    this[SET_LISTENER]('submit', listener)
  }

  get onsuspend() {
    return this[GET_LISTENER]('suspend')
  }

  set onsuspend(listener) {
    this[SET_LISTENER]('suspend', listener)
  }

  get ontimeupdate() {
    return this[GET_LISTENER]('timeupdate')
  }

  set ontimeupdate(listener) {
    this[SET_LISTENER]('timeupdate', listener)
  }

  get ontoggle() {
    return this[GET_LISTENER]('toggle')
  }

  set ontoggle(listener) {
    this[SET_LISTENER]('toggle', listener)
  }

  get ontouchcancel() {
    return this[GET_LISTENER]('touchcancel')
  }

  set ontouchcancel(listener) {
    this[SET_LISTENER]('touchcancel', listener)
  }

  get ontouchend() {
    return this[GET_LISTENER]('touchend')
  }

  set ontouchend(listener) {
    this[SET_LISTENER]('touchend', listener)
  }

  get ontouchmove() {
    return this[GET_LISTENER]('touchmove')
  }

  set ontouchmove(listener) {
    this[SET_LISTENER]('touchmove', listener)
  }

  get ontouchstart() {
    return this[GET_LISTENER]('touchstart')
  }

  set ontouchstart(listener) {
    this[SET_LISTENER]('touchstart', listener)
  }

  get ontransitioncancel() {
    return this[GET_LISTENER]('transitioncancel')
  }

  set ontransitioncancel(listener) {
    this[SET_LISTENER]('transitioncancel', listener)
  }

  get ontransitionend() {
    return this[GET_LISTENER]('transitionend')
  }

  set ontransitionend(listener) {
    this[SET_LISTENER]('transitionend', listener)
  }

  get ontransitionrun() {
    return this[GET_LISTENER]('transitionrun')
  }

  set ontransitionrun(listener) {
    this[SET_LISTENER]('transitionrun', listener)
  }

  get ontransitionstart() {
    return this[GET_LISTENER]('transitionstart')
  }

  set ontransitionstart(listener) {
    this[SET_LISTENER]('transitionstart', listener)
  }

  get onunhandledrejection() {
    return this[GET_LISTENER]('unhandledrejection')
  }

  set onunhandledrejection(listener) {
    this[SET_LISTENER]('unhandledrejection', listener)
  }

  get onunload() {
    return this[GET_LISTENER]('unload')
  }

  set onunload(listener) {
    this[SET_LISTENER]('unload', listener)
  }

  get onvisibilitychange() {
    return this[GET_LISTENER]('visibilitychange')
  }

  set onvisibilitychange(listener) {
    this[SET_LISTENER]('visibilitychange', listener)
  }

  get onvolumechange() {
    return this[GET_LISTENER]('volumechange')
  }

  set onvolumechange(listener) {
    this[SET_LISTENER]('volumechange', listener)
  }

  get onvrdisplayactivate() {
    return this[GET_LISTENER]('vrdisplayactivate')
  }

  set onvrdisplayactivate(listener) {
    this[SET_LISTENER]('vrdisplayactivate', listener)
  }

  get onvrdisplayblur() {
    return this[GET_LISTENER]('vrdisplayblur')
  }

  set onvrdisplayblur(listener) {
    this[SET_LISTENER]('vrdisplayblur', listener)
  }

  get onvrdisplayconnect() {
    return this[GET_LISTENER]('vrdisplayconnect')
  }

  set onvrdisplayconnect(listener) {
    this[SET_LISTENER]('vrdisplayconnect', listener)
  }

  get onvrdisplaydeactivate() {
    return this[GET_LISTENER]('vrdisplaydeactivate')
  }

  set onvrdisplaydeactivate(listener) {
    this[SET_LISTENER]('vrdisplaydeactivate', listener)
  }

  get onvrdisplaydisconnect() {
    return this[GET_LISTENER]('vrdisplaydisconnect')
  }

  set onvrdisplaydisconnect(listener) {
    this[SET_LISTENER]('vrdisplaydisconnect', listener)
  }

  get onvrdisplayfocus() {
    return this[GET_LISTENER]('vrdisplayfocus')
  }

  set onvrdisplayfocus(listener) {
    this[SET_LISTENER]('vrdisplayfocus', listener)
  }

  get onvrdisplaypointerrestricted() {
    return this[GET_LISTENER]('vrdisplaypointerrestricted')
  }

  set onvrdisplaypointerrestricted(listener) {
    this[SET_LISTENER]('vrdisplaypointerrestricted', listener)
  }

  get onvrdisplaypointerunrestricted() {
    return this[GET_LISTENER]('vrdisplaypointerunrestricted')
  }

  set onvrdisplaypointerunrestricted(listener) {
    this[SET_LISTENER]('vrdisplaypointerunrestricted', listener)
  }

  get onvrdisplaypresentchange() {
    return this[GET_LISTENER]('vrdisplaypresentchange')
  }

  set onvrdisplaypresentchange(listener) {
    this[SET_LISTENER]('vrdisplaypresentchange', listener)
  }

  get onwaiting() {
    return this[GET_LISTENER]('waiting')
  }

  set onwaiting(listener) {
    this[SET_LISTENER]('waiting', listener)
  }

  get onwheel() {
    return this[GET_LISTENER]('wheel')
  }

  set onwheel(listener) {
    this[SET_LISTENER]('wheel', listener)
  }

  get onzoom() {
    return this[GET_LISTENER]('zoom')
  }

  set onzoom(listener) {
    this[SET_LISTENER]('zoom', listener)
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
  onmspointerup;

  // WTF
  [index: number]: Window
}

class InlineListener {
  constructor(public handleEvent) {}
}
