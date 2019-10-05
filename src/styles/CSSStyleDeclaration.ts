import { camelCase, pascalCase } from '../core/utils'
import { SceneContext } from '../core/SceneContext'

// minimal impl just to get something working
export class CSSStyleDeclaration {
  constructor(private _scene: SceneContext, private _surfaceId) {}

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

      case 'content':
        this._scene.setText(this._surfaceId, { size: 16, line_height: 16, align: 'Left', text: v })
        break

      case 'width':
      case 'height':
      case 'min-width':
      case 'min-height':
      case 'max-width':
      case 'max-height':
      case 'flex-grow':
      case 'flex-shrink':
      case 'flex-basis':
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
