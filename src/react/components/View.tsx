// native (host) comp
// left here because people might be looking for View.tsx
//
// View is very common so it makes sense to handle it directly in the reconciler
// rather than destructuring/spreading and allocating more objects
// (there are 22 on* props and most of them would have been a prop-miss)
//
// BTW: react-specific, angular & vue have separate listeners & props
export default 'View'
