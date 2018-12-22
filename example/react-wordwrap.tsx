import * as React from 'react'
import { Window } from '..'
import { render, View, Text } from '../src/react'

const NBHY = '\u2011'
const NBSP = '\u00A0'

const App = () =>
  <View style={{ flex: 1, padding: 20 }}>
    <Text>One line</Text>
    <Text>
      Two{'\n'}
      lines
    </Text>
    <Text>
      Long text should wrap to multiple lines
    </Text>
    <Text>
      {'    '}Spaces{'\n'}    should be retained unless    they're    trailing
    </Text>
    <Text>
      Expressions do_not_wrap
      {Math.pow(2, 30)}
    </Text>
    <Text>
      Non{NBHY}breaking
      hyphens{NBSP}and{NBSP}spaces
      do{NBSP}not{NBSP}wrap{NBSP}either
    </Text>
  </View>

render(<App />, new Window("Hello", 300, 500))
