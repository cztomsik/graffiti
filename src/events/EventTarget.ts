import { isDeno } from '../util'
import { CANCEL_BUBBLE_IMMEDIATELY } from './Event'

// note that we are using provided EventTarget if possible (deno)
class EventTarget implements globalThis.EventTarget {
  // beware collisions, preact is using node._listeners
  #listeners = {}

  addEventListener(type: string, listener) {
    this.#listeners[type] = [...this.#listeners[type] ?? [], listener]
  }

  removeEventListener(type: string, listener) {
    this.#listeners[type] = (this.#listeners[type] ?? []).filter(l => l !== listener)
  }

  dispatchEvent(event: Event) {
    ;(event as any).target = this

    this._dispatch(event)

    return !event.defaultPrevented
  }

  // TODO: inline in dispatchEvent but it MUST NOT set event.target during bubbling
  _dispatch(event: Event) {
    ;(event as any).currentTarget = this

    for (const l of this.#listeners[event.type] ?? []) {
      if ('handleEvent' in l) {
        l.handleEvent(event)
      } else {
        l.call(this, event)
      }

      if (event[CANCEL_BUBBLE_IMMEDIATELY]) {
        break
      }
    }

    if (!event.cancelBubble) {
      // bubble
      this['parentNode']?._dispatch(event)
    }
  }
}

// prefer deno EventTarget
const BaseEventTarget: typeof EventTarget = isDeno ? (globalThis.EventTarget as any) : EventTarget

const INLINE_HANDLERS = Symbol()

// define new class for our purposes
class EventTargetWithHandlerProps extends BaseEventTarget {
  [INLINE_HANDLERS] = {}

  // on* event handler props
  // - makes TS happy
  // - everything is here so we don't need to repeat it again in all possible ev targets
  // - preact needs this for some golfing: name = (nameLower in dom ? nameLower : name).slice(2);
  //   https://github.com/preactjs/preact/blob/013dc382cf7239422e834e74a6ab0b592c5a9c43/src/diff/props.js#L80

  get onabort() {
    return getHandler(this, 'abort')
  }

  set onabort(listener) {
    setHandler(this, 'abort', listener)
  }

  get onafterprint() {
    return getHandler(this, 'afterprint')
  }

  set onafterprint(listener) {
    setHandler(this, 'afterprint', listener)
  }

  get onanimationcancel() {
    return getHandler(this, 'animationcancel')
  }

  set onanimationcancel(listener) {
    setHandler(this, 'animationcancel', listener)
  }

  get onanimationend() {
    return getHandler(this, 'animationend')
  }

  set onanimationend(listener) {
    setHandler(this, 'animationend', listener)
  }

  get onanimationiteration() {
    return getHandler(this, 'animationiteration')
  }

  set onanimationiteration(listener) {
    setHandler(this, 'animationiteration', listener)
  }

  get onanimationstart() {
    return getHandler(this, 'animationstart')
  }

  set onanimationstart(listener) {
    setHandler(this, 'animationstart', listener)
  }

  get onauxclick() {
    return getHandler(this, 'auxclick')
  }

  set onauxclick(listener) {
    setHandler(this, 'auxclick', listener)
  }

  get onbeforeprint() {
    return getHandler(this, 'beforeprint')
  }

  set onbeforeprint(listener) {
    setHandler(this, 'beforeprint', listener)
  }

  get onbeforeunload() {
    return getHandler(this, 'beforeunload')
  }

  set onbeforeunload(listener) {
    setHandler(this, 'beforeunload', listener)
  }

  get onblur() {
    return getHandler(this, 'blur')
  }

  set onblur(listener) {
    setHandler(this, 'blur', listener)
  }

  get oncancel() {
    return getHandler(this, 'cancel')
  }

  set oncancel(listener) {
    setHandler(this, 'cancel', listener)
  }

  get oncanplay() {
    return getHandler(this, 'canplay')
  }

  set oncanplay(listener) {
    setHandler(this, 'canplay', listener)
  }

  get oncanplaythrough() {
    return getHandler(this, 'canplaythrough')
  }

  set oncanplaythrough(listener) {
    setHandler(this, 'canplaythrough', listener)
  }

  get onchange() {
    return getHandler(this, 'change')
  }

  set onchange(listener) {
    setHandler(this, 'change', listener)
  }

  get onclick() {
    return getHandler(this, 'click')
  }

  set onclick(listener) {
    setHandler(this, 'click', listener)
  }

  get onclose() {
    return getHandler(this, 'close')
  }

  set onclose(listener) {
    setHandler(this, 'close', listener)
  }

  get oncompassneedscalibration() {
    return getHandler(this, 'compassneedscalibration')
  }

  set oncompassneedscalibration(listener) {
    setHandler(this, 'compassneedscalibration', listener)
  }

  get oncontextmenu() {
    return getHandler(this, 'contextmenu')
  }

  set oncontextmenu(listener) {
    setHandler(this, 'contextmenu', listener)
  }

  get oncopy() {
    return getHandler(this, 'copy')
  }

  set oncopy(listener) {
    setHandler(this, 'copy', listener)
  }

  get oncuechange() {
    return getHandler(this, 'cuechange')
  }

  set oncuechange(listener) {
    setHandler(this, 'cuechange', listener)
  }

  get oncut() {
    return getHandler(this, 'cut')
  }

  set oncut(listener) {
    setHandler(this, 'cut', listener)
  }

  get ondblclick() {
    return getHandler(this, 'dblclick')
  }

  set ondblclick(listener) {
    setHandler(this, 'dblclick', listener)
  }

  get ondevicelight() {
    return getHandler(this, 'devicelight')
  }

  set ondevicelight(listener) {
    setHandler(this, 'devicelight', listener)
  }

  get ondevicemotion() {
    return getHandler(this, 'devicemotion')
  }

  set ondevicemotion(listener) {
    setHandler(this, 'devicemotion', listener)
  }

  get ondeviceorientation() {
    return getHandler(this, 'deviceorientation')
  }

  set ondeviceorientation(listener) {
    setHandler(this, 'deviceorientation', listener)
  }

  get ondeviceorientationabsolute() {
    return getHandler(this, 'deviceorientationabsolute')
  }

  set ondeviceorientationabsolute(listener) {
    setHandler(this, 'deviceorientationabsolute', listener)
  }

  get ondrag() {
    return getHandler(this, 'drag')
  }

  set ondrag(listener) {
    setHandler(this, 'drag', listener)
  }

  get ondragend() {
    return getHandler(this, 'dragend')
  }

  set ondragend(listener) {
    setHandler(this, 'dragend', listener)
  }

  get ondragenter() {
    return getHandler(this, 'dragenter')
  }

  set ondragenter(listener) {
    setHandler(this, 'dragenter', listener)
  }

  get ondragexit() {
    return getHandler(this, 'dragexit')
  }

  set ondragexit(listener) {
    setHandler(this, 'dragexit', listener)
  }

  get ondragleave() {
    return getHandler(this, 'dragleave')
  }

  set ondragleave(listener) {
    setHandler(this, 'dragleave', listener)
  }

  get ondragover() {
    return getHandler(this, 'dragover')
  }

  set ondragover(listener) {
    setHandler(this, 'dragover', listener)
  }

  get ondragstart() {
    return getHandler(this, 'dragstart')
  }

  set ondragstart(listener) {
    setHandler(this, 'dragstart', listener)
  }

  get ondrop() {
    return getHandler(this, 'drop')
  }

  set ondrop(listener) {
    setHandler(this, 'drop', listener)
  }

  get ondurationchange() {
    return getHandler(this, 'durationchange')
  }

  set ondurationchange(listener) {
    setHandler(this, 'durationchange', listener)
  }

  get onemptied() {
    return getHandler(this, 'emptied')
  }

  set onemptied(listener) {
    setHandler(this, 'emptied', listener)
  }

  get onended() {
    return getHandler(this, 'ended')
  }

  set onended(listener) {
    setHandler(this, 'ended', listener)
  }

  get onerror() {
    return getHandler(this, 'error')
  }

  set onerror(listener) {
    setHandler(this, 'error', listener)
  }

  get onfocus() {
    return getHandler(this, 'focus')
  }

  set onfocus(listener) {
    setHandler(this, 'focus', listener)
  }

  get onfullscreenchange() {
    return getHandler(this, 'fullscreenchange')
  }

  set onfullscreenchange(listener) {
    setHandler(this, 'fullscreenchange', listener)
  }

  get onfullscreenerror() {
    return getHandler(this, 'fullscreenerror')
  }

  set onfullscreenerror(listener) {
    setHandler(this, 'fullscreenerror', listener)
  }

  get ongamepadconnected() {
    return getHandler(this, 'gamepadconnected')
  }

  set ongamepadconnected(listener) {
    setHandler(this, 'gamepadconnected', listener)
  }

  get ongamepaddisconnected() {
    return getHandler(this, 'gamepaddisconnected')
  }

  set ongamepaddisconnected(listener) {
    setHandler(this, 'gamepaddisconnected', listener)
  }

  get ongotpointercapture() {
    return getHandler(this, 'gotpointercapture')
  }

  set ongotpointercapture(listener) {
    setHandler(this, 'gotpointercapture', listener)
  }

  get onhashchange() {
    return getHandler(this, 'hashchange')
  }

  set onhashchange(listener) {
    setHandler(this, 'hashchange', listener)
  }

  get oninput() {
    return getHandler(this, 'input')
  }

  set oninput(listener) {
    setHandler(this, 'input', listener)
  }

  get oninvalid() {
    return getHandler(this, 'invalid')
  }

  set oninvalid(listener) {
    setHandler(this, 'invalid', listener)
  }

  get onkeydown() {
    return getHandler(this, 'keydown')
  }

  set onkeydown(listener) {
    setHandler(this, 'keydown', listener)
  }

  get onkeypress() {
    return getHandler(this, 'keypress')
  }

  set onkeypress(listener) {
    setHandler(this, 'keypress', listener)
  }

  get onkeyup() {
    return getHandler(this, 'keyup')
  }

  set onkeyup(listener) {
    setHandler(this, 'keyup', listener)
  }

  get onlanguagechange() {
    return getHandler(this, 'languagechange')
  }

  set onlanguagechange(listener) {
    setHandler(this, 'languagechange', listener)
  }

  get onload() {
    return getHandler(this, 'load')
  }

  set onload(listener) {
    setHandler(this, 'load', listener)
  }

  get onloadeddata() {
    return getHandler(this, 'loadeddata')
  }

  set onloadeddata(listener) {
    setHandler(this, 'loadeddata', listener)
  }

  get onloadedmetadata() {
    return getHandler(this, 'loadedmetadata')
  }

  set onloadedmetadata(listener) {
    setHandler(this, 'loadedmetadata', listener)
  }

  get onloadstart() {
    return getHandler(this, 'loadstart')
  }

  set onloadstart(listener) {
    setHandler(this, 'loadstart', listener)
  }

  get onlostpointercapture() {
    return getHandler(this, 'lostpointercapture')
  }

  set onlostpointercapture(listener) {
    setHandler(this, 'lostpointercapture', listener)
  }

  get onmessage() {
    return getHandler(this, 'message')
  }

  set onmessage(listener) {
    setHandler(this, 'message', listener)
  }

  get onmessageerror() {
    return getHandler(this, 'messageerror')
  }

  set onmessageerror(listener) {
    setHandler(this, 'messageerror', listener)
  }

  get onmousedown() {
    return getHandler(this, 'mousedown')
  }

  set onmousedown(listener) {
    setHandler(this, 'mousedown', listener)
  }

  get onmouseenter() {
    return getHandler(this, 'mouseenter')
  }

  set onmouseenter(listener) {
    setHandler(this, 'mouseenter', listener)
  }

  get onmouseleave() {
    return getHandler(this, 'mouseleave')
  }

  set onmouseleave(listener) {
    setHandler(this, 'mouseleave', listener)
  }

  get onmousemove() {
    return getHandler(this, 'mousemove')
  }

  set onmousemove(listener) {
    setHandler(this, 'mousemove', listener)
  }

  get onmouseout() {
    return getHandler(this, 'mouseout')
  }

  set onmouseout(listener) {
    setHandler(this, 'mouseout', listener)
  }

  get onmouseover() {
    return getHandler(this, 'mouseover')
  }

  set onmouseover(listener) {
    setHandler(this, 'mouseover', listener)
  }

  get onmouseup() {
    return getHandler(this, 'mouseup')
  }

  set onmouseup(listener) {
    setHandler(this, 'mouseup', listener)
  }

  get onmousewheel() {
    return getHandler(this, 'mousewheel')
  }

  set onmousewheel(listener) {
    setHandler(this, 'mousewheel', listener)
  }

  get onoffline() {
    return getHandler(this, 'offline')
  }

  set onoffline(listener) {
    setHandler(this, 'offline', listener)
  }

  get ononline() {
    return getHandler(this, 'online')
  }

  set ononline(listener) {
    setHandler(this, 'online', listener)
  }

  get onorientationchange() {
    return getHandler(this, 'orientationchange')
  }

  set onorientationchange(listener) {
    setHandler(this, 'orientationchange', listener)
  }

  get onpagehide() {
    return getHandler(this, 'pagehide')
  }

  set onpagehide(listener) {
    setHandler(this, 'pagehide', listener)
  }

  get onpageshow() {
    return getHandler(this, 'pageshow')
  }

  set onpageshow(listener) {
    setHandler(this, 'pageshow', listener)
  }

  get onpaste() {
    return getHandler(this, 'paste')
  }

  set onpaste(listener) {
    setHandler(this, 'paste', listener)
  }

  get onpause() {
    return getHandler(this, 'pause')
  }

  set onpause(listener) {
    setHandler(this, 'pause', listener)
  }

  get onplay() {
    return getHandler(this, 'play')
  }

  set onplay(listener) {
    setHandler(this, 'play', listener)
  }

  get onplaying() {
    return getHandler(this, 'playing')
  }

  set onplaying(listener) {
    setHandler(this, 'playing', listener)
  }

  get onpointercancel() {
    return getHandler(this, 'pointercancel')
  }

  set onpointercancel(listener) {
    setHandler(this, 'pointercancel', listener)
  }

  get onpointerdown() {
    return getHandler(this, 'pointerdown')
  }

  set onpointerdown(listener) {
    setHandler(this, 'pointerdown', listener)
  }

  get onpointerenter() {
    return getHandler(this, 'pointerenter')
  }

  set onpointerenter(listener) {
    setHandler(this, 'pointerenter', listener)
  }

  get onpointerleave() {
    return getHandler(this, 'pointerleave')
  }

  set onpointerleave(listener) {
    setHandler(this, 'pointerleave', listener)
  }

  get onpointerlockchange() {
    return getHandler(this, 'pointerlockchange')
  }

  set onpointerlockchange(listener) {
    setHandler(this, 'pointerlockchange', listener)
  }

  get onpointerlockerror() {
    return getHandler(this, 'pointerlockerror')
  }

  set onpointerlockerror(listener) {
    setHandler(this, 'pointerlockerror', listener)
  }

  get onpointermove() {
    return getHandler(this, 'pointermove')
  }

  set onpointermove(listener) {
    setHandler(this, 'pointermove', listener)
  }

  get onpointerout() {
    return getHandler(this, 'pointerout')
  }

  set onpointerout(listener) {
    setHandler(this, 'pointerout', listener)
  }

  get onpointerover() {
    return getHandler(this, 'pointerover')
  }

  set onpointerover(listener) {
    setHandler(this, 'pointerover', listener)
  }

  get onpointerup() {
    return getHandler(this, 'pointerup')
  }

  set onpointerup(listener) {
    setHandler(this, 'pointerup', listener)
  }

  get onpopstate() {
    return getHandler(this, 'popstate')
  }

  set onpopstate(listener) {
    setHandler(this, 'popstate', listener)
  }

  get onprogress() {
    return getHandler(this, 'progress')
  }

  set onprogress(listener) {
    setHandler(this, 'progress', listener)
  }

  get onratechange() {
    return getHandler(this, 'ratechange')
  }

  set onratechange(listener) {
    setHandler(this, 'ratechange', listener)
  }

  get onreadystatechange() {
    return getHandler(this, 'readystatechange')
  }

  set onreadystatechange(listener) {
    setHandler(this, 'readystatechange', listener)
  }

  get onrejectionhandled() {
    return getHandler(this, 'rejectionhandled')
  }

  set onrejectionhandled(listener) {
    setHandler(this, 'rejectionhandled', listener)
  }

  get onreset() {
    return getHandler(this, 'reset')
  }

  set onreset(listener) {
    setHandler(this, 'reset', listener)
  }

  get onresize() {
    return getHandler(this, 'resize')
  }

  set onresize(listener) {
    setHandler(this, 'resize', listener)
  }

  get onscroll() {
    return getHandler(this, 'scroll')
  }

  set onscroll(listener) {
    setHandler(this, 'scroll', listener)
  }

  get onsecuritypolicyviolation() {
    return getHandler(this, 'securitypolicyviolation')
  }

  set onsecuritypolicyviolation(listener) {
    setHandler(this, 'securitypolicyviolation', listener)
  }

  get onseeked() {
    return getHandler(this, 'seeked')
  }

  set onseeked(listener) {
    setHandler(this, 'seeked', listener)
  }

  get onseeking() {
    return getHandler(this, 'seeking')
  }

  set onseeking(listener) {
    setHandler(this, 'seeking', listener)
  }

  get onselect() {
    return getHandler(this, 'select')
  }

  set onselect(listener) {
    setHandler(this, 'select', listener)
  }

  get onselectionchange() {
    return getHandler(this, 'selectionchange')
  }

  set onselectionchange(listener) {
    setHandler(this, 'selectionchange', listener)
  }

  get onselectstart() {
    return getHandler(this, 'selectstart')
  }

  set onselectstart(listener) {
    setHandler(this, 'selectstart', listener)
  }

  get onstalled() {
    return getHandler(this, 'stalled')
  }

  set onstalled(listener) {
    setHandler(this, 'stalled', listener)
  }

  get onstorage() {
    return getHandler(this, 'storage')
  }

  set onstorage(listener) {
    setHandler(this, 'storage', listener)
  }

  get onsubmit() {
    return getHandler(this, 'submit')
  }

  set onsubmit(listener) {
    setHandler(this, 'submit', listener)
  }

  get onsuspend() {
    return getHandler(this, 'suspend')
  }

  set onsuspend(listener) {
    setHandler(this, 'suspend', listener)
  }

  get ontimeupdate() {
    return getHandler(this, 'timeupdate')
  }

  set ontimeupdate(listener) {
    setHandler(this, 'timeupdate', listener)
  }

  get ontoggle() {
    return getHandler(this, 'toggle')
  }

  set ontoggle(listener) {
    setHandler(this, 'toggle', listener)
  }

  get ontouchcancel() {
    return getHandler(this, 'touchcancel')
  }

  set ontouchcancel(listener) {
    setHandler(this, 'touchcancel', listener)
  }

  get ontouchend() {
    return getHandler(this, 'touchend')
  }

  set ontouchend(listener) {
    setHandler(this, 'touchend', listener)
  }

  get ontouchmove() {
    return getHandler(this, 'touchmove')
  }

  set ontouchmove(listener) {
    setHandler(this, 'touchmove', listener)
  }

  get ontouchstart() {
    return getHandler(this, 'touchstart')
  }

  set ontouchstart(listener) {
    setHandler(this, 'touchstart', listener)
  }

  get ontransitioncancel() {
    return getHandler(this, 'transitioncancel')
  }

  set ontransitioncancel(listener) {
    setHandler(this, 'transitioncancel', listener)
  }

  get ontransitionend() {
    return getHandler(this, 'transitionend')
  }

  set ontransitionend(listener) {
    setHandler(this, 'transitionend', listener)
  }

  get ontransitionrun() {
    return getHandler(this, 'transitionrun')
  }

  set ontransitionrun(listener) {
    setHandler(this, 'transitionrun', listener)
  }

  get ontransitionstart() {
    return getHandler(this, 'transitionstart')
  }

  set ontransitionstart(listener) {
    setHandler(this, 'transitionstart', listener)
  }

  get onunhandledrejection() {
    return getHandler(this, 'unhandledrejection')
  }

  set onunhandledrejection(listener) {
    setHandler(this, 'unhandledrejection', listener)
  }

  get onunload() {
    return getHandler(this, 'unload')
  }

  set onunload(listener) {
    setHandler(this, 'unload', listener)
  }

  get onvisibilitychange() {
    return getHandler(this, 'visibilitychange')
  }

  set onvisibilitychange(listener) {
    setHandler(this, 'visibilitychange', listener)
  }

  get onvolumechange() {
    return getHandler(this, 'volumechange')
  }

  set onvolumechange(listener) {
    setHandler(this, 'volumechange', listener)
  }

  get onvrdisplayactivate() {
    return getHandler(this, 'vrdisplayactivate')
  }

  set onvrdisplayactivate(listener) {
    setHandler(this, 'vrdisplayactivate', listener)
  }

  get onvrdisplayblur() {
    return getHandler(this, 'vrdisplayblur')
  }

  set onvrdisplayblur(listener) {
    setHandler(this, 'vrdisplayblur', listener)
  }

  get onvrdisplayconnect() {
    return getHandler(this, 'vrdisplayconnect')
  }

  set onvrdisplayconnect(listener) {
    setHandler(this, 'vrdisplayconnect', listener)
  }

  get onvrdisplaydeactivate() {
    return getHandler(this, 'vrdisplaydeactivate')
  }

  set onvrdisplaydeactivate(listener) {
    setHandler(this, 'vrdisplaydeactivate', listener)
  }

  get onvrdisplaydisconnect() {
    return getHandler(this, 'vrdisplaydisconnect')
  }

  set onvrdisplaydisconnect(listener) {
    setHandler(this, 'vrdisplaydisconnect', listener)
  }

  get onvrdisplayfocus() {
    return getHandler(this, 'vrdisplayfocus')
  }

  set onvrdisplayfocus(listener) {
    setHandler(this, 'vrdisplayfocus', listener)
  }

  get onvrdisplaypointerrestricted() {
    return getHandler(this, 'vrdisplaypointerrestricted')
  }

  set onvrdisplaypointerrestricted(listener) {
    setHandler(this, 'vrdisplaypointerrestricted', listener)
  }

  get onvrdisplaypointerunrestricted() {
    return getHandler(this, 'vrdisplaypointerunrestricted')
  }

  set onvrdisplaypointerunrestricted(listener) {
    setHandler(this, 'vrdisplaypointerunrestricted', listener)
  }

  get onvrdisplaypresentchange() {
    return getHandler(this, 'vrdisplaypresentchange')
  }

  set onvrdisplaypresentchange(listener) {
    setHandler(this, 'vrdisplaypresentchange', listener)
  }

  get onwaiting() {
    return getHandler(this, 'waiting')
  }

  set onwaiting(listener) {
    setHandler(this, 'waiting', listener)
  }

  get onwheel() {
    return getHandler(this, 'wheel')
  }

  set onwheel(listener) {
    setHandler(this, 'wheel', listener)
  }

  get onzoom() {
    return getHandler(this, 'zoom')
  }

  set onzoom(listener) {
    setHandler(this, 'zoom', listener)
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
}

const getHandler = (et, kind) => et[INLINE_HANDLERS][kind] ?? null

const setHandler = (et, kind, handler) => {
  if (!handler) {
    return et.removeEventListener(kind, handlerProxy)
  }

  if (!et[INLINE_HANDLERS][kind]) {
    et.addEventListener(kind, handlerProxy)
  }
  
  et[INLINE_HANDLERS][kind] = handler
}

function handlerProxy(e) {
  // @ts-expect-error
  (this[INLINE_HANDLERS] ?? {})[e.type].call(this, e)
}

export { EventTargetWithHandlerProps as EventTarget }
