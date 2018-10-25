import './init'
import { Component, Vue } from 'vue-property-decorator'

@Component({
  template: `
    <renderer>
      <rect :w="100" :h="100" :x="x" :y="y" />
    </renderer>
  `
})
class Example extends Vue {
  count = 0

  created() {
    console.log('example created')
    setInterval(() => this.count += 0.1, 100)
  }

  get x() {
    return 100 + Math.sin(this.count) * 50
  }

  get y() {
    return 100 + Math.cos(this.count) * 50
  }
}

new Example().$mount(window)
