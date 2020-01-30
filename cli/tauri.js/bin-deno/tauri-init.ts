// import parseArgs from 'https://cdn.jsdelivr.net/npm/minimist@1.2.0/index.js'
import { createRequire } from 'https://deno.land/std/node/module.ts'
const require_ = createRequire(import.meta.url)
const parseArgs = require_('minimist')
/**
 * @type {object}
 * @property {boolean} h
 * @property {boolean} help
 * @property {string|boolean} f
 * @property {string|boolean} force
 * @property {boolean} l
 * @property {boolean} log
 * @property {boolean} d
 * @property {boolean} directory
 */
const argv = parseArgs(Deno.args, {
  alias: {
    h: 'help',
    f: 'force',
    l: 'log',
    d: 'directory',
    t: 'tauri-path'
  },
  boolean: ['h', 'l']
})

if (argv.help) {
  console.log(`
  Description
    Inits the Tauri template. If Tauri cannot find the tauri.conf.json
    it will create one.
  Usage
    $ tauri init
  Options
    --help, -h        Displays this message
    --force, -f       Force init to overwrite [conf|template|all]
    --log, -l         Logging [boolean]
    --directory, -d   Set target directory for init
    --tauri-path, -t   Path of the Tauri project to use (relative to the cwd)
    `)
  Deno.exit(0)
}

const init = require_('../dist/api/init.js')

init({
  directory: argv.d || Deno.cwd(),
  force: argv.f || null,
  logging: argv.l || null,
  tauriPath: argv.t || null
})
