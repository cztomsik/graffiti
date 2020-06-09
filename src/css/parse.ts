import assert from 'assert'

// TODO: not complete but it's a good start

export const parseRules = (str: string): RuleData[] => {
  // we don't need many state flags because strings & comments can be
  // consumed in one step
  let rules: RuleData[] = []
  let rule, prop
  let buf = ''

  for (let ch, i = 0; (ch = str[i]); i++) {
    switch (ch) {
      // skip comments
      // (can be before/after prop but also in the middle of composed value)
      case '/':
        i = str.indexOf('*/', i + 2) + 1
        assert(i !== 0, 'unclosed comment')
        continue

      // strings
      // TODO: escaping (do-while?)
      case '"':
      case "'":
        const end = str.indexOf(ch, i + 1)
        assert(~end, 'unclosed string')

        buf = str.slice(i, end + 1)
        i = end
        continue

      case '{':
        rule = { selector: buf.trim(), props: {} }
        assert(rule.selector, 'empty selector')
        buf = ''
        continue

      case ':':
        prop = buf.trim()
        buf = ''
        continue

      case ';':
      case '}':
        if (prop) {
          rule.props[prop] = buf.trim()
          prop = ''
          buf = ''
        }

        if (ch === '}') {
          rules.push(rule)
          rule = null
        }

        continue

      default:
        buf += ch
    }
  }

  return rules
}

interface RuleData {
  selector: string,
  props: {
    [prop: string]: string
  }
}
