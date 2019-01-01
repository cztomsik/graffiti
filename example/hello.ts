import { Window, ResourceManager, TextContainer, TextPart, Surface } from '../src'

const w = new Window("Hello")

const text = new TextContainer()
text.update({ color: [1, 1, 1, 1], lineHeight: 30 })
text.appendChild(new TextPart("Hello"))

const brush = ResourceManager.getBrush({
  backgroundColor: '#ff0000'
})

const layout = ResourceManager.getLayout({
  flex: 1,
  padding: 20
})

const rect = new Surface()
rect.update({ brush, layout })
rect.appendChild(text)

w.appendChild(rect)
w.render()
