import { Node } from './Node'

export class Text extends Node {
  _data

  // we can't do inline layout (yet)
  // so we want to at least join adjacent text nodes
  //
  // - group can be shared
  // - group always contains at least this text
  // - only first one is used for rendering
  // - others are always empty/cleared
  _group: Text[] = [this]

  constructor(doc, data, _nativeId) {
    super(doc, Node.TEXT_NODE, _nativeId)
    this.data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentElement) {
      updateText(this)
    }
  }

  set textContent(v) {
    this.data = v
  }

  set nodeValue(v) {
    this.data = v
  }
}

export const updateText = (text: Text) => {
  const { fontSize, lineHeight } = text.parentElement.style._textStyle

  text.ownerDocument._scene.setText(text._group[0]._nativeId, fontSize, lineHeight, 0, text._group.map(t => t._data).join(''))
}

export const joinTexts = (left: Text, right: Text) => {
  clearText(right)

  left._group.push(...right._group)
  right._group.forEach(t => t._group = left._group)

  updateText(left)
}

export const splitTexts = (left: Text, right: Text) => {
  const newGroup = left._group.splice(left._group.indexOf(right))

  newGroup.forEach(t => t._group = newGroup)

  updateText(left)
  updateText(right)
}

export const removeText = (text: Text) => {
  const group = text._group

  if (group.length === 1) {
    // clear/reset
    return clearText(text)
  } else if (group[0] === text) {
    // swap ids with next node (so we dont need to clear)
    const id = text._nativeId
    text._nativeId = group[1]._nativeId
    group[1]._nativeId = id
  }

  // remove & get own group back
  text._group = group.splice(group.indexOf(text), 1)

  updateText(group[0])
}

const clearText = (text: Text) => text.ownerDocument._scene.setText(text._nativeId, 0, 0, 0, '')
