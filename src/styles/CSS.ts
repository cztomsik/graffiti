import { parseColor, pascalCase } from '../core/utils'

export class StylePropertyMapReadOnly {
  _data = new Map()

  get(property: string): CSSStyleValue {
    return this._data.get(property)
  }

  get size() {
    return this._data.size
  }
}

export class StylePropertyMap extends StylePropertyMapReadOnly {
  // TODO:
  //   map.set('padding', CSSStyleValue.parse('padding', '5px'))
  //   map.size === 4
  set(property: string, value: CSSStyleValue) {
    console.log('set', property, value)
    this._data.set(property, value)
  }

  delete(property: string) {
    console.log('delete', property)
    this._data.delete(property)
  }

  clear() {
    console.log('clear')
    this._data.clear()
  }
}

export class CSSStyleValue {
  static parse(_property: string, value: string): CSSStyleValue {
    if (value[0] === '#') {
      return raw(parseColor(value))
    }

    // TODO: image url

    if (value.match(/^[a-z-]+$/)) {
      return raw(pascalCase(value))
    }

    return CSSUnitValue.parse(value)
  }
}

// TODO: CSSNumericValue, CSSKeywordValue, ...
export class CSSUnitValue extends CSSStyleValue {
  constructor(public unit, public value) {
    super()
  }

  static parse(value: string) {
    console.log(value)
    const [, numStr, unit] = value.match(/^([\d.]+)([a-z%]+)$/)

    return new CSSUnitValue(unit, parseFloat(numStr))
  }
}

export const CSS = {
  px: unit('px'),
  percent: unit('percent')
}

function unit(unit: string) {
  return (n: number) => CSSUnitValue.parse(`n${unit}`)
}

function raw(v) {
  return Object.assign(new CSSStyleValue(), { raw: v })
}
