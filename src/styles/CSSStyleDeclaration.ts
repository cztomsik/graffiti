import { camelCase, pascalCase, kebabCase, parseColor, UNSUPPORTED } from '../core/utils'
import { SceneContext } from '../core/SceneContext'
import { Display, Dimension, Text, TextAlign, Transform } from '../core/nativeApi'
import { Node } from '../dom/Node'
import { updateText } from '../dom/Text'

// minimal impl just to get something working
export class CSSStyleDeclaration {
  _textStyle: any = { fontSize: 16, lineHeight: 20 }

  constructor(private _scene: SceneContext, private _elementId) {
    installProxy(this)
  }

  // kebab-case
  setProperty(k, v) {
    switch (k) {
      // shorthands first

      case 'flex':
        this.setProperty('flex-grow', v)
        this.setProperty('flex-shrink', v)
        // should be just 0 but chrome does percents too
        this.setProperty('flex-basis', v ? '0%' : 'auto')
        break

      case 'padding':
        this.setProperty('padding-top', v)
        this.setProperty('padding-right', v)
        this.setProperty('padding-bottom', v)
        this.setProperty('padding-left', v)
        break

      case 'padding-vertical':
        this.setProperty('padding-top', v)
        this.setProperty('padding-bottom', v)
        break

      case 'padding-horizontal':
        this.setProperty('padding-right', v)
        this.setProperty('padding-left', v)
        break

      case 'margin':
        this.setProperty('margin-top', v)
        this.setProperty('margin-right', v)
        this.setProperty('margin-bottom', v)
        this.setProperty('margin-left', v)
        break

      case 'margin-vertical':
        this.setProperty('margin-top', v)
        this.setProperty('margin-bottom', v)
        break

      case 'margin-horizontal':
        this.setProperty('margin-right', v)
        this.setProperty('margin-left', v)
        break

      case 'color':
        this._scene.setColor(this._elementId, parseColor(v || '#000'))
        break
      case 'font-size':
        this._textStyle = { ...this._textStyle, fontSize: parseFloat(v) }
        this._updateTexts()
        break
      case 'line-height':
        this._textStyle = { ...this._textStyle, lineHeight: parseFloat(v) }
        this._updateTexts()
        break

      // props
      // TODO: defaults, but be careful

      case 'transform':
        this._scene.setStyle(this._elementId, 'Transform', (v || undefined) && parseTransform(v))
        break

      case 'display':
        this._scene.setStyle(this._elementId, 'Display', Display[pascalCase(v || 'block')])
        break

      case 'width':
      case 'height':
      case 'min-width':
      case 'min-height':
      case 'max-width':
      case 'max-height':
      case 'flex-basis':
      case 'top':
      case 'right':
      case 'bottom':
      case 'left':
      case 'padding-top':
      case 'padding-right':
      case 'padding-bottom':
      case 'padding-left':
      case 'margin-top':
      case 'margin-right':
      case 'margin-bottom':
      case 'margin-left':
        this._scene.setDimension(this._elementId, pascalCase(k), parseDimension(v || 0))
        break
      case 'flex-grow':
      case 'flex-shrink':
        this._scene.setStyle(this._elementId, pascalCase(k), +v || 0)
        break;

      case 'align-content':
      case 'align-items':
      case 'align-self':
      case 'justify-content':
        this._scene.setAlign(this._elementId, pascalCase(k), pascalCase(v || 'FlexStart'))
        break

      case 'background-color':
        this._scene.setBackgroundColor(this._elementId, parseColor(v))
        break

      case 'flex-direction':
        this._scene.setFlexDirection(this._elementId, pascalCase(v))
        break
      case 'flex-wrap':
        this._scene.setFlexWrap(this._elementId, pascalCase(v))
        break
      /*
      case 'overflow':
        break

      case 'background-image':
        // + gradient
      case 'box-shadow':
      case 'border-*':
      case 'border-radius':
        break
      */

      // set cssText() wouldn't work with current proxy design
      // (it would get caught)
      case 'css-text':
        // TODO: mithrill does style.cssText = '' to reset
        if (v !== '') {
          UNSUPPORTED()
        }
        break

      default:
        console.log(`TODO: style.${k} ${v}`)
    }
  }

  _updateTexts() {
    // update first only (text joining)
    let first = true

    for (const c of (document as any)._getEl(this._elementId).childNodes) {
      if (c.nodeType === Node.TEXT_NODE) {
        if (first) {
          updateText(c)
          first = false
        }
      } else {
        first = true
      }
    }
  }
}

function parseDimension(value?: string | number) {
  value = '' + value

  if (value.endsWith('%')) {
    return Dimension.Percent(parseFloat(value))
  }

  if (value === undefined) {
    return Dimension.Undefined()
  }

  if (value === 'auto') {
    return Dimension.Auto()
  }

  return Dimension.Px(parseFloat(value))
}

function parseTransform(v) {
  let match

  if (match = v.match(/scale\(([\d\.\s]+)(?:,([\d\.\s]+))?\)/)) {
    const [, x, y = x] = match
    return Transform.Scale(parseFloat(x), parseFloat(y))
  }

  return undefined
}

/*

  This wouldnt work:

    Object.setPrototypeOf(
      CSSStyleDeclaration.prototype,
      new Proxy({} as any, {
        set: (_o, k, v, style) => (style.setProperty(kebabCase(k), v), true)
      })
    )

  TODO: maybe it's better to just define setters during init
  (either strings or enumerating StyleChange variants)

  that way, there would be just one prototype and it should have less overhead

*/
function installProxy(style: any) {
  style.__proto__ = new Proxy(style.__proto__, {
    set: (o, k, v) => (style.setProperty(kebabCase(k), v), true)
  })
}
