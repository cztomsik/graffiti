class TextPart {
  value
  textContainer?

  constructor(value) {
    this.setValue(value)
  }

  setValue(value) {
    this.value = value

    if (this.textContainer !== undefined) {
      this.textContainer.updateContent()
    }
  }
}

export default TextPart
