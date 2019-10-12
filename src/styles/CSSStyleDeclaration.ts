import { camelCase, pascalCase, kebabCase, parseColor } from '../core/utils'
import { SceneContext } from '../core/SceneContext'

// minimal impl just to get something working
export class CSSStyleDeclaration {
  // temp state for text changes
  text = undefined

  constructor(private _scene: SceneContext, private _surfaceId) {
    installProxy(this)
  }

  // kebab-case
  setProperty(k, v) {
    switch (k) {
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
        this._scene.setTextColor(this._surfaceId, parseColor(v || '#000'))
        break
      case 'font-size':
      case 'line-height':
        v = parseFloat(v)
      case 'text-align':
      // TODO: this was bad idea
      case 'content':
        this.text = { ...this.text, [camelCase(k)]: v }

        this._scene.setText(this._surfaceId, (this.text.content || undefined) && {
          font_size: this.text.fontSize || 16,
          line_height: this.text.lineHeight || this.text.fontSize || 16,
          align: pascalCase(this.text.align || 'left'),
          text: this.text.content
        })
        break

      case 'width':
      case 'height':
      case 'min-width':
      case 'min-height':
      case 'max-width':
      case 'max-height':
      case 'flex-basis':
        v = v || 'auto'
      case 'flex-grow':
      case 'flex-shrink':
      case 'padding-top':
      case 'padding-right':
      case 'padding-bottom':
      case 'padding-left':
      case 'margin-top':
      case 'margin-right':
      case 'margin-bottom':
      case 'margin-left':
        this._scene.setDimension(this._surfaceId, pascalCase(k), parseDimension(v || 0))
        break

      case 'align-content':
      case 'align-items':
      case 'align-self':
      case 'justify-content':
        this._scene.setAlign(this._surfaceId, pascalCase(k), pascalCase(v || 'FlexStart'))
        break

      case 'background-color':
        this._scene.setBackgroundColor(this._surfaceId, parseColor(v))
        break

      case 'flex-direction':
        this._scene.setFlexDirection(this._surfaceId, pascalCase(v))
        break
      case 'flex-wrap':
        this._scene.setFlexWrap(this._surfaceId, pascalCase(v))
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

      default:
        console.log(`TODO: style.${k} ${v}`)
    }
  }
}

function parseDimension(value?: string | number) {
  value = '' + value

  if (value.endsWith('%')) {
    return { percent: parseFloat(value) }
  }

  if (value === 'auto' || value === undefined) {
    return {}
  }

  return { point: parseFloat(value) }
}

function installProxy(style: any) {
  style.__proto__ = new Proxy(style.__proto__, {
    set: (o, k, v) => (style.setProperty(kebabCase(k), v), true)
  })
}
