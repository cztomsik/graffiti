import * as React from 'react'
import { render } from 'react-dom'
import { readdir, readFile } from 'fs'
import { basename } from 'path'
import { spawn } from 'child_process'

const App = () => {
  const [files, setFiles] = React.useState(null)
  const [selection, select] = React.useState(THIS_FILE)
  const [source, setSource] = React.useState('')

  React.useEffect(() => {
    readdir(__dirname, { withFileTypes: true }, (err, files) => setFiles(files.filter(f => f.isFile())))
  }, [])

  React.useEffect(() => {
    readFile(`${__dirname}/${selection}`, (err, source) => setSource('' + source))
  }, [selection])

  return (
    files && (
      <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', padding: 20 }}>
          <h3>{selection}</h3>

          <button onClick={() => runExample(selection)}>Run example</button>
        </div>

        <div style={{ flex: 1, display: 'flex' }}>
          <div style={{ flex: 1, backgroundColor: '#fff' }}>
            {files &&
              files.map(({ name }) => (
                <ExampleItem key={name} text={name} active={name === selection} onClick={() => select(name)} />
              ))}
          </div>

          <div style={{ flex: 2, padding: 20, backgroundColor: '#223' }}>
            <div style={{ color: '#ffc', fontSize: 14, lineHeight: 20 }}>{source || 'Loading...'}</div>
          </div>
        </div>
      </div>
    )
  )
}

const ExampleItem: any = ({ text, active, ...rest }) => {
  const [hovered, setHovered] = React.useState(false)

  return (
    <div
      style={{ padding: 20, backgroundColor: active ? '#223' : hovered ? '#ccc' : undefined }}
      onMouseOver={() => setHovered(true)}
      onMouseOut={() => setHovered(false)}
      {...rest}
    >
      <span style={{ color: active ? '#fff' : '#000' }}>{text}</span>
    </div>
  )
}

const runExample = example => {
  spawn('npm', ['run', 'example', example], {
    shell: true
  })
}

const THIS_FILE = basename(__filename)

render(<App />, document.body)
