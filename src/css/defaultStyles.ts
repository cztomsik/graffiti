// TODO: share with CSSStyleDeclaration
const EM = 16

// mostly inspired by css reboot
export const defaultStyles = `
  /*
    default styles
  */

  body {
    display: block;
    width: 100%;
    min-height: 100%;
  }

  div {
    display: block;
  }

  h1 {
    display: block;
    font-size: ${2.5 * EM}px;
    line-height: ${1.2 * 2.5 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  h2 {
    display: block;
    font-size: ${2 * EM}px;
    line-height: ${1.2 * 2 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  h3 {
    display: block;
    font-size: ${1.75 * EM}px;
    line-height: ${1.2 * 1.75 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  h4 {
    display: block;
    font-size: ${1.5 * EM}px;
    line-height: ${1.2 * 1.5 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  h5 {
    display: block;
    font-size: ${1.25 * EM}px;
    line-height: ${1.2 * 1.25 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  h6 {
    display: block;
    font-size: ${EM}px;
    line-height: ${1.2 * EM}px;
    margin-bottom: ${0.5 * EM}px;
  }

  button {
    background-color: #2196F3;
    padding-left: 10px;
    padding-right: 10px;
    border-radius: 2px;
    font-size: 14px;
    line-height: 32px;
    color: #ffffff;
    text-align: center;
    justify-content: space-around;
  }

  a {
    color: #4338ad;
  }

  p {
    margin-bottom: ${EM}px;
  }

  input {
    line-height: ${EM}px;
    padding: ${0.5 * EM}px;
    min-height: ${2 * EM}px;
  }

  textarea {
    display: block;
  }

  table {
    display: flex;
    flex-direction: column;
  }

  thead {
    display: flex;
    flex-direction: column;
  }

  tbody {
    display: flex;
    flex-direction: column;
  }

  tr {
    display: flex;
    width: 100%;
  }

  th {
    flex: 1;
    color: #666;
  }

  td {
    flex: 1;
  }
`
