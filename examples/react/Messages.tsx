import * as React from 'react'
import { useState } from 'react'
import {
  View,
  Button,
  Text,
  FlatList,
  StyleSheet,
  TextInput
} from '../../src/react'

const messages = [
  { text: 'Hello' },
  { text: 'World' },
  { text: '...' },
  { text: 'lorem' },
  { text: 'ipsum' },
  { text: 'lorem' },
  { text: 'ipsum' }
]

export function Messages() {
  const [text, setText] = useState('')

  return (
    <View style={{ flex: 1, width: 400 }}>
      <FlatList
        data={messages}
        renderItem={({ item, index }) => (
          <MessageItem key={index} item={item} me={index % 2 === 0} />
        )}
        contentContainerStyle={{ alignItems: 'flex-start' }}
      />

      <View style={{ flexDirection: 'row', marginTop: 10 }}>
        <TextInput style={styles.input} value={text} onChangeText={setText} />
        <Button title="Send" onPress={() => setText('')} />
      </View>
    </View>
  )
}

function MessageItem({ item, me }) {
  return (
    <View style={[styles.msg, me ? styles.msgMe : styles.msgYou]}>
      <Text style={[styles.text, me && styles.textMe]}>{item.text}</Text>
    </View>
  )
}

const styles = StyleSheet.create({
  msg: {
    borderRadius: 4,
    backgroundColor: '#ccc',
    padding: 5,
    paddingHorizontal: 12,
    marginVertical: 4
  },

  msgMe: {
    alignSelf: 'flex-end',
    backgroundColor: '#00f',
    borderTopLeftRadius: 14,
    borderBottomLeftRadius: 14
  },

  msgYou: {
    borderTopRightRadius: 14,
    borderBottomRightRadius: 14
  },

  text: {
    textAlign: 'right',
    lineHeight: 24
  },

  textMe: {
    color: '#fff',
    textAlign: 'left'
  },

  input: {
    flex: 1,
    marginRight: 10
  }
})
