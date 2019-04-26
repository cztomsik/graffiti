import * as React from 'react'
import { useState } from 'react'
import { readFileSync } from 'fs'
import { View, Text, FlatList, ScrollView } from '../../src/react'
import { Hello } from './Hello'
import { Counter } from './Counter'
import { TemperatureConverter } from './TemperatureConverter';
import { Calculator } from './Calculator'
import { Hover } from './Hover'
import { Bench } from './Bench'
import { Messages } from './Messages'
import { ImageExample } from './ImageExample'

const examples = [Hello, Counter, TemperatureConverter, Calculator, Hover, Bench, Messages, ImageExample].map(Comp => ({
  name: Comp.name,
  Comp,
  source: readFileSync(`${__dirname}/${Comp.name}.tsx`, 'utf-8')
}))

export function App() {
  const [activeIndex, setActive] = useState(2)
  const example = examples[activeIndex]

  return (
    <View style={{ flex: 1, flexDirection: 'row', alignContent: 'stretch' }}>
      <FlatList
        data={examples}
        renderItem={({ item, index }) => (
          <ExampleItem
            key={item.name}
            name={item.name}
            active={index === activeIndex}
            onClick={() => setActive(index)}
          />
        )}
        style={{
          flex: 0,
          width: 250,
          padding: 20,
          borderRightWidth: 1,
          borderRightColor: '#cccccc'
        }}
      />

      <View style={{ flex: 1, justifyContent: 'space-between' }}>
        <View style={{ flex: 1, padding: 20 }}>
          <example.Comp />
        </View>

        <ScrollView key={example.name} style={{ flex: 1.25, padding: 20, backgroundColor: '#222233' }}>
          <Text style={{ color: '#ffffcc', lineHeight: 27 }}>{example.source}</Text>
        </ScrollView>
      </View>
    </View>
  )
}

function ExampleItem({ name, active, onClick }) {
  return (
    <View
      style={{
        paddingHorizontal: 20,
        paddingVertical: 5,
        borderRadius: 3,
        backgroundColor: active && '#eeeeee'
      }}
      onClick={onClick}
    >
      <Text>{name}</Text>
    </View>
  )
}
