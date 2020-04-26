// TODO: share with CSSStyleDeclaration
const EM = 16

// mostly inspired by css reboot
export const defaultStyles = {
  BODY: {
    display: 'block',
    width: '100%',
    minHeight: '100%',
  },

  DIV: {
    display: 'block',
  },

  H1: {
    display: 'block',
    fontSize: 2.5 * EM,
    lineHeight: 1.2 * 2.5 * EM,
    marginBottom: 0.5 * EM,
  },

  H2: {
    display: 'block',
    fontSize: 2 * EM,
    lineHeight: 1.2 * 2 * EM,
    marginBottom: 0.5 * EM,
  },

  H3: {
    display: 'block',
    fontSize: 1.75 * EM,
    lineHeight: 1.2 * 1.75 * EM,
    marginBottom: 0.5 * EM,
  },

  H4: {
    display: 'block',
    fontSize: 1.5 * EM,
    lineHeight: 1.2 * 1.5 * EM,
    marginBottom: 0.5 * EM,
  },

  H5: {
    display: 'block',
    fontSize: 1.25 * EM,
    lineHeight: 1.2 * 1.25 * EM,
    marginBottom: 0.5 * EM,
  },

  H6: {
    display: 'block',
    fontSize: 1 * EM,
    lineHeight: 1.2 * EM,
    marginBottom: 0.5 * EM,
  },

  BUTTON: {
    backgroundColor: '#2196F3',
    paddingLeft: 10,
    paddingRight: 10,
    borderRadius: 2,
    fontSize: 14,
    lineHeight: 32,
    color: '#ffffff',
    textAlign: 'center',
    justifyContent: 'space-around',
  },

  A: {
    color: '#4338ad',
  },

  P: {
    marginBottom: 1 * EM,
  },

  INPUT: {
    lineHeight: 1 * EM,
    padding: 0.5 * EM,
    minHeight: 2 * EM,
  },

  TEXTAREA: {
    display: 'block'
  },

  TABLE: {
    display: 'flex',
    flexDirection: 'column'
  },

  THEAD: {
    display: 'flex',
    flexDirection: 'column'
  },

  TBODY: {
    display: 'flex',
    flexDirection: 'column'
  },

  TR: {
    display: 'flex',
    width: '100%'
  },

  TH: {
    flex: 1,
    color: '#666'
  },

  TD: {
    flex: 1
  }
}
