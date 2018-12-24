import * as React from 'react'
import { useState } from 'react'
import { Window } from '../src'
import { render, View, Text, Button, StyleSheet, FlatList } from '../src/react'

const Row = (props: { name: string }) => (
  <View style={styles.row}>
    <Text style={styles.rowText}>{props.name}</Text>
  </View>
)

const Separator = () => <View style={styles.separator} />

const genStr = (() => {
  let id = 1
  return () => `row ${id++}`
})()

const initialRows = Array.from(new Array(4), genStr)
const App = () => {
  const [rows, changeRows] = useState(initialRows)

  return (
    <View style={{ flex: 1 }}>
      <Button
        title="Add row"
        onPress={() => changeRows(rows.concat(genStr()))}
      />

      <FlatList
        data={rows}
        ItemSeparatorComponent={Separator}
        renderItem={({ item }) => <Row name={item} />}
      />
    </View>
  )
}

const styles = StyleSheet.create({
  row: {
    height: 50,
    margin: 10,
    padding: 10,
    backgroundColor: '#444466'

    // height: 100,
    // padding: 10,
    // justifyContent: "flex-end",
    // backgroundColor: "#000000"
  },

  rowText: {
    fontSize: 20,
    color: '#ffffff'
  },

  separator: {
    backgroundColor: '#444466',
    // borderBottomColor: '#bbbbbb',
    // width: '100%',
    // borderBottomWidth: 1
    height: 1
  }
})

render(<App />, new Window('FlatList', 400, 600))
