import { HTMLElement } from './HTMLElement'

export class HTMLButtonElement extends HTMLElement implements globalThis.HTMLButtonElement {
  disabled = false
  checkValidity
  form
  formAction
  formEnctype
  formMethod
  formNoValidate
  formTarget
  labels
  name
  reportValidity
  setCustomValidity
  type
  validationMessage
  validity
  value
  willValidate
}
