// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so

// const { default: init } = require('./pkg/teracoja.js')

// that no one else needs to worry about it again.
import('./pkg').then((module) => {
  module.run_app()
})
