import * as React from 'react'
import { useState, useEffect } from 'react'
import {
  View,
  Button,
  Text,
  FlatList,
  StyleSheet,
  ActivityIndicator
} from '../../src/react'
import * as http from 'http'

export function HackerNews() {
  const [page, setPage] = useState(1)
  const [submissions, setSubmissions] = useState(null as [])
  const [selection, setSelection] = useState(null)

  useEffect(() => {
    setSubmissions(null)

    getJSON(`http://node-hnapi.herokuapp.com/news?page=${page}`).then(items => {
      setSubmissions(items)
      setSelection(items[0])
    })
  }, [page])

  return (
    <View style={{ flex: 1, flexDirection: 'row' }}>
      {(submissions && selection) ? (
        <>
          <View style={{ flex: 1 }}>
            <FlatList
              data={submissions}
              renderItem={({ item, index }) => (
                <SubmissionListItem
                  key={index}
                  submission={item}
                  active={selection === item}
                  onClick={() => setSelection(item)}
                />
              )}
            />

            <Button title="Next page" onPress={() => setPage(page + 1)} />
          </View>

          <View style={{ flex: 2, marginLeft: 10 }}>
            <SubmissionDetail submission={selection} />
          </View>
        </>
      ) : (
        <ActivityIndicator />
      )}
    </View>
  )
}

const SubmissionListItem = ({
  submission: { title, time_ago, comments_count },
  active,
  onClick
}) => (
  <View style={[styles.item, active && styles.itemActive]} onClick={onClick}>
    <Text>{title}</Text>
    <Text>{time_ago}</Text>
    <Text>{comments_count} comments</Text>
  </View>
)

const SubmissionDetail = ({ submission: { title } }) => (
  <View>
    <Text>{title}</Text>

    <Text>TODO: load & show detail</Text>
  </View>
)

const styles = StyleSheet.create({
  item: {
    padding: 10
  },

  itemActive: {
    backgroundColor: '#eee'
  }
})

const getJSON = url => {
  return new Promise(resolve => {
    http.get(url, res => {
      res.setEncoding('utf-8')

      let data = ''
      res.on('data', chunk => (data += chunk))
      res.on('end', () => resolve(JSON.parse(data)))
    })
  }) as Promise<any>
}
