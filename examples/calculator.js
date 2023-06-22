import 'graffiti'

document.documentElement.innerHTML = `
  <head>
    <style>
      html {
        background: #000;
        color: #fff;
      }
      
      body {
        background: #446;
        max-width: 400px;
      }

      .display {
        display: flex;
        align-items: center;
        height: 70px;
        padding: 0 10px;
        background: #668;
        font-size: 20px;
      }

      .buttons {
        display: flex;
        flex-wrap: wrap;
        height: 400px;
        width: 400px;
      }

      .buttons > button {
        background: #2196F3;
        flex: 1 1 20%;
        margin: 3px;
      }
    </style>
  </head>

  <body>
    <div class="display">0</div>

    <div class="buttons">
      ${'789*456-123+0,/='
        .split('')
        .map(c => `<button>${c}</button>`)
        .join('')}
    </div>
  </body>
`

const display = document.querySelector('.display')

for (const btn of document.querySelectorAll('.buttons button')) {
  btn.addEventListener('click', () => {
    display.textContent += btn.textContent
  })
}
