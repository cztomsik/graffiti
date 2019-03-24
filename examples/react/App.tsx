import * as React from 'react'
import { readFileSync } from 'fs'
import { View, Text, FlatList } from '../../src/react'
import { Hello } from './Hello'
import { Counter } from './Counter'
import { Calculator } from './Calculator'

const examples = [Hello, Counter, Calculator].map(Comp => ({
  name: Comp.name,
  Comp,
  source: readFileSync(`${__dirname}/${Comp.name}.tsx`, 'utf-8')
}))

export function App() {
  // TODO: selection
  const activeIndex = 1
  const example = examples[activeIndex]

  return (
    <View style={{ flex: 1, flexDirection: 'row', alignContent: 'stretch' }}>
      <FlatList
        data={examples}
        renderItem={({ item, index }) => <ExampleItem key={item.name} active={index === activeIndex} {...item} />}
        style={{ flex: 0, width: 240, borderRightWidth: 1, borderStyle: 'solid', borderRightColor: '#cccccc' }}
      />

      <View style={{ flex: 1, justifyContent: 'space-between' }}>
        <View style={{ padding: 20 }}>
          <example.Comp />
        </View>

        <View style={{ padding: 20, backgroundColor: '#222233' }}>
          <Text style={{ color: '#ffffcc' }}>{example.source}</Text>
        </View>
      </View>

    </View>
  )
}

function ExampleItem({ name, active }) {
  return <View style={{ paddingHorizontal: 20, paddingVertical: 5, backgroundColor: active && '#eeeeee' }}>
    <Text>{name}</Text>
  </View>
}
