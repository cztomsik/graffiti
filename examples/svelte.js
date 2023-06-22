import 'graffiti'
import { compile } from 'https://unpkg.com/svelte@3.59.2/compiler.mjs'

const { js } = compile(`
  <script>
    let count = 0;

    function handleClick() {
      count += 1;
    }
  </script>

  <button on:click={handleClick}>
    Clicked {count} {count === 1 ? 'time' : 'times'}
  </button>
`)

const { default: App } = await import(
  'data:text/javascript;base64,' +
    btoa(js.code.replace('svelte/internal', 'https://unpkg.com/svelte@3.59.2/internal/index.mjs'))
)

new App({ target: document.body })
